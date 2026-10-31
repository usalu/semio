import json,re,sys
R="/Users/ueli/Documents/semio"
def register(rel,after,project):
    for p,pre in [(R+"/Cargo.toml",""),(R+"/🧰️framework/Cargo.toml","🧰️framework/")]:
        t=open(p).read()
        a='    "%s",\n'%after.replace(pre,"",1) if pre else '    "%s",\n'%after
        n='    "%s",\n'%(rel.replace(pre,"",1) if pre else rel)
        assert a in t,(p,a)
        assert n not in t,(p,"exists")
        open(p,"w").write(t.replace(a,a+n,1))
    op=R+"/📋️build-owners.json";o=open(op).read()
    row='    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/📋️project.json",\n      "packages": []\n    },\n'%(project,rel,rel)
    names=[(m.start(),m.group(1)) for m in re.finditer(r'    \{\n      "name": "(@semio-tech/[^"]+)"',o)]
    assert project not in o
    pos=next(p_ for p_,n in names if n>project)
    o=o[:pos]+row+o[pos:];json.loads(o);open(op,"w").write(o)
if __name__=="__main__":
    register(*sys.argv[1:4])
