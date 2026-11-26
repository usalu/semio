"""🦀 Rebuild one native library against its Cargo-produced dependency identities."""
from pathlib import Path
import json
import os
import re
import subprocess
import sys

ticket=Path(__file__).resolve().parent
generated=ticket/'🗑️generated'
original=Path(sys.argv[1]).resolve()
manifest=Path(sys.argv[2]).resolve()
name=original.name.split('-')[0][3:]
compiler=subprocess.check_output(['rustup','which','rustc'],text=True).strip()
deps=generated/'fixture-link'
metadata=subprocess.check_output([compiler,'-Zls=root',str(original.with_suffix('.rmeta'))],text=True)
exports=dict(re.findall(r'^\d+ (\w+)-(\w+) hash ',metadata,re.M))
targets=re.findall(r'^([\w-]+)(?:\.workspace)?\s*=',manifest.read_text().split('[dependencies]')[1].split('\n[')[0],re.M)
out=generated/'refresh'/name
out.mkdir(exist_ok=True,parents=True)
args=[compiler,'--crate-name',name,'--crate-type','rlib','--edition=2021','--emit=metadata,link','-Cmetadata=refreshed','-Cextra-filename=-refreshed','-Zno-parallel-backend','-Zthreads=1','-L','dependency='+str(deps),'--out-dir',str(out)]
for field in targets:
    crate=field.replace('-','_')
    if crate not in exports: continue
    for ext in ('.rmeta','.rlib','.dylib','.so','.dll'):
        p=deps/('lib'+crate+'-'+exports[crate]+ext)
        if p.is_file(): args.extend(['--extern',crate+'='+str(p)])
subprocess.run([*args,str(manifest.parent/'🦀️.rs')],env=dict(os.environ,CARGO_MANIFEST_DIR=str(manifest.parent),CARGO_PKG_NAME=name.replace('_','-'),CARGO_PKG_VERSION='0.1.0'),cwd=manifest.parent,check=True)
print(out/('lib'+name+'-refreshed.rlib'))
