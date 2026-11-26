from pathlib import Path
ticket = Path(__file__).resolve().parent
wrapper = (ticket/'current-2d-native-relink.py').read_text()
source = (ticket/'current-3d-live-graph.py').read_text()
start = source.index('from collections import defaultdict')
end = source.index("if os.environ.get('PUZZLE_REFRESH_3D_EDITOR'):")
resolve = source[start:end].replace('semio-s-artifact-puzzle-3d-editor','semio-s-artifact-puzzle-2d-v1').replace('semio_s_artifact_puzzle_3d_editor','semio_s_artifact_puzzle_2d_v1')
resolve += '''
if os.environ.get('PUZZLE_USE_FRESH_V1'):
    fresh=scratch/os.environ.get('PUZZLE_V1_DEST','live-2d-v1')
    artifacts[rootchecksum]=fresh/'libsemio_s_artifact_puzzle_2d_v1-puzzlecurrent.rlib'
    assert artifacts[rootchecksum].is_file()
    args.extend(['-L',f'dependency={fresh}'])
    retained=scratch/'live-2d-retained'
    if retained.is_dir():
        args.extend(['-L',f'dependency={retained}'])
        for pkg,alias,_,checksum in json.loads(libroot.read_text())['deps']:
            if alias=='semio_s_artifact_puzzle_2d_retained':
                artifacts[checksum.to_bytes(8,'little').hex()]=retained/f'lib{alias}-puzzlecurrent.rlib'

unitname='semio_s_artifact_puzzle_2d_unit_tests'
unitmanifest=next(root.glob("✏️s/🔌*/🧩*/🗿*/◻️2d/🏅*/🔖*/🪆*/✳*/✏*/🧪*/🔬️unit/📦*/🦀*/Cargo.toml")).parent
index=next(i for i,entry in enumerate(dependencies) if entry[1]==unitname)
unitdep=dependencies[index]
unitkey=unitdep[3].to_bytes(8,'little').hex()
unitpath=artifacts[unitkey].resolve()
unitfp=unitpath.parent.parent/'fingerprint'/f'lib-{unitname}.json'
unitmeta=json.loads(unitfp.read_text())
newmap={r[0]:r for r in dependencies}
dest=scratch/os.environ.get('PUZZLE_HISTORY_UNITS','history-units')
dest.mkdir(exist_ok=True)
build=[str(toolchain/'rustc'),str(unitmanifest/'🦀️.rs'),'--edition=2021','--crate-name',unitname,'--crate-type=lib','--emit=link,metadata','-Z','threads=1','-C','debuginfo=0','-C','metadata=historycurrent','-C','extra-filename=-historycurrent','-L',f'dependency={libs}','--out-dir',str(dest)]
if os.environ.get('PUZZLE_USE_FRESH_V1'):
    build.extend(['-L',f'dependency={fresh}','-L',f'dependency={retained}'])
for package,alias,kind,checksum in unitmeta['deps']:
    key=checksum.to_bytes(8,'little').hex()
    if package in newmap:key=newmap[package][3].to_bytes(8,'little').hex()
    elif key not in package_edges[package] and len(package_edges[package])==1:key=next(iter(package_edges[package]))
    link=artifacts[key]
    build.extend(['--extern',f'{alias}={link}'])
    if link.with_suffix('.rmeta').is_file():build.extend(['--extern',f'{alias}={link.with_suffix(".rmeta")}'])
unitlib=dest/f'lib{unitname}-historycurrent.rlib'
if not unitlib.exists() or any(artifacts[r[3].to_bytes(8,'little').hex()].resolve().stat().st_mtime_ns > unitlib.stat().st_mtime_ns for r in dependencies):
    subprocess.run(build,env={**os.environ,'CARGO_MANIFEST_DIR':str(unitmanifest),'CARGO_PKG_NAME':unitname.replace('_','-'),'CARGO_PKG_VERSION':'0.1.0','OUT_DIR':str(unitpath.parent)},check=True)
artifacts[unitkey]=dest/f'lib{unitname}-historycurrent.rlib'
args.extend(['-L',f'dependency={dest}'])
'''
insertion = "source = source.replace('for _, alias, _, checksum in dependencies:'," + repr(resolve) + "+'\\nfor _, alias, _, checksum in dependencies:')\n"
wrapper=wrapper.replace('exec(compile(source',insertion+'exec(compile(source')
exec(compile(wrapper,str(ticket/'current-2d-native-relink.py'),'exec'))
