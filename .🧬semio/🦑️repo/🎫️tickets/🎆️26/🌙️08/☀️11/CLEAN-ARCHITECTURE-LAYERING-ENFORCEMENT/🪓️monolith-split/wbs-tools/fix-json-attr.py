import re,sys
todo=[l.rstrip("\n").split(" -> ") for l in open(sys.argv[1],encoding="utf8") if " -> " in l]
bad=0;done=0
for path,spec in todo:
    s=open(path,encoding="utf8").read()
    pat=re.compile(r'((?:import|export)\s*[^"\';]*?\s*from\s*"'+re.escape(spec)+r'")(\s*;?)')
    m=list(pat.finditer(s))
    if len(m)!=1:
        print("SKIP",len(m),path,spec);bad+=1;continue
    if 'with' in s[m[0].end(1):m[0].end(1)+8]: continue
    s=s[:m[0].end(1)]+' with{type:"json"}'+s[m[0].end(1):]
    open(path,"w",encoding="utf8").write(s);done+=1
print("fixed",done,"skipped",bad)
