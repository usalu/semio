import csv,os,re,sys
C=sys.argv[1];apply=len(sys.argv)>2
pat=re.compile(r"\n    // 🚫️async: E1 pure inherent-impl helper[^\n]*\n    pub fn register\(\) \{\n        register_composer_entries\([^\n]*\)\.expect\(\"static Stdio registration must be available and conflict-free\"\);\n    \}\n")
for r in csv.DictReader(open(C+"/1105b-policy-counts.tsv"),delimiter="\t"):
    if r["unwrap_expect"]=="0":continue
    m=r["dir"]+"/🪞️mirror";mf=m+"/🦀️.rs";t=open(mf,encoding="utf-8").read()
    callers=[]
    seen=set()
    def walk(f):
        if f in seen or not os.path.exists(f):return
        seen.add(f);s=open(f,encoding="utf-8").read()
        if re.search(r"io_registry::register\(\)",s) and f!=mf:callers.append(f)
        for p in re.findall(r'#\[path = "([^"]*\.rs)"\]',s):walk(os.path.normpath(os.path.join(os.path.dirname(f),p)))
    walk(mf)
    lib=os.path.join(r["dir"],"🦀️.rs");walk(lib)
    n=len(pat.findall(t))
    print(r["crate"],"callers",len(callers),"match",n,"files",len(seen))
    if apply and not callers and n==1:
        t2=pat.sub("\n",t,count=1)
        t2=t2.replace("semio_plugin::io::register_composer_entries,","",1)
        open(mf,"w",encoding="utf-8").write(t2)
