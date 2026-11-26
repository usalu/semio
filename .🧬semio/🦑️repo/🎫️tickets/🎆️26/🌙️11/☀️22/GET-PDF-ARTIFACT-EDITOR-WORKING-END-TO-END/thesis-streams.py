import pymupdf,sys,re,collections
d=pymupdf.open(sys.argv[1])
n=d.xref_length()
kinds=collections.Counter();enc=collections.Counter();dec=collections.Counter()
for x in range(1,n):
    try:
        o=d.xref_object(x,compressed=False)
        if not d.xref_is_stream(x): continue
        raw=len(d.xref_stream_raw(x)); plain=len(d.xref_stream(x))
    except Exception as e: continue
    t="other"
    m=re.search(r"/Subtype\s*/(\w+)",o)
    if "/Length1" in o: t="fontfile-type1/truetype"
    elif "/Type/XObject" in o.replace(" ","") and "/Image" in o: t="image"
    elif "/Type/ObjStm" in o.replace(" ",""): t="objstm"
    elif "/Type/XRef" in o.replace(" ",""): t="xref"
    elif "/FunctionType" in o: t="function"
    elif "/Subtype/Type1C" in o.replace(" ","") or "/Subtype/CIDFontType0C" in o.replace(" ",""): t="cff"
    kinds[t]+=1;enc[t]+=raw;dec[t]+=plain
for t in kinds: print(t,kinds[t],"encoded",enc[t],"decoded",dec[t])
print("total encoded",sum(enc.values()),"decoded",sum(dec.values()))
