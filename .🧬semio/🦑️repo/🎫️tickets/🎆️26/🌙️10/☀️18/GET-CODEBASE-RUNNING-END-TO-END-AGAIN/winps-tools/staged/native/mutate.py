#!/usr/bin/env python3
"""🧬 Negative controls of batch `native` in a MIRROR root: python3 -I mutate.py <root> <id> applies one deliberate defect to the new modules (restore with `stage.py apply` after swapping, or by `restore`). ids: parent (reads the id where the parent id sits), leak (never closes a process handle), floor (keeps the 100 ns digit), recycled (no recycled-id check), command (drops the image name), size (no structure size before the first call)."""
import os, sys
root, name = sys.argv[1], sys.argv[2]
A = "🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🧭️ancestry"
M = {
 "parent": (A + "/🪟️native/🟦️.ts", "parent:entry.readUInt32LE(record.parentId)", "parent:entry.readUInt32LE(record.processId)"),
 "leak": (A + "/🪟️native/🟦️.ts", "}finally{binding.close(handle);}", "}finally{}"),
 "floor": (A + "/🪟️native/🟦️.ts", "fraction-fraction%time.microsecondTicks", "fraction"),
 "recycled": (A + "/🟦️.ts", "if(child!==undefined&&detail.started>child)break;", ""),
 "command": (A + "/🟦️.ts", 'command:(detail.commandLine+" "+row.name).trim()', "command:detail.commandLine.trim()"),
 "size": (A + "/🪟️native/🟦️.ts", "entry.writeUInt32LE(record.bytes,0);", ""),
}
rel, old, new = M[name]
path = os.path.join(root, rel)
if os.path.exists(os.path.join(root, ".git")): sys.exit("refusing the live checkout")
text = open(path, encoding="utf-8").read()
if text.count(old) != 1: sys.exit("anchor %d× for %s" % (text.count(old), name))
open(path, "w", encoding="utf-8").write(text.replace(old, new))
