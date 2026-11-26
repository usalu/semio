"""🧊️ Compare each Rust report read from physical IFC4 with IfcOpenShell's committed measurements."""
import json,math,runpy,subprocess
from pathlib import Path
ticket=Path(__file__).resolve().parent
test=Path('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests')
fixture=test/'🧫️fixtures'
oracle=runpy.run_path(str(test/'🏢️export-bim-1-ifc4/🐍️.py'))
for case in [*oracle['CASES'],*oracle['EXAMPLES']]:
    child=oracle['child']
    source=fixture if case in oracle['CASES'] else fixture.parent.parent
    for name in oracle['CASES'].get(case) or oracle['EXAMPLES'][case]:source=child(source,name)
    if source.is_dir():source=child(source,'.json')
    folder=oracle['case_folder'](fixture,case)
    model=child(folder,case+'.ifc')
    expected=json.loads((folder/oracle['MEASURE_FOLDER']/oracle['JSON_FILE']).read_text())
    measured=json.loads(subprocess.check_output([ticket/'🗑️generated/ifc-closed-shells-n/export','report4',source,model]))
    volumes=measured.pop('volumes')
    reference=expected.pop('volumes')
    assert expected==measured,(case,[k for k in expected if expected[k]!=measured.get(k)])
    assert reference.keys()==volumes.keys(),(case,reference.keys()^volumes.keys())
    for key,value in reference.items():
        assert math.isclose(value,volumes[key],rel_tol=1e-7,abs_tol=1e-7),(case,key,value,volumes[key])
    print(case,len(volumes),'measured volumes and all metadata agree',flush=True)
