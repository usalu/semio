import sys,time
from pathlib import Path
T=Path(__file__).resolve().parents[1]
sys.stdout.reconfigure(encoding="utf-8")
g=next(p for p in T.iterdir() if p.name.endswith("generated"))
for d in sorted((g/"preview").iterdir()):
    if d.is_dir() and d.name.startswith(("Beat","Optim")):
        f=[p for p in d.rglob("*.mp4")]
        new=max((p.stat().st_mtime for p in f),default=0)
        print(d.name,len(f),"files","last",round((time.time()-new)/60,1) if f else "-","min ago", sum(p.stat().st_size for p in f))
print((g/"previews-bim-opt.out").read_text(encoding="utf-8"))
