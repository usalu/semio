"""📊 Inspect only this ticket's complete physical-render work."""
import json,re,time
from pathlib import Path
root=Path(__file__).with_name('🗑️generated')/'full-physical'
for log in sorted(root.glob('de/spectator/s/*/attempt-*.log')):
    metadata=json.loads(log.with_suffix('.json').read_text(encoding='utf-8'))
    content=log.read_text(encoding='utf-8',errors='replace')
    animation=re.findall(r'Animation (\d+)',content)
    clip=list((log.parent/'v').glob('Beat*.mp4'))
    print(json.dumps({'ordinal':log.parent.name,'scene':metadata['command'][-1],'last_animation':int(animation[-1]) if animation else None,'log_age_seconds':round(time.time()-log.stat().st_mtime),'exit_code':metadata.get('exit_code'),'complete_bytes':clip[0].stat().st_size if clip else None},ensure_ascii=False))
