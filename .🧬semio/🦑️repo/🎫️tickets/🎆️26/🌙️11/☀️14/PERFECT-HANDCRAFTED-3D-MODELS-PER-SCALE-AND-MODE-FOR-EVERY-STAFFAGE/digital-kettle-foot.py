"""🫖 Seat the kettle on an explicitly closed annular skirt beneath its bearing plate."""
from pathlib import Path
import json
T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/electric-kettle/🔣️.json'))
def author(asset):
    foot=next(p for p in asset['volumes']['500']['structure'] if p['id']=='member-base')
    if foot['profile'][-1]!=foot['profile'][0]:foot['profile'].append(foot['profile'][0][:])
if __name__=='__main__':
    a=json.loads(CAT.read_text());author(a['asset']);(T/'🗑️generated/digital-kettle-foot.json').write_text(json.dumps(a,ensure_ascii=False))
