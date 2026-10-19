import json,sys
F=sys.argv[1]
d=json.load(open(F))
print("PROJECT",d.get("name"),"namedInputs:",list((d.get("namedInputs") or {}).keys()))
for n,t in d["targets"].items():
    o=t.get("options",{}) or {}
    cmd=o.get("command") or json.dumps(o.get("commands"),ensure_ascii=False)
    cmd=(cmd or "").replace("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","<L>/")
    extra={k:v for k,v in t.items() if k in("cache","inputs","outputs","dependsOn","continuous","executor")}
    if o.get("cwd"): extra["cwd"]=o["cwd"]
    print(f"{n} :: {cmd[:130]} :: {json.dumps(extra,ensure_ascii=False)[:300]}")
