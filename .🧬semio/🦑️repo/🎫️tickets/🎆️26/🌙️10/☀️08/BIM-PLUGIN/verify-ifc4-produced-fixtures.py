"""🏗️ Reproduce each committed IFC4 exchange file and audit its quantities and meshes."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import runpy, subprocess
ticket = Path(__file__).resolve().parent
test = Path('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests')
fixtures = test / '🧫️fixtures'
oracle = runpy.run_path(str(test / '🏢️export-bim-1-ifc4/🐍️.py'))
def reproduce(case):
    child = oracle['child']
    source = fixtures if case in oracle['CASES'] else fixtures.parent.parent
    for name in oracle['CASES'].get(case) or oracle['EXAMPLES'][case]:
        source = child(source,name)
    if source.is_dir(): source = child(source,'.json')
    dest = child(oracle['case_folder'](fixtures,case),case + '.ifc')
    subprocess.run([ticket/'🗑️generated/ifc-closed-shells-n/export','4',source,dest],check=True)
with ThreadPoolExecutor(max_workers=2) as pool:
    list(pool.map(reproduce,[*oracle['CASES'],*oracle['EXAMPLES']]))
for cmd in ['write','check']:
    if oracle['main']([cmd,fixtures]): raise SystemExit(1)
