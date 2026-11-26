from pathlib import Path
base=Path(__file__).resolve().with_name('current-3d-fill-paid.py')
s=base.read_text()
m="exec(compile(code, str(base), 'exec'))"
s=s.replace(m,'''insert = """
from collections import defaultdict
package_edges = defaultdict(dict)
seen = set()
libroot = max((cache/'semio-s-artifact-puzzle-3d-editor').glob('*/fingerprint/lib-semio_s_artifact_puzzle_3d_editor.json'),key=lambda p:p.stat().st_mtime_ns)
if os.environ.get("PUZZLE_GRAPH_HASH"):libroot=libroot.parents[2]/os.environ["PUZZLE_GRAPH_HASH"]/"fingerprint"/libroot.name
def walk(fp):
    if fp in seen:return
    seen.add(fp)
    for package,alias,_,checksum in json.loads(fp.read_text())['deps']:
        key=checksum.to_bytes(8,'little').hex()
        if not key.strip('0') or key not in artifacts:continue
        lib=artifacts[key];real=lib.resolve()
        package_edges[package][key]=lib
        name=real.name.split('-')[0][3:]
        dep=real.parent.parent/'fingerprint'/f'lib-{name}.json'
        if dep.exists():walk(dep)
walk(libroot)
rootchecksum=(libroot.with_suffix('')).read_text().strip() or libroot.parents[1].name
artifacts[rootchecksum]=next((libroot.parents[1]/'out').glob('libsemio_s_artifact_puzzle_3d_editor-*.rlib'))
newdeps=[]
for package,alias,kind,checksum in dependencies:
    previous=checksum.to_bytes(8,'little').hex()
    options=package_edges[package]
    if alias=='semio_s_artifact_puzzle_3d_editor':key=rootchecksum
    elif previous in options:key=previous
    elif len(options)==1:key=next(iter(options))
    elif previous in artifacts and not options:key=previous
    else:
        keys=options or {key:path for key,path in artifacts.items() if key and path.name.split('-')[0]=='lib'+alias}
        if not keys:
            candidates=list(libs.glob('lib'+alias+'-*.rlib'))
            if len(candidates)==1:
                artifacts[previous]=candidates[0]
                keys={previous:candidates[0]}
        assert keys,alias
        key=max(keys,key=lambda k:keys[k].resolve().stat().st_mtime_ns)
    newdeps.append([package,alias,kind,int.from_bytes(bytes.fromhex(key),'little')])
dependencies=newdeps
if os.environ.get('PUZZLE_REFRESH_3D_EDITOR'):
    alias='semio_s_artifact_puzzle_3d_editor'
    dest=scratch/'live-editor'
    dest.mkdir(exist_ok=True)
    actual=artifacts[rootchecksum].resolve()
    libmanifest=next(root.glob("✏️s/🔌*/🧩*/🗿*/🧊*/🏅*/🔖*/🪆*/✳*/✏*/📦*/🦀*/Cargo.toml")).parent
    import tomllib
    sourcepath=libmanifest/tomllib.loads((libmanifest/'Cargo.toml').read_text())['lib']['path']
    build=[str(toolchain/'rustc'),str(sourcepath),'--edition=2021','--crate-name',alias,'--crate-type=lib','--emit=link,metadata','-Z','threads=1','-C','debuginfo=0','-C','metadata=puzzlecurrent','-C','extra-filename=-puzzlecurrent','-L',f'dependency={libs}','--out-dir',str(dest)]
    libmeta=json.loads(libroot.read_text())
    for feature in json.loads(libmeta['features']):
        build.extend(['--cfg',f'feature="{feature}"'])
    for _,dep,_,h in libmeta['deps']:
        link=artifacts[h.to_bytes(8,'little').hex()]
        build.extend(['--extern',f'{dep}={link}'])
        if link.with_suffix('.rmeta').is_file():
            build.extend(['--extern',f'{dep}={link.with_suffix(".rmeta")}'])
    build_env={**os.environ,'CARGO_MANIFEST_DIR':str(libmanifest),'CARGO_PKG_VERSION':'0.1.0','CARGO_PKG_NAME':alias.replace('_','-'),'OUT_DIR':str(actual.parent)}
    subprocess.run(build,env=build_env,check=True)
    artifacts[rootchecksum]=dest/f'lib{alias}-puzzlecurrent.rlib'
    args.extend(['-L',f'dependency={dest}'])
"""
code=code.replace("for _, alias, _, checksum in dependencies:",insert+"\\nfor _, alias, _, checksum in dependencies:")
code=code.replace('original-fill-host','live-fill-graph')
exec(compile(code, str(base), 'exec'))''')
exec(compile(s,str(base),'exec'))
