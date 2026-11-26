"""🏗️ Export the current IFC fixtures and validate schema, volumes and topology with IfcOpenShell."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess, sys
ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
test = Path('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests')
fixture = test / '🧫️fixtures'
ramps = fixture / '💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json'
equipment = fixture / '🏗️ifc/🪑️components/📸️snapshot/🔣️.json'
def run(args):
    subprocess.run(args, check=True)
def export(case):
    schema,source,dest = case
    run([generated / 'ifc-closed-shells-n/export',schema,source,dest])
def validate(job):
    if job == 'components4':
        run([sys.executable,ticket / 'r4-x-ifc-validate.py',generated / 'ifc4-component-check.ifc','express'])
    elif job == '4':
        for cmd in ('write','check'):
            run([sys.executable,test / '🏢️export-bim-1-ifc4/🐍️.py',cmd,fixture,'ramps'])
    else:
        for cmd in ('write','check'):
            run([sys.executable,test / '🏗️export-bim-1-ifc/🐍️.py',cmd,fixture / '🏗️ifc','🛝️ramps','🪑️components'])
with ThreadPoolExecutor(max_workers=2) as pool:
    list(pool.map(export,[('2x3',ramps,fixture/'🏗️ifc/🛝️ramps/🛝️ramps.ifc'),('4',ramps,fixture/'🏢️ifc4/🛝️ramps/🛝️ramps.ifc'),('2x3',equipment,fixture/'🏗️ifc/🪑️components/🪑️components.ifc'),('4',equipment,generated/'ifc4-component-check.ifc')]))
    list(pool.map(validate,['2x3','4','components4']))
