"""🎬 Run all six actual current preparations and owned physical drawing callbacks without native rendering."""
import ast,contextlib,hashlib,importlib,io,json,marshal,time,traceback
from pathlib import Path
ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated';clip=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');source=clip/'🧱️model/🔣️.json'
expected='528b3ffeab26e4c038b9eb603e7ae1e9bc52c66a1f068b3e799de8fcd66ccc8f'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def producers():return {str(p.relative_to(clip)):digest(p) for p in sorted(clip.rglob('*')) if p.is_file() and p.suffix in {'.py','.mjs','.json'} and '🧪️' not in str(p.relative_to(clip))}
assert digest(source)==expected
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');points=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry')
point=points.P([1.25,2.5]);assert type(point) is tuple and all(type(v) is float for v in point) and point==(1.25,2.5,0.0)
provider=lambda:hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
started=time.monotonic();current='initial';phases=[]
inputs={'source_before':digest(source),'provider_before':provider(),'production_before':producers(),'owned_harnesses':{name:digest(ticket/name) for name in ('clips-preparation-r11.py','chapter-native-drawings-r24.py')},'actual_point':{'value':point,'container':type(point).__name__,'coordinate_types':[type(v).__name__ for v in point],'code_sha256':hashlib.sha256(marshal.dumps(points.P.__code__)).hexdigest()},'scope':'Fresh retained EN preparation paths; physical callbacks and independent geometry oracles; native frames/fonts/captions excluded'}
(gen/'clips-current-r86-inputs.json').write_text(json.dumps(inputs),encoding='utf8')
def integration_progress(detail):
 if (gen/'clips-current-r86-cancel').exists():raise InterruptedError('Explicit integration cancellation')
 (gen/'clips-current-r86-progress.json').write_text(json.dumps({'debug':'[DEBUG] fresh six actual preparations and physical callbacks','stage':current,'detail':detail,'completed_stages':len(phases),'elapsed':time.monotonic()-started,'source':digest(source)}),encoding='utf8')
required={'preparations':['PASS object preparation:','PASS exchange preparation:','PASS consistency preparation:','PASS actual models preparation:','PASS actual representations preparation:','PASS actual information preparation:'],'callbacks':['PASS actual entry drawing:','PASS five actual facade cut callbacks:','PASS actual hypothetical clash preparation/drawing:','PASS actual native entrance material callback:','PASS actual selected-column floor section:']}
for current,name in [('preparations','clips-preparation-r11.py'),('callbacks','chapter-native-drawings-r24.py')]:
 integration_progress('starting actual owned harness')
 stage_start=time.monotonic();source_start=digest(source);production_start=producers();output=io.StringIO();tree=ast.parse((ticket/name).read_text())
 if current=='preparations':
  position=next(i for i,n in enumerate(tree.body) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='objects' for t in n.targets))
  tree.body.insert(position,ast.Assign(targets=[ast.Name(id='progress',ctx=ast.Store())],value=ast.Name(id='integration_progress',ctx=ast.Load())))
 else:
  method=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='compile_methods')
  method.body.insert(1,ast.Expr(value=ast.Call(func=ast.Name(id='integration_progress',ctx=ast.Load()),args=[ast.BinOp(left=ast.Constant(value='compile actual callbacks: '),op=ast.Add(),right=ast.Name(id='folder',ctx=ast.Load()))],keywords=[])))
 namespace={'__file__':str(ticket/name),'__name__':'current_owned_'+current,'integration_progress':integration_progress}
 try:
  with contextlib.redirect_stdout(output),contextlib.redirect_stderr(output):exec(compile(ast.fix_missing_locations(tree),str(ticket/name),'exec'),namespace)
  lines=output.getvalue().splitlines();assert all(any(line.startswith(prefix) for line in lines) for prefix in required[current])
  result={'stage':current,'pass':True,'required_contracts':len(required[current])}
 except Exception:result={'stage':current,'pass':False,'traceback':traceback.format_exc()};lines=output.getvalue().splitlines()
 result.update(seconds=time.monotonic()-stage_start,source_start=source_start,source_end=digest(source),provider_end=provider(),production_start=production_start,production_end=producers(),output_lines=lines if len(lines)<=200 else lines[:100]+lines[-100:],output_line_count=len(lines),output_truncated=len(lines)>200)
 phases.append(result)
 (gen/'clips-current-r86-stages.json').write_text(json.dumps(phases),encoding='utf8')
 if not result['pass']:break
record={**inputs,'seconds':time.monotonic()-started,'phases':phases,'pass':len(phases)==2 and all(p['pass'] for p in phases),'source_after':digest(source),'provider_after':provider(),'production_after':producers(),'actual_point_after':{'container':type(points.P([1.25,2.5])).__name__,'code_sha256':hashlib.sha256(marshal.dumps(points.P.__code__)).hexdigest()},'native_frames':False}
(gen/'clips-current-r86-result.json').write_text(json.dumps(record),encoding='utf8');print({'pass':record['pass'],'seconds':record['seconds'],'stages':[(p['stage'],p['pass'],p['seconds']) for p in phases],'source_equal':inputs['source_before']==record['source_after'],'provider_equal':inputs['provider_before']==record['provider_after'],'production_equal':inputs['production_before']==record['production_after']})
