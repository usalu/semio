SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🟦.ts")


def split_top(text):
    parts, depth, start = [], 0, 0
    for i, ch in enumerate(text):
        if ch in "<([{":
            depth += 1
        elif ch in ">)]}" and not (ch == ">" and text[i - 1] == "="):
            depth -= 1
        elif ch == "," and depth == 0:
            parts.append(text[start:i])
            start = i + 1
    parts.append(text[start:])
    return [p.strip() for p in parts if p.strip()]


def members():
    text = open(SOURCE, encoding="utf8").read()
    body = re.search(r"export interface SelfTestLanes \{(.*?)\n\}", text, re.S).group(1)
    out = []
    for m in re.finditer(r"readonly (\w+): \((.*)\) => (.+?);\s*$", body, re.M):
        params = [[p.split(":", 1)[0].strip(), p.split(":", 1)[1].strip()] for p in split_top(m.group(2))]
        out.append({"name": m.group(1), "params": params, "returns": m.group(3).strip()})
    return out


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: members() for c in cases})
