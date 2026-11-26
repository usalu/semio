"""🏗️ Link the production exchange driver for WASI using the IFC crate's actual dependency graph."""
from pathlib import Path
import json
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
output = generated / 'exchange-wasi'
output.mkdir(exist_ok=True)
capture = max((generated/'compiler-retry').glob('w-ifc-*.success.json'),key=lambda path:path.stat().st_mtime)
request = json.loads(capture.read_text())
compiler = request['compiler']
args = request['arguments']
out = Path(args[args.index('--out-dir')+1])
ifc = next(out.glob('libsemio_s_artifact_bim_model_ifc-*.rlib'))
flags = [compiler,'--crate-name','bim_exchange','--crate-type','bin','--edition=2024','--target','wasm32-wasip2',str(ticket/'exchange-fixtures.rs'),'-o',str(output/'exchange.wasm'),'-Clink-arg=--threads=1','-Clink-arg=--strip-all','-Clink-arg=--max-memory=536870912','-Clink-arg=-zstack-size=8388608']
if (generated/'pdf-cwd').is_dir():
    flags.extend(['-L','dependency='+str(generated/'pdf-cwd')])
metadata = subprocess.check_output([compiler, '-Zls=root', str(ifc.with_suffix('.rmeta'))], text=True)
for i, arg in enumerate(args[:-1]):
    if arg == '-L':
        flags.extend([arg, args[i+1]])
for alias, crate in [('bim_ifc','semio_s_artifact_bim_model_ifc'),('bim_svg','semio_s_artifact_bim_model_svg'),('bim_snapshot','semio_s_artifact_bim_model_snapshot')]:
    if alias == 'bim_ifc':
        archive = ifc
    else:
        match = re.search(r'^\d+ ('+crate+r'-[a-f0-9]+) hash ',metadata,re.M)
        family = out.parents[2]/crate.replace('_','-')
        if match:
            archive = next(family.glob('*/out/lib'+match[1]+'.rlib'))
        else:
            candidates = sorted(family.glob('*/out/lib'+crate+'-*.rlib'),key=lambda item:item.stat().st_mtime,reverse=True)
            snapshot = re.search(r'^\d+ (semio_s_artifact_bim_model_snapshot-\S+ hash \w+)',metadata,re.M)[1]
            archive = next(item for item in candidates if snapshot in subprocess.check_output([compiler,'-Zls=root',str(item.with_suffix('.rmeta'))],text=True))
    if alias == 'bim_svg' and len(sys.argv) > 1:
        archive = Path(sys.argv[1]).resolve()
    flags.extend(['-L','dependency='+str(archive.parent),'--extern',alias+'='+str(archive),'--extern',alias+'='+str(archive.with_suffix('.rmeta'))])
subprocess.run(flags,cwd=request['cwd'],check=True)
