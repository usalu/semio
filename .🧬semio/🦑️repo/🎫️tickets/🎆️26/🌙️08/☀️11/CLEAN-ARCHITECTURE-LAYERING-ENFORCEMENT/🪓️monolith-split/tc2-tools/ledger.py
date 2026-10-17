"""tc2: ledger of production-file edits: widened items, testkit gates, mount removals (orig snapshot vs working tree)."""
import difflib,os,re,sys,collections,json
sys.path.insert(0,os.path.dirname(__file__))
import fixrt
R="/Users/ueli/Documents/semio/"
widen=[];gates=0;mounts=0;cfgattr=0;files=0;inc=0;cat=collections.Counter()
for l in open(fixrt.ORIG+"/index.tsv"):
    h,sha,p=l.rstrip("\n").split("\t")
    if not p.endswith(".rs") or not os.path.exists(p): continue
    old=open(fixrt.ORIG+f"/{h}.rs").read().split("\n"); new=open(p).read().split("\n")
    if old==new: continue
    files+=1
    is_test="🧪️tests" in p
    sm=difflib.SequenceMatcher(None,old,new,autojunk=False)
    for tag,i1,i2,j1,j2 in sm.get_opcodes():
        if tag=="equal": continue
        o=old[i1:i2]; n=new[j1:j2]
        if tag=="replace" and len(o)==len(n):
            for a,b in zip(o,n):
                if re.sub(r"pub(\([^)]*\))?\s+","",b.strip(),count=1)==re.sub(r"pub(\([^)]*\))?\s+","",a.strip(),count=1) and a.strip()!=b.strip() and not is_test:
                    widen.append((p[len(R):],b.strip()[:110])); break
            else:
                pass
        for a in o:
            if "#[cfg(test)]" in a or "mod tests" in a or "mod " in a and "#[path" not in a: pass
    t_old="\n".join(old); t_new="\n".join(new)
    cfgattr+=len(re.findall(r"cfg_attr\(any\(test, feature = \"testkit\"\)",t_new))-len(re.findall(r"cfg_attr\(any\(test, feature = \"testkit\"\)",t_old))
    inc+=len(re.findall(r"include_(?:str|bytes)!",t_old))-len(re.findall(r"include_(?:str|bytes)!",t_new))
    if not is_test and len(re.findall(r"#\[cfg\(test\)\]",t_old))>len(re.findall(r"#\[cfg\(test\)\]",t_new)): mounts+=len(re.findall(r"#\[cfg\(test\)\]",t_old))-len(re.findall(r"#\[cfg\(test\)\]",t_new))
print("rs files changed",files,"cfg_attr gated",cfgattr,"include sites removed",inc,"cfg(test) mount lines removed from production files",mounts)
print("widened items:",len(widen))
for w in widen: print("  ",w[0].split("subsets/")[-1][-70:],"|",w[1])
