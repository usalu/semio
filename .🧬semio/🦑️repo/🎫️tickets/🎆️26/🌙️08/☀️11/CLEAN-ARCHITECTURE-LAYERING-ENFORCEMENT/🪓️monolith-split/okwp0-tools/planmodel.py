# 🧭️ Parses the crate tables of PLAN-OS-KERNEL-CRATES.md (6.1, 6.2, Appendix D) into a JSON model.
import json, re, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
PLAN = HERE + "/../PLAN-OS-KERNEL-CRATES.md"
text = open(PLAN, encoding="utf-8").read()
def section(start, end):
    i = text.index(start); j = text.index(end, i + 1) if end else len(text)
    return text[i:j]
s61 = section("### 6.1 Catalogue", "### 6.2 Per-crate specifications")
crates = {}
for line in s61.split("\n"):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) == 9 and cells[0].isdigit():
        name = cells[1].strip("`").split(" ")[0]
        deps = re.findall(r"`([^`]+)`", cells[7])
        crates[name] = dict(n=int(cells[0]), wp=int(cells[2]), lib_lines=int(cells[3]), lib_kb=int(cells[4]), lib_files=int(cells[5]), tests=cells[6], deps=deps, cfg=cells[8])
s62 = section("### 6.2 Per-crate specifications", "### 6.3 Crate-level imports")
for m in re.finditer(r"#### (\d+)\. `([a-z0-9-]+)[^`]*` \(WP (\d); (\d+) lines, (\d+) KB, (\d+) files\)\n\n(.*?)(?=\n#### |\Z)", s62, re.S):
    name = m.group(2); body = m.group(7)
    if name not in crates: print("62 unknown", name, file=sys.stderr); continue
    ext = re.search(r"\* external crates named in the code: (.*)", body)
    ids = re.findall(r"`([A-Za-z0-9_]+)`", ext.group(1)) if ext and not ext.group(1).startswith("none") else []
    node = re.search(r"\* node: `([^`]*)`", body)
    crates[name]["external_idents"] = ids
    crates[name]["node"] = node.group(1) if node else ""
sd = section("## Appendix D. Skeleton table", "## Appendix E.")
nodes = {}
for line in sd.split("\n"):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) == 5 and cells[0].isdigit():
        name = cells[1].strip("`").split(" ")[0].strip("`")
        nodes[name] = dict(n=int(cells[0]), node=cells[2].replace("`", "").replace(" (new)", ""), new="(new)" in cells[2], project=cells[3].strip("`"), ups=int(cells[4]))
json.dump(dict(lib=crates, nodes=nodes), open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
print(len(crates), "6.1 rows;", len(nodes), "Appendix D rows;", sum(1 for c in crates.values() if "external_idents" in c), "with 6.2 externals")
