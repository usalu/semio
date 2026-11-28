#!/usr/bin/env python3
"""H53: the owner pool never stops its children for a wait they did not cause.
Usage: pool-yield.py stage <outdir>   (writes new texts, changes nothing in the tree)
       pool-yield.py land            (atomic replace of the tree files, checks anchors and shas first)"""
import hashlib, json, os, subprocess, sys, tempfile
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/"
TS = ROOT + "🟦️.ts"
FIX = ROOT + "🧪️tests/🧫️fixtures/🔣️.json"
ORA = ROOT + "🧪️tests/🧫️fixtures/🐍️oracle.py"
OLD_CORES = "const coresHeld=idle===null||idle<1&&idle+input.children>=1,memoryHeld=foreground.waitingMegabytes===undefined||blocked(foreground.waitingMegabytes);"
NEW_CORES = "const coresHeld=(idle===null||idle<1&&idle+input.children>=1)&&(foreground.waitingMegabytes===undefined||foreground.waitingMegabytes<=roomNow),memoryHeld=foreground.waitingMegabytes===undefined||blocked(foreground.waitingMegabytes),feeds=(foreground.retainedUnrealizedMegabytes??0)>0;"
OLD_YIELD = 'if(foreground.waiters>0&&waited>=policy.yieldAfterMilliseconds&&(memoryHeld||coresHeld))return'
NEW_YIELD = 'if(foreground.waiters>0&&waited>=policy.yieldAfterMilliseconds&&!feeds&&(memoryHeld||coresHeld))return'
OLD_DOC = "a wait the children did not cause is no reason to stop them, and there is no time after which they come back);"
NEW_DOC = "a wait the children did not cause is no reason to stop them: a request that does not fit the room even now is not held by the cores they take, and while a retained reservation of the machine's own read is unrealized the children are what feeds that read, so stopping them never frees what the waiters wait for, and there is no time after which they come back);"
OLD_PY = 'cores_held = idle is None or (idle < 1 and idle + case["children"] >= 1)\n    memory_held = waiting is None or blocked(waiting)\n    if fg["waiters"] > 0 and waited >= p["yieldAfterMilliseconds"] and (memory_held or cores_held):'
NEW_PY = 'cores_held = (idle is None or (idle < 1 and idle + case["children"] >= 1)) and (waiting is None or waiting <= room_now)\n    memory_held = waiting is None or blocked(waiting)\n    feeds = fg.get("retainedUnrealizedMegabytes", 0) > 0\n    if fg["waiters"] > 0 and waited >= p["yieldAfterMilliseconds"] and not feeds and (memory_held or cores_held):'
def host(**fg):
    base = {"cores": 12, "totalMegabytes": 15848, "availableMegabytes": 9513, "pressure": "normal", "load": 20.9, "idleCores": 0, "swapOutMegabytesPerSecond": 0,
            "foreground": {"cpu": 3, "unrealizedMegabytes": 0, "waiters": 6, "waitingMegabytes": 19248, "oldestWaitMilliseconds": 2331000}}
    base["foreground"].update(fg)
    return base
CASES = [
 {"name": "H53 a retained graph reservation of the own read keeps the children alive while the machine's own work waits behind it (g1-m04 07:16)", "host": host(retainedUnrealizedMegabytes=6414), "children": 1, "childMegabytes": 800},
 {"name": "H53 a request that does not fit the room even now is not held by the cores the children take", "host": host(unrealizedMegabytes=6414), "children": 1, "childMegabytes": 800},
 {"name": "H53 a request that fits the room and still waits is held by the cores: the children yield", "host": host(waitingMegabytes=1000), "children": 1, "childMegabytes": 800},
]
def read(path): return open(path, encoding="utf8").read()
def sha(text): return hashlib.sha256(text.encode("utf8")).hexdigest()[:12]
def build():
    ts, fix, ora = read(TS), read(FIX), read(ORA)
    for text, anchor in ((ts, OLD_CORES), (ts, OLD_YIELD), (ts, OLD_DOC), (ora, OLD_PY)):
        if text.count(anchor) != 1: sys.exit("anchor missing or not unique: " + anchor[:60])
    ts = ts.replace(OLD_CORES, NEW_CORES).replace(OLD_YIELD, NEW_YIELD).replace(OLD_DOC, NEW_DOC)
    ora = ora.replace(OLD_PY, NEW_PY)
    doc = json.loads(fix)
    doc["target"] += CASES
    with tempfile.TemporaryDirectory() as folder:
        for name, text in (("o.py", ora), ("f.json", json.dumps(doc, indent=1, ensure_ascii=False))):
            open(os.path.join(folder, name), "w", encoding="utf8").write(text)
        out = json.loads(subprocess.run(["python3", "-I", os.path.join(folder, "o.py"), os.path.join(folder, "f.json")], capture_output=True, text=True, check=True).stdout)
    old = len(doc["expect"]["target"])
    doc["expect"]["target"] += out["target"][old:]
    doc["expect"]["limit"] += out["limit"][old:]
    return ts, ora, json.dumps(doc, indent=1, ensure_ascii=False), out["target"][old:]
def atomic(path, text):
    temporary = path + ".h53.tmp"
    open(temporary, "w", encoding="utf8").write(text)
    os.replace(temporary, path)
if __name__ == "__main__":
    ts, ora, fix, added = build()
    if sys.argv[1] == "stage":
        folder = sys.argv[2]; os.makedirs(folder, exist_ok=True)
        for name, text in (("owner-pool.ts", ts), ("oracle.py", ora), ("fixtures.json", fix)): open(os.path.join(folder, name), "w", encoding="utf8").write(text)
        print("staged", sha(ts), sha(ora), sha(fix), json.dumps(added))
    else:
        for path, text in ((ROOT + "🟦️.ts", ts), (ORA, ora), (FIX, fix)): atomic(path, text)
        print("landed", json.dumps(added))
