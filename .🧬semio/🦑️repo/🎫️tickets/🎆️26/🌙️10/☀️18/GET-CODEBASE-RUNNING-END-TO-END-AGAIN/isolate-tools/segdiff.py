"""🔍 [isolate] shows changed segments (±W chars of context) between the live and the mirror copy of each file of a batch. usage: segdiff.py <batch|rel-list-file> [W=70]"""
import difflib, os, sys
HERE=os.path.dirname(os.path.abspath(__file__)); arg=sys.argv[1]; W=int(sys.argv[2]) if len(sys.argv)>2 else 70
path=os.path.join(HERE,"batches",arg+".txt") if not os.path.exists(arg) else arg
for rel in [l.strip() for l in open(path,encoding="utf-8") if l.strip()]:
    a=open("/Users/ueli/Documents/semio/"+rel,encoding="utf-8").read(); m="/private/tmp/m/is1/"+rel; b=open(m,encoding="utf-8").read() if os.path.exists(m) else ""
    if a==b: continue
    print("##", rel.split("🔨️modules/")[-1])
    al,bl=a.split("\n"),b.split("\n")
    for op,i1,i2,j1,j2 in difflib.SequenceMatcher(None,al,bl,autojunk=False).get_opcodes():
        if op=="equal": continue
        x,y="\n".join(al[i1:i2]),"\n".join(bl[j1:j2])
        if op!="replace" or len(x)>20000: print("  L%d %s: -%s +%s" % (i1+1,op,x[:W*2].replace("\n","⏎"),y[:W*3].replace("\n","⏎"))); continue
        for o,k1,k2,l1,l2 in difflib.SequenceMatcher(None,x,y).get_opcodes():
            if o!="equal": print("  L%d: …%s[-%s-]{+%s+}%s…" % (i1+1+x.count("\n",0,k1),x[max(0,k1-W):k1].replace("\n","⏎"),x[k1:k2].replace("\n","⏎"),y[l1:l2].replace("\n","⏎"),x[k2:k2+25].replace("\n","⏎")))
