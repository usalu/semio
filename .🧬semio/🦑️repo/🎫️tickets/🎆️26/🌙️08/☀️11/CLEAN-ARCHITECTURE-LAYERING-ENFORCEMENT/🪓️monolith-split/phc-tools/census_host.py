#!/usr/bin/env python3
import os, sys, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, regs_host
from carve import REPO
SCR = '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/phc-census'


def census(ws, label='run'):
    os.makedirs(SCR, exist_ok=True)
    ov = f'{SCR}/overlay-{label}.json'
    json.dump({p: ws.cur[p] for p in ws.cur}, open(ov, 'w'))
    dl = f'{SCR}/deleted-{label}.json'
    json.dump(sorted(ws.deleted), open(dl, 'w'))
    specs = [dict(name=v['name'], root=v['root'], pkg=regs_host.pkg_dir_of(v['root'])) for k, v in plan_host.CRATES.items()]
    sp = f'{SCR}/specs-{label}.json'
    json.dump(specs, open(sp, 'w'))
    r = subprocess.run(['bun', os.path.join(os.path.dirname(os.path.abspath(__file__)), 'census-overlay.ts')], capture_output=True, text=True, cwd=REPO, env=dict(os.environ, OVERLAY=ov, DELETED=dl, SPECS=sp))
    return r.stdout + (('\n' + r.stderr[-600:]) if r.stderr.strip() else '')


if __name__ == '__main__':
    import state
    ws, model, roots = state.load()
    print(census(ws))
