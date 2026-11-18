import json,re,sys
s=open(sys.argv[1]).read()
n=int(sys.argv[2]) if len(sys.argv)>2 else 14
for part in re.split(r'(?<=\n)(?=\{\n "label")',s):
    try: d=json.loads(part)
    except Exception: continue
    ev=[e for e in d["events"] if not e.startswith(("mouse","pointer")) or "proc 0" not in e]
    print("==",d["label"],"long",d["long"],ev[:6]); print("\n".join("  "+x.replace("%F0%9F%8C%90%EF%B8%8F-","") for x in d["inclusive"][:n]))
