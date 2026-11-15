"""📇 Moves the path book, the compact table and the local paths of the replica into their own module, so code that only reads the replica state does not import the daemon."""
import os, sys
root = sys.argv[1]
replica = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica")
main_path, book_dir = os.path.join(replica, "🟦️.ts"), os.path.join(replica, "📇️book")
reader_path = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📥️replica/🟦️.ts")
lines = open(main_path, encoding="utf-8").read().split("\n")
start = next(i for i, l in enumerate(lines) if l.startswith("function widen<T extends"))
end = next(i for i, l in enumerate(lines) if l.startswith("/** 📥️ The store winners that the tree does not hold yet"))
while lines[end - 1] == "":
    end -= 1
block = lines[start:end]
assert block[-1] == "}" and any(l.startswith("export class ReplicaBookV1{") for l in block) and any(l.startswith("export class CompactTableV1{") for l in block)
type_doc = next(i for i, l in enumerate(lines) if l.startswith("/** 🗄️ The local state files below"))
assert lines[type_doc + 1].startswith("export type LocalPathsV1=")
fn_doc = next(i for i, l in enumerate(lines) if l.startswith("/** 🗄️ Resolves the machine-local state files of a worktree. */"))
assert lines[fn_doc + 1].startswith("export function localPathsV1(") and lines[fn_doc + 4] == "}"
entry = next(i for i, l in enumerate(lines) if l.startswith("type StateEntry=FleetManifestEntryV1&Readonly<{mtimeMs:number;seenMs?:number}>;"))
book = ['import {join} from "node:path";', 'import type {FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";', "",
        "/** 🧾️ One entry of the replica state: the manifest entry and the times the tree was seen with it. */",
        "export type ReplicaStateEntryV1=FleetManifestEntryV1&Readonly<{mtimeMs:number;seenMs?:number}>;", "",
        lines[type_doc], lines[type_doc + 1], "", *lines[fn_doc:fn_doc + 5], ""]
body = "\n".join(block).replace("StateEntry", "ReplicaStateEntryV1")
for name in ("function widen<", "const hashText=", "const encoder="):
    assert body.count("\n" + name) + body.startswith(name) == 1, name
    body = ("export " + body) if body.startswith(name) else body.replace("\n" + name, "\nexport " + name)
book.append(body)
keep = [l for i, l in enumerate(lines) if not (start <= i < end or i in (type_doc, type_doc + 1) or fn_doc <= i < fn_doc + 5)]
at = next(i for i, l in enumerate(keep) if l.startswith("type StateEntry="))
keep[at] = "type StateEntry=ReplicaStateEntryV1;"
anchor = next(i for i, l in enumerate(keep) if l.startswith('import schema from "./🧬️schema/🔣️.json"'))
keep[anchor:anchor] = ['import {CompactTableV1,ReplicaBookV1,encoder,hashText,localPathsV1,pageBytes,widen,type LocalPathsV1,type ReplicaStateEntryV1} from "./📇️book/🟦️.ts";',
                       'export {CompactTableV1,ReplicaBookV1,localPathsV1,type LocalPathsV1} from "./📇️book/🟦️.ts";']
reader = open(reader_path, encoding="utf-8").read()
old = 'import {CompactTableV1,ReplicaBookV1,localPathsV1} from "../../../../🤝️shared-worktree/🔁️replica/🟦️.ts";'
assert reader.count(old) == 1
if "--apply" in sys.argv:
    os.makedirs(book_dir, exist_ok=True)
    open(os.path.join(book_dir, "🟦️.ts"), "w", encoding="utf-8").write("\n".join(book) + "\n")
    open(main_path, "w", encoding="utf-8").write("\n".join(keep))
    open(reader_path, "w", encoding="utf-8").write(reader.replace(old, old.replace("🔁️replica/🟦️.ts", "🔁️replica/📇️book/🟦️.ts")))
print("block", start + 1, end, "lines", len(block), "main", len(lines), "->", len(keep), "book", len("\n".join(book).split("\n")))
