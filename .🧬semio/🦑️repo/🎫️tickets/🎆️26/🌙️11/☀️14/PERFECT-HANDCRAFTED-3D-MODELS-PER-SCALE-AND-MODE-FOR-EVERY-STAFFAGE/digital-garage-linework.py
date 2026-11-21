"""🚪 Keep the moving panels, hinges and hanger stock legible through each drawing band."""
from pathlib import Path
import json
T=Path(__file__).resolve().parent
CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/sectional-garage-door/🔣️.json'))
def author(asset):
    for d,stems,pen in [('200',('hanger-','brace-','hinge-axis-','track-left','track-right'),.15),('500',('panel-','track-left','track-right'),.18)]:
        for shapes in asset['representations'][d]['geometry'].values():
            for s in shapes:
                if s['id'].startswith(stems):
                    s['penMm']=max(.20 if s['id'].startswith('hinge-axis-') else pen,s.get('penMm') or asset['representations'][d]['pens'][s['role']] or 0)
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (T/'🗑️generated/digital-garage-linework.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
