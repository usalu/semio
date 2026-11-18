"""Summary of a triage jsonl. python3 humansb-summary.py <file.jsonl>"""
import json, sys
for l in open(sys.argv[1]):
    r = json.loads(l)
    if 'error' in r: print(r); continue
    f = []
    for x in r['fails']:
        k = x['kind']
        if k == 'bbox': f.append(f"{x['scale']}:b{x['axis']}({x['got']}/{x['want']})")
        elif k == 'iou': f.append(f"{x['scale']}:i{x['view'][0]}({x['iou']})")
        elif k == 'hausdorff': f.append(f"{x['scale']}:h{x['view'][0]}({x['h']})")
        else: f.append(str(x))
    print(r['asset'], len(f), r['seconds'], ' '.join(f))
