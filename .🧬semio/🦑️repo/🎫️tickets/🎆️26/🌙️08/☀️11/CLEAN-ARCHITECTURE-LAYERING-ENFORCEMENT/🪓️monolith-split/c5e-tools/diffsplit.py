import re, sys, os
D = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff"
leaf = D + "/🟦️.ts"
child_dir = D + "/🧾️program-diff"
text = open(leaf, encoding="utf8").read()
lines = text.split("\n")
assert lines[0].startswith("/** 🧬️ ProgramSnapshot diff schema") and lines[1] == ""
assert lines[2].startswith("import { architectProgramArtifactGuardExactObject")
imp = lines[3:7]
assert all(l.startswith("import type {") for l in imp) and lines[7] == ""
start = lines.index("export interface ProgramDiff {")
end = start
while lines[end] != "}": end += 1
assert lines[end + 1] == "" and lines[end + 2].startswith("export const PROGRAM_DIFF_FIELDS")
iface = lines[start:end + 1]
rest = lines[end + 2:]
rest_text = "\n".join(rest)
iface_text = "\n".join(iface)
parsed = []
for l in imp:
    m = re.match(r'^import type \{ (.*) \} from "(.*)";$', l)
    parsed.append(([n.strip() for n in m.group(1).split(",")], m.group(2)))
def used(n, t): return re.search(r"\b" + re.escape(n) + r"\b", t) is not None
leaf_imports, child_imports = [], []
for names, spec in parsed:
    keep = [n for n in names if used(n, rest_text)]
    move = [n for n in names if used(n, iface_text)]
    assert not (set(keep) & set(move)) and set(keep) | set(move) == set(names), (spec, set(names) - set(keep) - set(move))
    if keep: leaf_imports.append(f'import type {{ {", ".join(keep)} }} from "{spec}";')
    if move: child_imports.append(f'import type {{ {", ".join(move)} }} from ".{spec}";')
new_leaf = [lines[0], "", lines[2], 'import type { ProgramDiff } from "./🧾️program-diff/🟦️.ts";'] + leaf_imports + ["", "export type { ProgramDiff };", ""] + rest
child = ["/** 🧾️ ProgramSnapshot diff schema: the root sparse diff, one optional delta or edit per program register and per singleton. */"] + child_imports + [""] + iface + [""]
lc = len(new_leaf); print("leaf lines", lc, "child lines", len(child))
if len(sys.argv) > 1 and sys.argv[1] == "apply":
    os.makedirs(child_dir, exist_ok=True)
    open(child_dir + "/🟦️.ts", "w", encoding="utf8").write("\n".join(child))
    open(leaf, "w", encoding="utf8").write("\n".join(new_leaf))
    print("applied")
