import json, re, sys

def digits_mean(before, ranges):
    text = list(before + "\n# total bytes: %d" % (len(before) // 2))
    digits = len(before)
    out, position, touched = [], 0, False
    for r in ranges:
        end = r["offset"] + r["delete"]
        if end > len(text) or r["offset"] < position:
            return None
        if r["offset"] > digits:
            continue
        if end > digits or (r["offset"] == digits and r["delete"] > 0):
            return None
        touched = True
        out.append("".join(text[position:r["offset"]]))
        out.append(r["insert"])
        position = end
    if not touched:
        return "ignored"
    out.append("".join(text[position:digits]))
    joined = re.sub(r"\s+", "", "".join(out))
    if len(joined) % 2 or not re.fullmatch(r"[0-9a-f]*", joined):
        return None
    return bytes.fromhex(joined)

def apply_leaves(before, leaves):
    data = bytes.fromhex(before)
    for leaf in leaves:
        data = data[:leaf["offset"]] + bytes.fromhex(leaf["insert"]) + data[leaf["offset"] + leaf["removeLen"]:]
    return data

def expected(case):
    if case["leaves"] is None:
        return None
    return apply_leaves(case["before"], case["leaves"]).hex()

def main(path, write):
    corpus = json.load(open(path, encoding="utf-8"))
    for case in corpus["cases"]:
        after = expected(case)
        mean = digits_mean(case["before"], case["splices"])
        if after is not None and mean not in ("ignored", None):
            assert mean.hex() == after, case["id"]
        if case["leaves"] is None:
            assert mean is None, case["id"]
        case["after"] = after
    if write:
        json.dump(corpus, open(path, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
        open(path, "a").write("\n")
    print(len(corpus["cases"]), "cases independently verified")

if __name__ == "__main__":
    main(sys.argv[1], "--write" in sys.argv)
