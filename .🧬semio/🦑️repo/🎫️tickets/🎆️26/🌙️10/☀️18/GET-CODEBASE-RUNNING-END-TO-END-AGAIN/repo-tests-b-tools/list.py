import json,os,sys
roots=["🧰️framework/🛍️products/🦑️repo","🧰️framework/🔨️modules/🏃️process"]
for r in roots:
  for dp,dn,fn in os.walk(r):
    dn[:]=[d for d in dn if d not in("node_modules",".cache","🧫️fixtures","target")]
    if "📋️project.json" in fn:
      p=os.path.join(dp,"📋️project.json")
      try: j=json.load(open(p))
      except Exception as e: print("BAD",p,e); continue
      ts=[t for t in (j.get("targets") or {}) if "test" in t or t in("check","typecheck","lint")]
      print(j.get("name"),"|",",".join(ts),"|",p.replace("🧰️framework/","F/"))
