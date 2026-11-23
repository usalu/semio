import os, sys, collections

R = r"C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END\🗑️generated\rescue-conflicts"


def parse(txt):
    out = []
    cur = None
    state = None
    for l in txt.split("\n"):
        if l.startswith("<<<<<<< "):
            cur = {"new": [], "base": [], "old": []}
            state = "new"
            continue
        if cur is not None:
            if l.startswith("||||||| "):
                state = "base"
                continue
            if l.startswith("======="):
                state = "old"
                continue
            if l.startswith(">>>>>>> "):
                out.append(cur)
                cur = None
                continue
            cur[state].append(l)
    return out


def load():
    res = {}
    for line in open(R + r"\index.tsv", encoding="utf-8").read().split("\n"):
        if not line:
            continue
        i, d, p = line.split("\t")
        f = "merged.lf" if i in ("02", "08") else "merged"
        txt = open(os.path.join(d, f), encoding="utf-8", errors="replace", newline="").read().replace("\r\n", "\n")
        res[i] = (d, p, parse(txt))
    return res


def kind(h):
    n = "\n".join(h["new"])
    o = "\n".join(h["old"])
    if n.replace("_din_ref(", "reference_chip(") == o:
        return "R"
    if n.strip() == o.strip():
        return "ws"
    return "other"


if __name__ == "__main__":
    data = load()
    for i in sys.argv[1].split(","):
        d, p, hs = data[i]
        k = 0
        for h in hs:
            if kind(h) != "other":
                continue
            print(f"=== file {i} hunk {k}")
            k += 1
            for s in ("new", "base", "old"):
                print(f"[{s}]")
                print("\n".join(x[:210] for x in h[s][:int(sys.argv[2]) if len(sys.argv) > 2 else 14]))
