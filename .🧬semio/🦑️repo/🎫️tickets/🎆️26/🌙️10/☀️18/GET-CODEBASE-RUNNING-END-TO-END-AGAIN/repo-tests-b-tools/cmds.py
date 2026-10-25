import json,os,sys
roots=["🧰️framework/🛍️products/🦑️repo","🧰️framework/🔨️modules/🏃️process"]
for r in roots:
  for dp,dn,fn in os.walk(r):
    dn[:]=[d for d in dn if d not in("node_modules",".cache","🧫️fixtures","target")]
    if "📋️project.json" in fn:
      p=os.path.join(dp,"📋️project.json")
      j=json.load(open(p))
      for t,v in (j.get("targets") or {}).items():
        if "test" in t or t in("check","typecheck","lint"):
          o=v.get("options") or {}
          cmd=o.get("command") or o.get("commands") or v.get("executor")
          print(j["name"],t,"|",str(cmd)[:150])
