# 🗂️ Generates the mutation leaf case tables deterministically from the committed fixture bundles. usage: gen-tables.py discover <inventory dir> | write | check
# discover: reads the `mutation-fixture-unpaired` findings of per-scope inventories (inv-*.json) into leaves.json (subset root + leaf owner path).
# write/check: for each leaf in leaves.json, `<subset>/🧬️schema/🧬️mutations/<owner>/🧪️tests/🔣️.cases.json` lists every fixture bundle directory of the leaf (a child directory holding all four facets) (NFC, sorted by code point).
import glob,json,os,sys,unicodedata
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT="/Users/ueli/Documents/semio"
MARK="/🧪️tests/🧫️fixtures/🧬️mutations/"
TABLE="🔣️.cases.json"
FACETS=["📸️snapshot","🦠️mutation","🔺️diff","🎯️outcome"]
def discover(directory):
    leaves={}
    for path in sorted(glob.glob(os.path.join(directory,"inv-*.json"))):
        for row in json.load(open(path,encoding="utf8"))["violations"]:
            if row["code"]!="mutation-fixture-unpaired": continue
            at=row["path"].index(MARK); subset=row["path"][:at]; owner=row["path"][at+len(MARK):].rsplit("/",1)[0]
            leaves[(subset,owner)]=None
    rows=[{"subset":s,"owner":o} for s,o in sorted(leaves)]
    json.dump(rows,open(os.path.join(HERE,"leaves.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
    print("leaves",len(rows))
def render(cases):
    return json.dumps({"cases":cases},ensure_ascii=False,indent=2)+"\n"
def run(write):
    leaves=json.load(open(os.path.join(HERE,"leaves.json"),encoding="utf8")); drift=0; total=0
    for row in leaves:
        fixtures=os.path.join(ROOT,row["subset"]+MARK+row["owner"])
        cases=sorted((unicodedata.normalize("NFC",n) for n in os.listdir(fixtures) if all(os.path.isdir(os.path.join(fixtures,n,facet)) for facet in FACETS)))
        raw={unicodedata.normalize("NFC",n):n for n in os.listdir(fixtures)}
        cases=[raw[n] for n in cases]
        total+=len(cases)
        table=os.path.join(ROOT,row["subset"],"🧬️schema/🧬️mutations",row["owner"],"🧪️tests",TABLE)
        text=render(cases)
        have=open(table,encoding="utf8").read() if os.path.isfile(table) else None
        if have==text: continue
        drift+=1
        if write:
            os.makedirs(os.path.dirname(table),exist_ok=True)
            open(table,"w",encoding="utf8").write(text)
    print(("wrote" if write else "drift"),drift,"of",len(leaves),"tables;",total,"cases")
    return drift
if __name__=="__main__":
    mode=sys.argv[1]
    if mode=="discover": discover(sys.argv[2])
    else: sys.exit(1 if run(mode=="write") and mode=="check" else 0)
