"""📊 Lib and test closure totals for every new os-flow crate. usage: censusall.py [id...]"""
import sys,os,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from build_crates import CR
REPO="/Users/ueli/Documents/semio/"
here=os.path.dirname(os.path.abspath(__file__))
def run(c,extra,tcfg=None):
    root=os.path.normpath(os.path.join(c.pkgdir,c.lib_rel_from_pkg()))
    env=dict(os.environ,TOTAL_ONLY="1",EXTRA=extra)
    if tcfg:env["TARGET_CFG"]=tcfg
    r=subprocess.run(["bun",here+"/closure.ts",root,c.pkgdir],capture_output=True,text=True,env=env,cwd=REPO)
    return (r.stdout.strip().split("\n")[-1] or r.stderr.strip()[-200:])
if __name__=="__main__":
    ids=sys.argv[1:] or list(CR)
    for i in ids:
        c=CR[i]
        print(i.ljust(15),"lib ",run(c,""))
        print(i.ljust(15),"test",run(c,"test"))
