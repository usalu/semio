"""🧾 Rust register-function splitter: cuts the statement run of one `pub fn register(...)` body into group functions in child modules (one file per semantic group), keeping every statement line verbatim and in order.
usage: rsplit.py <spec.json> [APPLY]   spec: { file, function, children: [ { dir, doc, funcs: [ { name, emoji, doc, from, to } ] } ] }   from/to = first and last line of the group in the original file."""
import json, os, sys, re
spec = json.load(open(sys.argv[1], encoding="utf8"))
APPLY = len(sys.argv) > 2 and sys.argv[2] == "APPLY"
path = spec["file"]
text = open(os.environ.get("ORIG_TEXT", path), encoding="utf8").read()
L = text.split("\n")
start = next(i for i, l in enumerate(L) if l.startswith(f"pub fn {spec['function']}("))
end = next(i for i in range(start + 1, len(L)) if L[i] == "}")
errors = []
body = L[start + 1:end]
sig = L[start]
m = re.match(r"pub fn \w+\((.*)\) \{$", sig)
params = m.group(1)
args = ", ".join(p.split(":")[0].strip() for p in params.split(","))
funcs = [(c, f) for c in spec["children"] for f in c["funcs"]]
funcs.sort(key=lambda cf: cf[1]["from"])
covered = set()
ordered = []
prev_end = start + 1
for c, f in funcs:
    a, b = f["from"], f["to"]
    if a <= start + 1 or b >= end + 1: errors.append(f"{f['name']} outside the body")
    for l in range(a, b + 1): 
        if l in covered: errors.append(f"line {l} covered twice")
        covered.add(l)
    ordered.append((c, f, L[a - 1:b]))
keep_after = [l for l in range(start + 2, end + 1) if l not in covered and L[l - 1].strip() != ""]
tail = [(l, L[l - 1]) for l in keep_after]
for l, t in tail:
    if not (t.strip().startswith("mesh::register_mesh") or t.strip() == "registry.finalize();"): errors.append(f"uncovered statement line {l}: {t.strip()[:60]}")
for c, f, lines in ordered:
    if lines and (lines[0].strip() == "" or lines[-1].strip() == ""): errors.append(f"{f['name']} starts or ends with a blank line")
emojis = {}
def claim(e, who):
    k = e.replace("️", "")
    if k in emojis: errors.append(f"emoji {e} repeated ({emojis[k]}, {who})")
    emojis[k] = who
V = "️"
outputs = []
root_dir = os.path.dirname(path)
for c in spec["children"]:
    claim(c["dir"].split(V)[0], "folder " + c["dir"])
    parts = []
    for f in c["funcs"]:
        claim(f["emoji"], f["name"])
        lines = L[f["from"] - 1:f["to"]]
        parts.append(f"/// {f['emoji']} {f['doc']}\npub(super) fn {f['name']}({params}) {{\n" + "\n".join(lines) + "\n}\n")
    doc = f"//! {c['doc']}"
    out = os.path.join(root_dir, c["dir"], "🦀️.rs")
    outputs.append((out, f"{doc}\n\nuse super::*;\n\n" + "\n".join(parts)))
root_lines = L[:start]
mods = []
for c in spec["children"]:
    modname = re.sub(r"[^a-z0-9]+", "_", c["dir"].split(V, 1)[1]).strip("_")
    names = ", ".join(f["name"] for f in c["funcs"])
    names = names if len(c["funcs"]) == 1 else "{" + names + "}"
    mods.append(f"#[path = \"{c['dir']}/🦀️.rs\"]\nmod {modname};\nuse {modname}::{names};\n")
calls = []
tail_lines = {l: t for l, t in tail}
tail_pos = sorted(tail_lines)
events = [(f["from"], f"    {f['name']}({args});") for c, f, _ in ordered] + [(l, t) for l, t in tail_lines.items()]
events.sort()
new_fn = [sig] + [e[1] for e in events] + ["}"]
new_text = "\n".join(root_lines[:start]) 
head = L[:start]
# insert the module declarations after the existing `use super::*;` line of the file
idx = next(i for i, l in enumerate(head) if l.strip() == "use super::*;")
new_all = head[:idx + 1] + [""] + "\n".join(mods).rstrip("\n").split("\n") + head[idx + 1:] + new_fn + L[end + 1:]
new_text = "\n".join(new_all)
# lossless proof: the group bodies concatenated with the kept tail statements equal the original body statements
orig_stmt = [l for l in body if l.strip()]
new_stmt = [l for _, _, ls in ordered for l in ls if l.strip()]
tail_stmt = [t for _, t in tail]
merged = []
tl = list(events)
pos = 0
rebuilt = []
for a, e in events:
    pass
rebuilt = []
for (c, f, lines) in ordered:
    rebuilt.append((f["from"], [l for l in lines if l.strip()]))
for l, t in tail: rebuilt.append((l, [t]))
rebuilt.sort()
flat = [x for _, ls in rebuilt for x in ls]
if flat != orig_stmt: errors.append("LOSSLESS FAIL: rebuilt statements differ from the original body")
print(json.dumps({"errors": errors, "lossless": flat == orig_stmt, "statementLines": len(orig_stmt), "outputs": [(os.path.relpath(o, root_dir), t.count("\n")) for o, t in outputs], "root": new_text.count("\n")}, ensure_ascii=False, indent=1))
if os.environ.get("DUMP"):
    os.makedirs(os.environ["DUMP"], exist_ok=True)
    open(os.path.join(os.environ["DUMP"], "root.rs"), "w", encoding="utf8").write(new_text)
    for o, t in outputs: open(os.path.join(os.environ["DUMP"], os.path.basename(os.path.dirname(o)) + ".rs"), "w", encoding="utf8").write(t)
if APPLY:
    if errors: print("REFUSED"); sys.exit(2)
    if open(path, encoding="utf8").read() != text: print("REFUSED: changed"); sys.exit(5)
    for o, t in outputs:
        if os.path.exists(o): print("REFUSED: exists", o); sys.exit(4)
    for o, t in outputs:
        os.makedirs(os.path.dirname(o), exist_ok=True); open(o, "w", encoding="utf8").write(t)
    open(path, "w", encoding="utf8").write(new_text)
    print("APPLIED")
