"""🧱 Compile an updated exchange library against a captured, coherent native fixture graph."""
from pathlib import Path
import json
import os
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
manifest = Path(sys.argv[1]).resolve()
source = manifest.parent / '🦀️.rs'
declarations = manifest.read_text()
package = re.search(r'^name = "([^"]+)"', declarations, re.M)[1]
name = package.replace('-', '_')
deps = generated / 'fixture-link'
output = generated / 'fixture-library'
output.mkdir(exist_ok=True)
args = [subprocess.check_output(['rustup','which','rustc'],text=True).strip(),'--crate-name',name,'--crate-type','rlib','--edition=2021',str(source),'--emit=metadata,link','-Cmetadata=bim_fixture','--out-dir',str(output),'-L','dependency='+str(deps),'-L','native='+str(deps)]
externs = {}
for record in sorted((generated/'compiler-retry').glob('n-*.invocation.json'),key=lambda path:path.stat().st_mtime,reverse=True):
    original = json.loads(record.read_text())['arguments']
    for index, item in enumerate(original[:-1]):
        if item == '--extern' and '=' in original[index+1]:
            alias, path = original[index+1].split('=',1)
            if Path(path).exists() and path.endswith(('.rlib','.so','.dylib','.dll')):
                externs.setdefault(alias,Path(path))
for line in declarations.split('[dependencies]')[1].split('\n[')[0].splitlines():
    field = re.match(r'^([\w-]+).*?=',line)
    if not field: continue
    alias = field[1].replace('-','_')
    if alias in externs: archive=externs[alias]
    else:
        files = [p for ext in ('.rlib','.dylib','.so','.dll') for p in deps.glob('lib'+alias+'-*'+ext) if p.is_file()]
        if not files: continue
        archive=max(files,key=lambda p:p.stat().st_mtime)
    args.extend(['--extern',alias+'='+str(archive)])
    if archive.suffix=='.rlib' and archive.with_suffix('.rmeta').exists():
        args.extend(['--extern',alias+'='+str(archive.with_suffix('.rmeta'))])
subprocess.run(args,check=True,env=dict(os.environ,CARGO_MANIFEST_DIR=str(manifest.parent),CARGO_PKG_NAME=package,CARGO_PKG_VERSION='0.1.0'))
