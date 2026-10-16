"""🔬 Runs the baseline differential for the pack crate and pickles the error multisets."""
import os,pickle,shutil,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import oracle as o
SP=o.SCR
root=SP+"/S0"
shutil.rmtree(root,ignore_errors=True); shutil.copytree(SP+"/base",root,symlinks=True)
o.patch_tree(root)
lib=root+"/"+o.FW+"/🎒️pack/📦️packages/🦀️rust/🦀️.rs"
res={}
for tag,extra in o.CFGS.items():
    if tag=="native-nodeflate": c,err=o.run(root,lib,"pack",extra)
    else: c,err=o.run(root,lib,"pack",extra+['--cfg','feature="deflate"'])
    if tag=="native-nodeflate": pass
    res[tag]=c
    open(f"{SP}/err-S0-{tag}.txt","w").write(err)
    print(tag,sum(c.values()),flush=True)
pickle.dump(res,open(SP+"/base-pack.pkl","wb"))
