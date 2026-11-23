import os, sys, json, py_compile, subprocess
R = r"C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END\🗑️generated"
bad = 0
for line in open(R + r"\rescue-status-classes.tsv", encoding="utf-8").read().split("\n"):
    if not line or not line.startswith(("c-", "b-")):
        continue
    k, st, p = line.split("\t")
    fn = os.path.join(r"C:\semio", p)
    try:
        if fn.endswith(".py"):
            src = open(fn, encoding="utf-8").read()
            compile(src, fn, "exec")
        elif fn.endswith(".json"):
            json.load(open(fn, encoding="utf-8"))
        if "<<<<<<<" in open(fn, encoding="utf-8", errors="replace").read() and fn.endswith((".py", ".json", ".ts", ".tsx", ".mjs")):
            print("MARKER", p.encode("ascii", "replace")); bad += 1
    except Exception as e:
        bad += 1
        print("FAIL", p.encode("ascii", "replace").decode()[-60:], str(e)[:100])
print("bad", bad)
