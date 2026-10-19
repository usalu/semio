import re,rs,unused,os,subprocess
def prune(p):
    names=unused.report(p)
    if not names:return []
    s=open(p,encoding="utf-8").read()
    for n in names:
        # whole-line single use
        s2=re.sub(r"(?m)^use [^\n{]*::"+re.escape(n)+r";\n","",s)
        if s2!=s:s=s2;continue
        # inside braces
        s2=re.sub(r"\{([^{}]*)\}",lambda m:"{"+re.sub(r"(?:^|,)\s*"+re.escape(n)+r"\s*(?=,|$)","",m.group(1)).lstrip(", ")+"}" if re.search(r"(?:^|,)\s*"+re.escape(n)+r"\s*(?=,|$)",m.group(1)) else m.group(0),s,count=0)
        s=s2
    open(p,"w",encoding="utf-8").write(s)
    return names
if __name__=="__main__":
    for d in sorted(os.listdir(rs.SUBSETS)):
        p=rs.SUBSETS+d+"/"+rs.SNAP+"🛬️native/🦀️.rs"
        if os.path.exists(p):
            got=prune(p)
            r=subprocess.run(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true",p],capture_output=True,text=True)
            print(d,got,r.returncode,unused.report(p))
