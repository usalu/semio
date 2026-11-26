"""🏛️ Link a production view-mesh census to an internally consistent native inference library."""
from pathlib import Path
import json, os, re, subprocess, sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated';b=g/'ntb/debug/build'
rust=subprocess.check_output(['rustup','which','rustc'],text=True).strip()
for archive in sorted((b/'semio-s-artifact-bim-model-inferences').glob('*/out/*.rlib'),key=lambda p:p.stat().st_mtime,reverse=True):
    meta=subprocess.check_output([rust,'-Zls=root',str(archive.with_suffix('.rmeta'))],text=True)
    externs=['bim_graph='+str(archive)]
    for alias,pkg in [('bim_diff','semio-s-artifact-bim-model-diff'),('bim_spaces','semio-s-artifact-bim-model-spaces'),('pack_json','semio-framework-pack-json')]:
        crate=pkg.replace('-','_');matched=re.search(r'^\d+ ('+crate+r'-[a-f0-9]+) hash ',meta,re.M)
        if not matched:break
        a=next((b/pkg).glob('*/out/lib'+matched[1]+'.rlib'),None)
        if not a:break
        externs.append(alias+'='+str(a))
    if len(externs)!=4:continue
    with (g/'view-bodies-link.log').open('w') as log:
        r=subprocess.run([sys.executable,str(t/'link-fixture.py'),*externs],env=dict(os.environ,BIM_FIXTURE_LINK_SOURCE=str(t/'view-bodies.rs'),BIM_FIXTURE_LINK_OUTPUT=str(g/'view-bodies')),stdout=log,stderr=log)
    if r.returncode==0:sys.exit(0)
sys.exit(1)
