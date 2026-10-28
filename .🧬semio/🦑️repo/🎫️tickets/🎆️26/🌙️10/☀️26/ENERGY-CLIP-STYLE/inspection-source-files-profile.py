"""🔎 Measure actual dependency discovery without UI frames."""
import importlib,pathlib,sys,time,json,os
r=pathlib.Path("C:/git/semio")
t=pathlib.Path(__file__).parent
sys.path.insert(0,str(r))
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]=str(t/"🗑️generated/inspection-cache")
m=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🐍️")
rows=[]
for name in ("_membership","_identity","_call"):
 original=getattr(m,name)
 def wrapper(*args,_name=name,_original=original,**kwargs):
  start=time.perf_counter()
  result=_original(*args,**kwargs)
  row={"function":_name,"seconds":time.perf_counter()-start,"count":len(result) if isinstance(result,(list,tuple)) else None}
  rows.append(row)
  print("[DEBUG] "+json.dumps(row),flush=True)
  return result
 setattr(m,name,wrapper)
for label in ("cold","warm"):
 start=time.perf_counter(); files=m.source_files(); elapsed=time.perf_counter()-start
 members=m._membership(m._SCAN_SOURCES)
 covered={str(p) for p in files}
 missing=sorted(set(members)-covered)
 rows.append({"phase":label,"seconds":elapsed,"sources":len(files),"members":len(members),"missing_count":len(missing),"missing_sample":missing[:12]})
 print("[DEBUG] "+json.dumps(rows[-1],ensure_ascii=False),flush=True)
(t/"inspection-source-files-performance.md").write_text("# Browser Source Files Performance\n\nActual source-only profiling; no UI frames or production edits.\n\n"+json.dumps(rows,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
