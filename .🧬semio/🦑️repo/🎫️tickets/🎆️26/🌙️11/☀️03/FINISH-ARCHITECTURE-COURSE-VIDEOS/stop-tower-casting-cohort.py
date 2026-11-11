"""Stop only the verified six-video worker cohort and its active partials."""
import sys,shutil
from pathlib import Path
sys.path.insert(0,str(Path.cwd()/".venv"/"Lib"/"site-packages"))
import psutil
root=psutil.Process(11400)
expected=["grasshopper_tower","grasshopper_casting","excel_anatomy","excel_references","excel_program","excel_energy"]
args=root.cmdline()
videos=[args[i+1] for i,x in enumerate(args[:-1]) if x=="--video"]
if videos!=expected or root.name().lower()!="python.exe": raise RuntimeError("main video guard")
children=root.children(recursive=True)
print("main",root.pid,"children",[(p.pid,p.ppid(),p.cmdline()[-1]) for p in children],flush=True)
workers=[p for p in children if p.pid in (67340,56276)]
if len(workers)!=2: raise RuntimeError("worker guard")
for p,scene in zip(sorted(workers,key=lambda x:x.pid),("Beat4_Casting","Beat3_Tower")):
    a=p.cmdline()
    if a[-1]!=scene or not any(a[i:i+2]==["-m","manim"] for i in range(len(a)-1)): raise RuntimeError("scene guard")
for p in reversed(children):
    try:p.terminate()
    except psutil.NoSuchProcess:pass
root.terminate()
gone,alive=psutil.wait_procs([root,*children],timeout=10)
if alive: raise RuntimeError("processes alive: "+repr([p.pid for p in alive]))
for number,name in (("122","Beat3_Tower"),("123","Beat4_Casting")):
    parent=(Path(".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s")/number/"p").resolve()
    target=(parent/name).resolve()
    if target.parent!=parent or target.name!=name: raise RuntimeError("path guard")
    if target.exists(): shutil.rmtree(target)
    print("cleared",target,flush=True)
