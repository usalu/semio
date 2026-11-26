"""🏗️ Regenerate the exchange witnesses from the freshly compiled native libraries."""
import os
import re
from pathlib import Path
import subprocess
import sys
import time

ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
subset = Path('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any')
fixtures = subset / '🧪️tests/🧫️fixtures'
notated = fixtures / '💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json'
def archive_after_sources():
    directory = generated / 'ntb/debug/build/semio-s-artifact-bim-model-ifc'
    inputs = subset / '🚪️io/📤️export/🏗️ifc'
    minimum = max((inputs/'🦀️.rs').stat().st_mtime, (inputs/'🔥️energy/🦀️.rs').stat().st_mtime, (inputs/'📏️grids/🦀️.rs').stat().st_mtime)
    return next((p for p in directory.glob('*/out/*.rlib') if p.stat().st_mtime > minimum), None)

while not (archive := archive_after_sources()):
    time.sleep(3)
compiler = subprocess.check_output(['rustup','which','rustc'],text=True).strip()
metadata = subprocess.check_output([compiler,'-Zls=root',str(archive.with_suffix('.rmeta'))],text=True)
def dependency(package):
    name = package.replace('-','_')
    match = re.search(r'^\d+ ('+name+r'-[a-f0-9]+) hash ',metadata,re.M)
    family = generated/'ntb/debug/build'/package
    if match:
        return next(family.glob('*/out/lib'+match[1]+'.rlib'))
    snapshot = re.search(r'^\d+ (semio_s_artifact_bim_model_snapshot-\S+ hash \w+)',metadata,re.M)[1]
    candidates = sorted(family.glob('*/out/lib'+name+'-*.rlib'),key=lambda item:item.stat().st_mtime,reverse=True)
    return next(item for item in candidates if snapshot in subprocess.check_output([compiler,'-Zls=root',str(item.with_suffix('.rmeta'))],text=True))

driver = generated / 'exchange-final'
subprocess.run([sys.executable,str(ticket/'link-fixture.py'),'bim_ifc='+str(archive),'bim_svg='+str(dependency('semio-s-artifact-bim-model-svg')),'bim_snapshot='+str(dependency('semio-s-artifact-bim-model-snapshot'))],env=dict(os.environ,BIM_FIXTURE_LINK_SOURCE=str(ticket/'exchange-fixtures.rs'),BIM_FIXTURE_LINK_OUTPUT=str(driver)),check=True)
cases = [('svg',notated,'🚪️svg/🪧️notated/🪧️notated.svg'),('2x3',notated,'🏗️ifc/🪧️notated/🪧️notated.ifc'),('4',notated,'🏢️ifc4/🪧️notated/🪧️notated.ifc')]
for asset, directory in [('🏡️house','🏡️example-house'),('🏢️office','🏢️example-office')]:
    cases.append(('4',subset/'🖼️assets'/asset/'📸️snapshot.json','🏢️ifc4/'+directory+'/'+directory+'.ifc'))
for kind, source, target in cases:
    subprocess.run([str(driver),kind,str(source),str(fixtures/target)],check=True)
