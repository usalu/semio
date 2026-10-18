import re, sys, os
# cfgtest.py <lib-closure-list> <test-file>... [--apply] : #[cfg(test)] items (non-mount) of the owner lib closure whose names the moved test files reference; --apply gates them with `any(test, feature = "testkit")`
R="/Users/ueli/Documents/semio/"
apply = "--apply" in sys.argv
args=[a for a in sys.argv[1:] if a!="--apply"]
files=[l.split("\t")[-1].strip() for l in open(args[0]) if l.count("\t")==4]
text="".join(open(t).read() for t in args[1:])
idents=set(re.findall(r"\b[A-Za-z_]\w{3,}\b", text))
def extent(lines,i):
    j=i; depth=0; seen=False
    while j<len(lines):
        s=re.sub(r'"(?:[^"\\]|\\.)*"','""',lines[j]); s=re.sub(r"//.*","",s)
        depth+=s.count("{")-s.count("}")
        if "{" in s: seen=True
        if (seen and depth<=0) or (not seen and s.rstrip().endswith(";")): return j
        j+=1
    return j
hits=0
for f in files:
    p=R+f
    lines=open(p).read().split("\n")
    changed=False
    for i,l in enumerate(lines):
        if re.match(r"\s*#\[cfg\(test\)\]\s*$",l):
            k=i+1
            while k<len(lines) and lines[k].strip().startswith("#["): k+=1
            if k>=len(lines): continue
            if "#[path" in lines[i+1] or re.match(r"\s*(pub )?mod tests?\b", lines[k]): continue
            e=extent(lines,k)
            blk="\n".join(lines[k:e+1])
            names=set(re.findall(r"\b(?:fn|const|static|struct|enum|trait|type|mod|macro_rules!)\s+(\w+)",blk))
            if re.match(r"\s*(unsafe\s+)?(impl|use|extern)\b",lines[k]):
                if lines[k].lstrip().startswith("use") or lines[k].lstrip().startswith("extern"):
                    names|=set(re.findall(r"\b\w{4,}\b",blk))
            m=[n for n in names if len(n)>=4 and n in idents]
            if m:
                hits+=1
                print(f[-50:], i+1, lines[k].strip()[:80], "->", m[:6])
                if apply:
                    lines[i]=l.replace("#[cfg(test)]",'#[cfg(any(test, feature = "testkit"))]'); changed=True
    if changed: open(p,"w").write("\n".join(lines))
print("hits",hits)
