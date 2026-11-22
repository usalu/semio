import json,os,re,sys
R="/Users/ueli/Documents/semio/"
LIB=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
INV=LIB+"🧪️testing/🪪️native-birth/🟦️.ts"
G=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E19/"
rows=[]
for base in ("⚡️caching","🏗️build"):
    d=json.load(open(LIB+base+"/🗂️suites/🔣️.json"))
    for s in d["suites"]: rows.append((LIB+base+"/",s))
def row_of(path):
    exact=[];trees=[]
    for base,s in rows:
        rel=os.path.relpath(path,base)
        if rel in s.get("files",[]): exact.append(s)
        for t in s.get("trees",[]):
            if rel.startswith(t.rstrip("/")+"/"): trees.append((len(t),s))
    if exact: return exact
    if not trees: return []
    m=max(n for n,_ in trees); return [s for n,s in trees if n==m]
def apply(path,write):
    hit=row_of(path)
    if len(hit)!=1: return "ROWS=%d %s"%(len(hit),[h["id"] for h in hit])
    s=hit[0]
    if any(os.path.relpath(path,b)in s.get("fileLaunchers",{}) for b,r in rows if r is s) or s.get("function") or s.get("testNamePattern"): return "LAUNCHER %s"%s["id"]
    text=open(path).read()
    if "declareLibraryBuildTestEnvelopeV1(" in text or "declareBuildTestEnvelopeV1(" in text: return "ALREADY"
    arg='suite:"%s"'%s["id"]
    if s["budgetMs"]!=300000: arg+=',maximumElapsedMilliseconds:%d'%s["budgetMs"]
    if s.get("scratch")=="checkout": arg+=',scratch:"checkout"'
    call='declareLibraryBuildTestEnvelopeV1({%s});'%arg
    rel=os.path.relpath(INV,os.path.dirname(path))
    lines=text.split("\n"); last=-1; merged=False
    for i,l in enumerate(lines):
        if re.match(r'^import\b',l): last=i
        m=re.match(r'^(import \{)([^}]*)(\} from ")([^"]*🪪️native-birth/🟦️\.ts)(".*)$',l)
        if m and os.path.normpath(os.path.join(os.path.dirname(path),m.group(4)))==INV: lines[i]=m.group(1)+"declareLibraryBuildTestEnvelopeV1,"+m.group(2).lstrip()+m.group(3)+m.group(4)+m.group(5); merged=True
    if last<0: return "NOIMPORT"
    ins=([] if merged else ['import {declareLibraryBuildTestEnvelopeV1} from "%s";'%rel])+[call]
    lines[last+1:last+1]=ins
    if write: open(path,"w").write("\n".join(lines))
    return "OK %s"%s["id"]
if __name__=="__main__":
    write=sys.argv[1]=="write"
    for f in sys.argv[2:]:
        print(f.split("/library/")[-1],"->",apply(R+f if not f.startswith("/") else f,write))
