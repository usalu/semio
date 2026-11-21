"""✒️ Set the paper weight of individual visible stock edges and facial contours."""
from pathlib import Path
from runpy import run_path
import json,sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated'
cases=json.loads((t/'digital-detail-lines.json').read_text())
def make(slug,publish):
    path=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/'+slug+'/🔣️.json'))
    def apply(asset):
        for scale,marks in cases[slug].items():
            rep=asset['representations'][scale]
            for shapes in rep['geometry'].values():
                for q in shapes:
                    if q['id'] in marks:q['penMm']=max(marks[q['id']],q.get('penMm') or rep['pens'][q['role']] or 0)
    target=g/('digital-lines-'+slug+'.json')
    if publish:run_path(str(t/'digital-publish-asset.py'))['publish'](apply,path,target)
    else:
        doc=json.loads(path.read_bytes());apply(doc['asset']);target.write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
if __name__=='__main__':
    for n in sys.argv[2:]:make(n,sys.argv[1]=='publish')
