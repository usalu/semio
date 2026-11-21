"""🧑 Install the specific independently inspected face meshes and their physiological references."""
from pathlib import Path
from runpy import run_path
import json
t=Path(__file__).resolve().parent;g=t/'🗑️generated'
dest=next(t.parents[6].glob('🎓*/*architecture/*staffage/*catalog/*/walking-commuter/*.json'))
checked=json.loads((g/'digital-face-fitted.json').read_bytes())['asset']
candidate=g/'digital-commuter-faced.json'
current=json.loads(candidate.read_bytes())['asset']
for k in ('50','100','200','500'):
    assert current['volumes'][k]['body']==checked['volumes'][k]['body']
    assert current['volumes'][k]['thermal']==checked['volumes'][k]['thermal']
    row=json.loads(next(l for l in (g/('digital-face-'+k+'.log')).read_text().splitlines() if l.startswith('{')))
    assert row['issues']==[] and len(row['rules'])==13
def author(a):
    for k in ('50','100'):
        assert all(not b['id'].startswith('head-eyelid-') for b in a['volumes'][k]['body'])
        for field in ('body','thermal','decisions'):a['volumes'][k][field]=current['volumes'][k][field]
    assert a==current
run_path(str(t/'digital-publish-asset.py'))['publish'](author,dest,candidate)
