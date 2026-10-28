"""🇩🇪 Builder of lane germanqa: re-reads the CURRENT files of a root and returns {rel: (base_bytes, new_bytes)} for every file whose German text the rules change.
usage: python3 -I build.py <root> <out.json>   (out.json: {"changes": {rel: {"base": sha, "new": sha}}, "files": ...} is written as a directory listing by stage_all.py)"""
import hashlib, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gqlib, rules

SKIP = set()
TRANSLIT_ONLY = {"🧰️framework/🛍️products/🦑️repo/README.md"}
TRANSLIT_TOKENS = [k for k in rules.TRANSLIT if len(k) >= 5]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def edit_json(raw, rel):
    task = rules.task_applies(rel)
    try: doc = json.loads(raw)
    except Exception: return raw, 0
    leaves = {}
    for path, en, de in gqlib.pairs_of(doc):
        if de is None or en == de: continue
        new = rules.transform(de, path.endswith("/title"), task)
        if new != de: leaves.setdefault(de, new)
    out, count = raw, 0
    for old, new in sorted(leaves.items(), key=lambda kv: -len(kv[0])):
        a, b = json.dumps(old, ensure_ascii=False), json.dumps(new, ensure_ascii=False)
        n = out.count(a)
        if n == 0: raise SystemExit(f"{rel}: cannot locate {a[:90]}")
        out = out.replace(a, b); count += n
    for path, text in gqlib.language_leaves(json.loads(out)):
        new = rules.transform(text, False, task) if gqlib.german_like(text, TRANSLIT_TOKENS) else text
        if new != text and text not in leaves:
            a, b = json.dumps(text, ensure_ascii=False), json.dumps(new, ensure_ascii=False)
            if out.count(a) == 0: raise SystemExit(f"{rel}: cannot locate {a[:90]}")
            out = out.replace(a, b); count += 1
    slash = {}

    def swalk(v):
        if isinstance(v, dict):
            for x in v.values(): swalk(x)
        elif isinstance(v, list):
            for x in v: swalk(x)
        elif isinstance(v, str) and " / " in v and not re.search(r"(?:^|\s)DE: ", v):
            parts = gqlib.slash_split(v, TRANSLIT_TOKENS)
            if parts:
                new = rules.transform(parts[1], False, task)
                if new != parts[1]: slash[v] = parts[0] + " / " + new

    swalk(json.loads(out))
    for old, new in slash.items():
        a, b = json.dumps(old, ensure_ascii=False), json.dumps(new, ensure_ascii=False)
        if out.count(a) == 0: raise SystemExit(f"{rel}: cannot locate slash text {a[:90]}")
        out = out.replace(a, b); count += 1
    desc = {}

    def walk(v):
        if isinstance(v, dict):
            for x in v.values(): walk(x)
        elif isinstance(v, list):
            for x in v: walk(x)
        elif isinstance(v, str) and re.search(r"(?:^|\s)DE: ", v) and re.search(r"(?:^|\s)EN: ", v):
            segs = gqlib.split_markers(v)
            rebuilt = "".join((f"{k}: " if k else "") + (rules.transform(t, False, task) if k == "DE" else t) for k, t in segs)
            if rebuilt != v: desc[v] = rebuilt

    walk(json.loads(out))
    for old, new in desc.items():
        a, b = json.dumps(old, ensure_ascii=False), json.dumps(new, ensure_ascii=False)
        if out.count(a) == 0: raise SystemExit(f"{rel}: cannot locate description {a[:90]}")
        out = out.replace(a, b); count += 1
    return out, count


INLINE = re.compile(r"(`[^`\n]*`)")


def edit_markdown(text, task=True, translit_only=False):
    lines = text.split("\n")
    start = next((i for i, l in enumerate(lines) if gqlib.DE_HEADING.match(l)), None)
    if start is None: return text, 0
    fenced, count = False, 0
    for i in range(start, len(lines)):
        line = lines[i]
        if line.lstrip().startswith("```"): fenced = not fenced; continue
        if fenced: continue
        parts = INLINE.split(line)
        new = [p if j % 2 else (rules.fix_translit(p) if translit_only else rules.transform(p, False, task)) for j, p in enumerate(parts)]
        joined = "".join(new)
        if joined != line: lines[i] = joined; count += 1
    return "\n".join(lines), count


def edit_code(text, task=True):
    spans = gqlib.code_literals(text, TRANSLIT_TOKENS)
    out, last, count = [], 0, 0
    for a, b, s in spans:
        new = rules.transform(s, False, task)
        out.append(text[last:a]); out.append(new); last = b
        if new != s: count += 1
    out.append(text[last:])
    return "".join(out), count


def build(root):
    changes = {}
    for rel in gqlib.files_of(root, (".json", ".md") + gqlib.CODE_EXT):
        if rel in SKIP: continue
        path = os.path.join(root, rel)
        raw = open(path, "rb").read()
        try: text = raw.decode("utf-8")
        except UnicodeDecodeError: continue
        if rel.endswith(".json"): new, n = edit_json(text, rel)
        elif rel.endswith(".md"): new, n = edit_markdown(text, rules.task_applies(rel), rel in TRANSLIT_ONLY)
        else: new, n = edit_code(text, rules.task_applies(rel))
        if new != text: changes[rel] = (raw, new.encode("utf-8"), n)
    return changes


if __name__ == "__main__":
    root = os.path.abspath(sys.argv[1])
    ch = build(root)
    kinds = {}
    for rel, (b, n, c) in ch.items(): kinds[os.path.splitext(rel)[1]] = kinds.get(os.path.splitext(rel)[1], 0) + 1
    print(len(ch), "files changed", kinds)
    if len(sys.argv) > 2:
        os.makedirs(sys.argv[2], exist_ok=True)
        json.dump({rel: {"base": sha(b), "new": sha(n), "edits": c} for rel, (b, n, c) in ch.items()}, open(os.path.join(sys.argv[2], "summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
