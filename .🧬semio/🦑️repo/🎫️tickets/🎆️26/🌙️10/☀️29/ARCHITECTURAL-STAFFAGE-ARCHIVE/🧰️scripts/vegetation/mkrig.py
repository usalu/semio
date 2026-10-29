import json,subprocess,sys,shlex
# usage: mkrig.py <id> <woodMaterial> : design/<id>-bones.txt (one bone.py argument line each) + design/<id>-anchors.json -> design/<id>-a.json
i=sys.argv[1]; mat=json.load(open(f"design/{i}-meta.json"))["wood"]; lines=[l for l in open(f"design/{i}-bones.txt").read().splitlines() if l.strip()]
out=subprocess.check_output(["python3","bone.py",mat]+[l.strip() for l in lines],text=True)
bones=[json.loads(l.rstrip(",")) for l in out.splitlines()]
anchors=json.load(open(f"design/{i}-anchors.json"))
meta=json.load(open(f"design/{i}-meta.json"))
d={"id":meta["id"],"category":"vegetation","rig":{"kind":"botanical","root":bones[0]["id"],"bones":bones},"anchors":anchors}
open(f"design/{i}-a.json","w").write('{"id":%s,"category":"vegetation",\n"rig":{"kind":"botanical","root":%s,"bones":[\n%s\n]},\n"anchors":[\n%s\n]}\n'%(json.dumps(d["id"]),json.dumps(d["rig"]["root"]),",\n".join(json.dumps(b,separators=(",",":"),ensure_ascii=False) for b in bones),",\n".join(json.dumps(a,separators=(",",":"),ensure_ascii=False) for a in anchors)))
