"""🏷️ Applies the kind-name convention rename through every layer (dirs, rs/json/ts/graphql/proto/py/feature/semio, taxonomy, oracle, framework index). Usage: python -I rename.py [--apply]"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, os, re, sys
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = NORM + "/🗿️artifacts/"
FW = "/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json"
PLURAL = [("floors", "floor"), ("self-weight-elements", "self-weight-element"), ("roofs", "roof"), ("wind-faces", "wind-face"),
          ("accidental-cases", "accidental-case"), ("bridge-lanes", "bridge-lane"), ("construction-stages", "construction-stage"), ("roof-surfaces", "roof-surface")]
GLOBAL = [("set-case-number", "change-case-number"), ("set-case-text", "change-case-text")]
SCOPES = [(NORM, GLOBAL)]
SCOPES.append((ART + "🪶️en1999", [("add-" + n, "insert-" + n) for n in ("member", "weld-run", "outline-vertex", "placement")]))
SCOPES.append((ART + "🏋️en1991", [(v + "-" + a, v + "-" + b) for v in ("insert", "remove") for a, b in PLURAL]))
SCOPES.append((ART + "🏛️en1992", [("reorder-members", "change-member-order")]))
EXT = (".rs", ".json", ".ts", ".graphql", ".proto", ".py", ".feature", ".semio", ".toml", ".sql")


def pas(k):
    return "".join(p.capitalize() for p in k.split("-"))


def rules(pairs):
    out = []
    for o, n in sorted(pairs, key=lambda p: -len(p[0])):
        po, pn = pas(o), pas(n)
        out += [(re.compile(r"(?<![A-Za-z0-9_-])%s(?![A-Za-z0-9_-])" % re.escape(o)), n),
                (re.compile(r"(?<![A-Za-z0-9_])%s(?![A-Za-z0-9_])" % re.escape(o.replace("-", "_"))), n.replace("-", "_")),
                (re.compile(r"(?<![A-Za-z0-9])%s(?![a-z0-9])" % po), pn),
                (re.compile(r"(?<![A-Za-z0-9])%s(?![a-z0-9])" % (po[0].lower() + po[1:])), pn[0].lower() + pn[1:]),
                (re.compile(r"parse%s(?![a-z0-9])" % po), "parse" + pn),
                (re.compile(r"(?<![A-Za-z0-9_-])%s(?=-dupe)" % re.escape(o)), n)]
    return out


def walk(root):
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in ("node_modules", "target", ".git")]
        yield d, ds, fs


def main(apply):
    total = 0
    for root, pairs in SCOPES:
        rs = rules(pairs)
        files = [os.path.join(d, f) for d, _, fs in walk(root) for f in fs if f.endswith(EXT)]
        if root == NORM:
            files.append(FW)
        for f in files:
            try:
                t = open(f, encoding="utf-8").read()
            except (UnicodeDecodeError, FileNotFoundError):
                continue
            u = t
            for rx, b in rs:
                u = rx.sub(b, u)
            if u != t:
                total += 1
                if apply:
                    open(f, "w", encoding="utf-8").write(u)
        m = dict(pairs)
        for d, ds, fs in os.walk(root, topdown=False):
            for x in ds + fs:
                k = re.sub(r"^[^a-z]+", "", x)
                if k in m:
                    pre = x[: len(x) - len(k)]
                    dst = os.path.join(d, pre + m[k])
                    print("path", os.path.join(d, x)[len("/home/deck/semio/"):], "->", pre + m[k])
                    if apply:
                        os.rename(os.path.join(d, x), dst)
    print("files changed", total)
    scoped(apply)
    taxonomy(apply)


def scoped(apply):
    t = open(FW, encoding="utf-8").read()
    lines = t.splitlines(True)
    for root, pairs in SCOPES[1:]:
        tok = root.rsplit("/", 1)[1][-6:]
        rs = rules(pairs)
        for i, l in enumerate(lines):
            if tok in l:
                for rx, b in rs:
                    l = rx.sub(b, l)
                lines[i] = l
    if apply:
        open(FW, "w", encoding="utf-8").write("".join(lines))


def taxonomy(apply):
    p = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    d = json.load(open(p, encoding="utf-8"))
    for root, pairs in SCOPES[1:]:
        tok = root.rsplit("/", 1)[1][-6:]
        rs = rules(pairs)
        for r in d["rows"]:
            if r["artifact"].endswith(tok):
                for k, v in list(r.items()):
                    for rx, b in rs:
                        v = rx.sub(b, v) if isinstance(v, str) else v
                    r[k] = v
    for r in d["rows"]:
        if "en199" in r["artifact"]:
            verb = r["kind"].split("-")[0]
            r["verb"] = verb
            if verb in ("insert", "remove"):
                r["entity"] = r["kind"][len(verb) + 1:]
                if re.match(r"(Added|Inserted|Removed)", r["record"]):
                    r["record"] = ("Inserted" if verb == "insert" else "Removed") + pas(r["entity"])
            elif r["record"].startswith("Reordered"):
                r["record"] = r["type"]
    if apply:
        open(p, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


main("--apply" in sys.argv)
