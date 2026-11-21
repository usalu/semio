"""🌱 Verify each requested living asset in a separate sequential process."""
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('mode', choices=['strict','glb'])
parser.add_argument('assets', nargs='*')
parser.add_argument('--fresh', action='store_true')
parser.add_argument('--bun', default=os.environ.get('BUN_BINARY') or shutil.which('bun'))
options=parser.parse_args()
if not options.bun:
    parser.error('Supply the repository runtime with --bun or BUN_BINARY.')
here=Path(__file__).parent
root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
subjects=sorted([*root.glob('*animals/*/*.json'),*root.glob('*vegetation/*/*.json')],key=lambda p:p.parent.name)
for path in subjects:
    slug=path.parent.name
    if options.assets and slug not in options.assets:
        continue
    target=here/'🗑️generated'/'living-current'/f'{slug}-{options.mode}.json'
    digest=hashlib.sha256(path.read_bytes()).hexdigest()
    if not options.fresh and target.exists() and json.loads(target.read_text())['hash']==digest:
        continue
    command=[sys.executable,here/'equipment-guard.py',options.bun,'--smol',here/'living-current.ts',slug,options.mode]
    outcome=subprocess.run(command)
    if outcome.returncode==0:
        continue
    if options.mode!='strict' or outcome.returncode not in (-15,75,241):
        raise SystemExit(outcome.returncode)
    all_findings=[]
    for scale in ('50','100','200','500'):
        part=here/'🗑️generated'/'living-current'/f'{slug}-strict-{scale}.json'
        if options.fresh or not part.exists() or json.loads(part.read_text())['hash']!=digest:
            subprocess.run(command+[scale],check=True)
        audited=json.loads(part.read_text())
        if audited['hash']!=digest:
            raise SystemExit(f'{slug} changed during verification')
        all_findings.extend(audited['findings'])
    if hashlib.sha256(path.read_bytes()).hexdigest()!=digest:
        raise SystemExit(f'{slug} changed during verification')
    target.write_text(json.dumps({'id':slug,'hash':digest,'scales':[50,100,200,500],'findings':all_findings},ensure_ascii=False))

