import json,glob,re,sys
T=sys.path[0]+"/.."
G=T+"/🗑️generated/authoring-humans-a/"
slugs="child-on-kick-scooter courier-with-hand-truck cyclist elderly-with-cane jogger kneeling-gardener leaning-person parent-carrying-toddler parent-with-stroller person-cooking person-on-crutches person-with-rollator person-with-umbrella person-with-white-cane".split()
rows=[]
for s in slugs:
    c=json.load(open(G+s+".check.json"))
    a=json.load(open(glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*humans/"+s+"/🔣️.json")[0]))['asset']
    dp=a.get('presentation',{}).get('defaultPose') or a['poses'][0]['id']
    dims=a['dimensions']; D=[dims['widthM'],dims['heightM'],dims['depthM']]
    poses=open(G+s+".poses.txt").read().splitlines()
    pl=[l for l in poses if l.startswith("pose ")]
    dl=[l for l in pl if l.startswith("pose "+dp+":")][0]
    m=re.search(r"bbox ([\d.]+) x ([\d.]+) x ([\d.]+)",dl); B=[float(x) for x in m.groups()]
    dpe=["%+.1f"%((B[i]-D[i])/D[i]*100) for i in range(3)]
    re_=["%+.1f"%(b['relativeError']*100) for b in c['bbox']]
    iou=[ "%.2f"%v['iou'] for v in c['views']]
    val=open(G+s+".validate.txt").read().splitlines()[0]
    vi=re.search(r"volume rules (\d+)",val).group(1); oth=re.search(r"other validateAsset (\d+)",val).group(1)
    flips=[l for l in pl if "SIDE FLIPS" in l]; det=[l for l in pl if "NEW DETACH" in l]
    rows.append((s,c['solids'],"/".join(re_),"/".join(dpe),dp,"/".join(iou),vi,oth,len(pl),bool(flips),bool(det)))
print("| slug | solids | bbox err % x/y/z at rest | bbox err % x/y/z in default pose | default pose | IoU e/f/p | validator volume issues | other validateAsset | poses checked | flips | detach |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows: print("| "+" | ".join(str(x) for x in r)+" |")
