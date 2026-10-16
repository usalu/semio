# 🧭️ Parses the crate tables of PLAN-RENDERER-WGPU-CRATES.md into a JSON model (crates, WP, deps, external identifiers, cfg).
import json, re, os, sys
PLAN = os.path.dirname(os.path.abspath(__file__)) + "/../PLAN-RENDERER-WGPU-CRATES.md"
text = open(PLAN, encoding="utf-8").read()
def section(start, end):
    i = text.index(start); j = text.index(end, i + 1) if end else len(text)
    return text[i:j]
s61 = section("### 6.1 Catalogue", "### 6.2 Per-crate specifications")
crates = {}
for line in s61.split("\n"):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) == 9 and cells[0].isdigit():
        name = cells[1].strip("`")
        deps = [d.strip().strip("`") for d in re.split(r",\s*(?![^`]*`\s*,)", cells[7]) if d.strip()] if cells[7] else []
        deps = re.findall(r"`([^`]+)`", cells[7])
        shell = "shell-*" in cells[7]
        crates[name] = dict(n=int(cells[0]), wp=int(cells[2]), lib_lines=int(cells[3]), lib_kb=int(cells[4]), lib_files=int(cells[5]), tests=cells[6], deps=[d for d in deps if d != "shell-*"], shell_dep=shell, cfg=cells[8], kind="lib")
print(len(crates), "6.1 rows", file=sys.stderr)
s62 = section("### 6.2 Per-crate specifications", "### 6.3 Crate-level imports")
for m in re.finditer(r"#### `([a-z0-9-]+)` \(WP(\d), (\d+) lines, (\d+) KB\)\n\n(.*?)(?=\n#### |\Z)", s62, re.S):
    name = m.group(1); body = m.group(5)
    if name not in crates: print("62 unknown", name, file=sys.stderr); continue
    ext = re.search(r"\* external crates named in the code: (.*)", body)
    ids = re.findall(r"`([A-Za-z0-9_]+)`", ext.group(1)) if ext and "none" not in ext.group(1)[:6] else []
    members = re.search(r"\* members \(source file[^\n]*\): (.*)", body)
    crates[name]["external_idents"] = ids
    crates[name]["members"] = members.group(1) if members else ""
# shell crates 7.7
s77 = section("### 7.7 Shell crates, sizes and order", "### 7.8 Other large impls")
shell = {}
for line in s77.split("\n"):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) == 8 and cells[0].isdigit():
        name = cells[1].strip("`")
        shell[name] = dict(n=int(cells[0]), tier=cells[2], fields=int(cells[3]), est_lines=int(cells[6]), wp=int(cells[7]), kind="shell")
print(len(shell), "7.7 rows", file=sys.stderr)
# test bins appendix D
sd = section("## Appendix D. Shell test bins", "## Appendix E.")
bins = {}
for line in sd.split("\n"):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) == 6 and cells[0].startswith("`shell-tests-"):
        bins[cells[0].strip("`")] = dict(tests=[t.strip() for t in cells[1].split(",")], lines=int(cells[2]), kb=int(cells[3]), files=int(cells[4]), wp=int(cells[5]), kind="shell-tests")
print(len(bins), "App D bins", file=sys.stderr)
json.dump(dict(lib=crates, shell=shell, bins=bins), open(sys.argv[1] if len(sys.argv) > 1 else "/dev/stdout", "w"), ensure_ascii=False, indent=1)
