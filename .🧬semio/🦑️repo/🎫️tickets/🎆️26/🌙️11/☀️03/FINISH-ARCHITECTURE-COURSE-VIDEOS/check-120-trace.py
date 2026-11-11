import importlib,json
from pathlib import Path
r=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
clip=Path('.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/s/120/v/Beat1_Anatomy.mp4').resolve()
state=json.loads(clip.with_suffix('.render.json').read_text(encoding='utf-8'))
marks=r.source_marks(clip,state)
videos=[v for v in r.load_manifest(source_ids=['grasshopper_anatomy']) if v['id']=='grasshopper_anatomy']
segments=[s for v in videos for c in v['chapters'] for s in c['segments'] if s['script']==state['segment']['script'] and s['scene']==state['segment']['scene']]
print('matching_segments',len(segments),'excerpt_segments',sum('clauses' in s for s in segments),'bound_marks',None if marks is None else len(marks))
for s in segments:
 if 'clauses' in s:
  if marks is None: raise RuntimeError('missing state-bound marks')
  print('interval',s['clauses'],r.clause_interval(marks,**s['clauses']))
raw=r.source_trace_path(state['segment'],clip.parents[3])
print('raw_trace',raw,'exists',raw.is_file())
if raw.is_file():
 data=json.loads(raw.read_text(encoding='utf-8')); print('raw_type',type(data).__name__,'keys',list(data)[:8] if isinstance(data,dict) else len(data))

import ast
scene_path=Path('🎓️teaching')/state['segment']['script']
tree=ast.parse(scene_path.read_text(encoding='utf-8'))
scene=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name==state['segment']['scene'])
assignment=next(n for n in scene.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='NARRATION' for t in n.targets))
value=assignment.value
rows=ast.literal_eval(value.args[0] if isinstance(value,ast.Call) else value)
expected=[row[0] for row in rows]
actual=data['beats'][state['segment']['scene']]
if list(actual)!=expected: raise RuntimeError('narration keys differ from raw trace')
print('raw_current',raw.stat().st_mtime>=clip.stat().st_mtime-30,'beat_count',len(expected),'interval',r.clause_interval(actual,first=expected[0],last=expected[-1]))
