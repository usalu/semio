import ast,json
from pathlib import Path
s=Path("C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial")
out=[]
for f in sorted(s.glob("energy/demand/*/full_*video.py")):
 tr=ast.parse(f.read_text(encoding="utf-8-sig")); playlists=[]
 for n in tr.body:
  value=n.value if isinstance(n,(ast.Assign,ast.AnnAssign)) else None
  targets=n.targets if isinstance(n,ast.Assign) else [n.target] if isinstance(n,ast.AnnAssign) else []
  if any(isinstance(x,ast.Name) and "PLAYLIST" in x.id for x in targets): playlists.append(ast.unparse(value))
 builders=[]
 for p in sorted(f.parent.rglob("build_full_video.py")):
  pt=ast.parse(p.read_text(encoding="utf-8-sig"))
  scenes=[]
  for n in ast.walk(pt):
   if isinstance(n,ast.Assign) and any(isinstance(x,ast.Name) and x.id=="scenes" for x in n.targets): scenes.append(ast.unparse(n.value))
  calls=[ast.unparse(n) for n in ast.walk(pt) if isinstance(n,ast.Assign) and any(isinstance(x,ast.Name) and x.id=="render_cmd" for x in n.targets)]
  builders.append({"path":str(p.relative_to(s)),"scenes":scenes,"render_commands":calls})
 out.append({"script":str(f.relative_to(s)),"playlists":playlists,"builders":builders})
print(json.dumps(out,ensure_ascii=False,indent=2))
output=Path(__file__).parent/"🗑️generated"/"segment-audit.json"
output.parent.mkdir(parents=True,exist_ok=True)
output.write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding="utf-8")
