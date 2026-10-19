import os,re,sys
ROOT="/Users/ueli/Documents/semio"
def run(src, manifest_dir):
    p=os.path.join(ROOT,src); d=os.path.dirname(p); m=os.path.join(ROOT,manifest_dir)
    s=open(p,encoding='utf8').read()
    def rep(mo):
        rel=mo.group(1); tgt=os.path.normpath(os.path.join(d,rel))
        assert os.path.exists(tgt),(src,rel)
        r=os.path.relpath(tgt,m)
        return 'semio_framework_test_fixtures::fixture_text!("%s")'%r
    n=re.sub(r'include_str!\("([^"]+)"\)',rep,s)
    assert n!=s
    open(p,'w',encoding='utf8').write(n)
    print("ok",src,len(n)-len(s))
if __name__=="__main__":
    run(sys.argv[1],sys.argv[2])

def add_dev_dep(manifest_dir):
    import os
    FX=ROOT+"/🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/📦️packages/🦀️rust"
    f=ROOT+"/"+manifest_dir+"/Cargo.toml"
    s=open(f,encoding='utf8').read()
    if "semio-framework-test-fixtures" in s: print("already",manifest_dir[-30:]); return
    rel=os.path.relpath(FX,os.path.dirname(f))
    line='semio-framework-test-fixtures = { path = "%s" }\n'%rel
    if "[dev-dependencies]\n" in s: s=s.replace("[dev-dependencies]\n","[dev-dependencies]\n"+line,1)
    else: s=s.rstrip("\n")+"\n\n[dev-dependencies]\n"+line
    open(f,'w',encoding='utf8').write(s); print("added",manifest_dir[-30:])
