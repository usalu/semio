from pathlib import Path
import sys,os
ticket=Path(__file__).resolve().parent
p=ticket/'current-2d-live-graph.py'
s=p.read_text()
a=s.index("if os.environ.get('PUZZLE_USE_FRESH_V1'):")
b=s.index("'''\ninsertion",a)
body="""
import tomllib, functools
dest=scratch/'fill-paged-closure'
dest.mkdir(exist_ok=True)
args.extend(['-L',f'dependency={dest}'])
@functools.cache
def rebuild(key, root_unit=False):
    baseline=artifacts[key].resolve()
    alias=baseline.name.split('-')[0][3:]
    fp=baseline.parents[1]/'fingerprint'/f'lib-{alias}.json'
    meta_local=json.loads(fp.read_text())
    children={}
    dirty=alias=='semio_s_artifact_puzzle_2d_tests'
    if 'puzzle_2d' not in alias:return baseline,False
    for package,k,_,h in meta_local['deps']:
        child=h.to_bytes(8,'little').hex()
        if root_unit and package in by_package:child=by_package[package]
        elif child not in artifacts and len(package_edges[package])==1:child=next(iter(package_edges[package]))
        target,rebuilt=rebuild(child)
        children[k]=target;dirty|=rebuilt
    if not dirty:return baseline,False
    pkg=baseline.parents[2].name
    result=dest/f'lib{alias}-paged{baseline.parents[1].name}.rlib'
    if result.exists() and '--reuse' in sys.argv:return result,True
    manifest_paths=subprocess.check_output(['rg','-l','-g','Cargo.toml',f'^name = "{pkg}"','✏️s'],cwd=root,text=True).splitlines()
    assert len(manifest_paths)==1,(pkg,manifest_paths)
    manifest_file=root/manifest_paths[0]
    src=manifest_file.parent/tomllib.loads(manifest_file.read_text())['lib']['path']
    flags=[str(toolchain/'rustc'),str(src),'--edition=2021','--crate-name',alias,'--crate-type=rlib','-Z','threads=1','-C','debuginfo=0','-C',f'metadata=paged{baseline.parents[1].name}','-L',f'dependency={dest}','-L',f'dependency={libs}','-o',str(result)]
    for k,v in children.items():
        flags+=['--extern',f'{k}={v}']
        if v.with_suffix('.rmeta').is_file():flags+=['--extern',f'{k}={v.with_suffix(".rmeta")}']
    for f in json.loads(meta_local['features']):flags+=['--cfg',f'feature="{f}"']
    print('[DEBUG] Building',pkg,flush=True)
    subprocess.run(flags,check=True,env={**os.environ,'CARGO_MANIFEST_DIR':str(manifest_file.parent),'CARGO_PKG_NAME':pkg,'CARGO_PKG_VERSION':'0.1.0','OUT_DIR':str(baseline.parent)})
    return result,True
by_package={r[0]:r[3].to_bytes(8,'little').hex() for r in dependencies}
for package,alias,kind,h in dependencies:
    key=h.to_bytes(8,'little').hex()
    target,dirty=rebuild(key,alias=='semio_s_artifact_puzzle_2d_unit_tests')
    if dirty:overrides[key]=target
artifacts.update(overrides)
if '--reuse' in sys.argv:sys.argv.remove('--reuse')
"""
s=s[:a]+"overrides={}\n"+body+s[b:]
exec(compile(s,str(p),'exec'))
