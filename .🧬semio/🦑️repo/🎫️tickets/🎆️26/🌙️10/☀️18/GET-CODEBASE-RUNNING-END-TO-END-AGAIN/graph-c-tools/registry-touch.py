"""touch|restore: append one newline to the committed owner registry (harmless: same parsed JSON) and later remove exactly that byte again, only when the file is still byte-identical to what touch left."""
import hashlib, json, os, sys
root = "/Users/ueli/Documents/semio"
path = os.path.join(root, "📋️build-owners.json")
state = os.path.join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/graph-c/registry-touch.json")
sha = lambda b: hashlib.sha256(b).hexdigest()
data = open(path, "rb").read()
if sys.argv[1] == "touch":
    assert not data.endswith(b"\n\n"), "already touched"
    new = data + b"\n"
    open(path, "wb").write(new)
    json.dump({"before": sha(data), "after": sha(new), "size": len(data)}, open(state, "w"))
    print("touched", sha(data)[:12], "->", sha(new)[:12])
else:
    st = json.load(open(state))
    if sha(data) == st["after"]:
        open(path, "wb").write(data[:-1]); print("restored", sha(data[:-1])[:12], "expected", st["before"][:12])
    else:
        print("NOT restored: file changed since touch (a peer rewrote it)", sha(data)[:12])
