"""🌿 Verify that a machine-readable checkpoint reports the measured failing view and the repaired one."""
import json,sys
from pathlib import Path
from runpy import run_path
t=Path(__file__).resolve().parent
g=t/'🗑️generated'
case=json.loads((t/'digital-view-results.json').read_bytes())
parse=run_path(str(t/'digital-view-results.py'))['issue_lines']
for name,before in [('digital-birch-led-100-elevation.json',True),('digital-birch-fringe-sil.json',False)]:
    row=json.loads((g/name).read_bytes())
    metric=next(m for m in row['measurements'] if m['name']==case['metric'])
    assert metric['value']==case['beforeM' if before else 'afterM'] and metric['limit']==case['limitM']
    lines=parse(row,case['id'],row['hash'],case['scale'],case['view'])
    assert bool(lines)==before
    if before:assert case['path'] in lines[0] and case['metric'] in lines[0]
    for name,value in [('view','plan'),('hash','wrong'),('id','wrong'),('scale',500)]:
        try:parse({**row,name:value},case['id'],row['hash'],case['scale'],case['view'])
        except ValueError:continue
        raise AssertionError('Accepted a mismatched '+name)
print('Both numerical checkpoints, four identity fields and the measured failure passed.')
