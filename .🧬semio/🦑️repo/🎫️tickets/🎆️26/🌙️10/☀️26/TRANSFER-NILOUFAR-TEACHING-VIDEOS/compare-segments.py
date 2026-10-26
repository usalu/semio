import ast,json
from pathlib import Path
src=Path("C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial")
dst=Path("C:/git/semio/🎓️teaching")
manifest=json.loads((dst/"video_manifest.json").read_text(encoding="utf-8-sig"))
results=[]
def targets(n):
 return n.targets if isinstance(n,ast.Assign) else [n.target] if isinstance(n,ast.AnnAssign) else []
def assigned(tree,name):
 return next(n.value for n in ast.walk(tree) if isinstance(n,(ast.Assign,ast.AnnAssign)) and any(isinstance(x,ast.Name) and x.id==name for x in targets(n)))
for video in manifest["videos"]:
 if "chapters" not in video: raise SystemExit("Manifest is still old contract")
 p=src/video["script"]; tree=ast.parse(p.read_text(encoding="utf-8-sig")); modules={}
 for n in tree.body:
  if isinstance(n,ast.Assign) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Name) and n.value.func.id=="_load_module":
   pieces=[x.value for x in ast.walk(n.value.args[-1]) if isinstance(x,ast.Constant) and isinstance(x.value,str)]
   # AST visits binary division right first only after left subtree, so preserve filename/path by source position.
   pieces=sorted([x for x in ast.walk(n.value.args[-1]) if isinstance(x,ast.Constant) and isinstance(x.value,str)],key=lambda x:x.col_offset)
   path=p.parent.joinpath(*(x.value for x in pieces))
   modules[n.targets[0].id]=path
 playlist=assigned(tree,video["playlist"]); expected=[]
 for chapter in playlist.elts:
  beats=chapter.elts[1]
  if isinstance(beats,ast.Call):
   module=beats.args[0].value.id; attr=beats.args[0].attr; path=modules[module]
   names=[x.id for x in assigned(ast.parse(path.read_text(encoding="utf-8-sig")),attr).elts]
   expected.append([{"script":path.relative_to(src).as_posix(),"scene":name} for name in names])
  else:
   expected.append([{"script":modules[x.value.id].relative_to(src).as_posix(),"scene":x.attr} for x in beats.elts])
 actual=[c["segments"] for c in video["chapters"]]
 results.append({"id":video["id"],"chapter_count":len(expected),"beat_counts":[len(c) for c in expected],"exact_original_segment_order":expected==actual,"intro_script_exists":(src/video["intro"]["script"]).is_file(),"actual":actual if actual!=expected else None})
print(json.dumps(results,ensure_ascii=False,indent=2))
assert all(r["exact_original_segment_order"] and r["intro_script_exists"] for r in results)
output=Path(__file__).parent/"🗑️generated"/"compare-segments.json"
output.parent.mkdir(parents=True,exist_ok=True)
output.write_text(json.dumps(results,ensure_ascii=False,indent=2),encoding="utf-8")
