import json,os,sys
root="✏️s/🔌️plugins"
rows=[]
for dp,dn,fn in os.walk(root):
    dn[:]=[d for d in dn if d not in("node_modules",".cache")]
    if "📋️project.json" in fn and "🟦️typescript" in dp:
        try: j=json.load(open(os.path.join(dp,"📋️project.json")))
        except Exception as e: continue
        if "test" in j.get("targets",{}):
            plugin=dp.split(os.sep)[2]
            rows.append((plugin,j["name"],dp))
rows.sort()
for r in rows: print("\t".join(r))
