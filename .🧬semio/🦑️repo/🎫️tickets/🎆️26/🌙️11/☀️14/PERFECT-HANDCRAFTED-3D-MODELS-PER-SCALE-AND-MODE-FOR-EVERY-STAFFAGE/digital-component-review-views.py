"""🔍 Reopen the cut edge, rear supports and glazing in shaded presentation and surface temperature."""
from pathlib import Path
from runpy import run_path
import json,shutil,subprocess,sys
T=Path(__file__).resolve().parent
G=T/'🗑️generated';catalog=run_path(str(T/'digital-building-equipment.py'))['catalog']
for name in sys.argv[1:]:
    file=catalog(name);a=json.loads(file.read_bytes())['asset']
    with (G/('component-'+name+'-reverse.log')).open('w') as out:
        status=subprocess.run([shutil.which('bun'),'--smol',T/'living-visual.ts',name,'all',a['presentation']['defaultPose'],file,'--temperature','--opposite','--presentation','--shaded','--short-name'],stdout=out,stderr=out).returncode
    print('[DEBUG]',name,status,flush=True)
    assert status==0
