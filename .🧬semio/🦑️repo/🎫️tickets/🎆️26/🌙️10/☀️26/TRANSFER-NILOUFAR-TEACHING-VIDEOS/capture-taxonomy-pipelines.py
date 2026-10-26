"""📸 Capture authored ordering and narration contracts before path-only edits."""
import ast
import hashlib
import json
from pathlib import Path
TICKET=Path(__file__).resolve().parent
ROOT=Path('C:/git/semio')
plan=json.loads((TICKET/'taxonomy-paths.cases.json').read_text(encoding='utf-8'))
rows=[]
for move in plan['moves']:
    old=move['old']
    if not move['new'] or not (old.endswith('/build_full_video.py') or old.endswith('/generate_audio.py') or '/full_' in old and old.endswith('.py')): continue
    tree=ast.parse((ROOT/move['new']).read_text(encoding='utf-8'))
    protected={}
    for node in tree.body:
        if isinstance(node,ast.ClassDef) or isinstance(node,(ast.Assign,ast.AnnAssign)) and any(isinstance(target,ast.Name) and (target.id in ('BEATS','NARRATIONS') or target.id.endswith('_PLAYLIST')) for target in (node.targets if isinstance(node,ast.Assign) else [node.target])):
            label=node.name if isinstance(node,ast.ClassDef) else (node.targets[0] if isinstance(node,ast.Assign) else node.target).id
            protected[label]=hashlib.sha256(ast.dump(node,include_attributes=False).encode()).hexdigest()
    rows.append({'path':move['new'],'kind':'composition' if old.endswith('/build_full_video.py') else 'speech' if old.endswith('/generate_audio.py') else 'lesson','protected':protected})
(TICKET/'taxonomy-pipelines.tests.json').write_text(json.dumps({'counts':{'lesson':5,'composition':25,'speech':26},'workspace_marker':'🧰️framework','forbidden':['from scene_','from manim_visuals','from video_runtime','from tts_pipeline','manim_fonts.py','sys.modules['],'modules':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Captured',len(rows),'module ordering/narration contracts')
