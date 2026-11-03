s=open("humanlib.py",encoding="utf8").read()
i=s.index("def save(a, doc):"); j=s.index("class Rest:")
s=s[:i]+'''def save(a, doc):
    sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️FW")
    from atomic_json import write_json_atomic
    write_json_atomic(f"{ROOT}\\{a}\\🔣️.json", doc)


'''+s[j:]
s=s.replace("import json, math, re, os","import json, math, re, os, sys",1)
open("humanlib.py","w",encoding="utf8").write(s)
