from pathlib import Path
import os,json,subprocess
ticket=Path(__file__).resolve().parent
root=ticket.parents[6]
gen=ticket/'🗑️generated'
manifest=next(root.glob('✏️s/🔌*/🧩*/🗿*/*3d/🏅*/🔖*/🪆*/✳*/✏️*/📦*/🦀*/Cargo.toml')).parent
index=json.loads((gen/'ui-history-frozen/fingerprint-index.json').read_text())
deps=gen/'ui-history-frozen/deps'
fp=next((gen/'builder/ov/u/debug/build/semio-s-artifact-puzzle-3d-editor').glob('*/fingerprint/test-integration-test-editor-panels.json'))
data=json.loads(fp.read_text())
out=gen/'fill-host-probe';out.mkdir(exist_ok=True)
source=ticket/'current-fill-host-probe.rs'
contextfile=manifest/'../../🧪️tests/🧰️context/🦀️.rs'
source_actual=out/'probe.rs'
source_actual.write_text(source.read_text().replace('mod context {\n    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️context/🦀️.rs"));\n}',f'#[path = "{contextfile.resolve()}"] mod context;'))
binary=out/'probe'
args=[str(root/'.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc'),str(source_actual),'--test','-Z','unstable-options','--edition=2021','-C','opt-level=0','-C','debuginfo=0','-Z','threads=1','-C','linker-features=+lld','-C','link-self-contained=+linker','-L',f'dependency={deps}','-o',str(binary)]
for _,alias,_,f in data['deps']:
    path=Path(index['artifacts'][f.to_bytes(8,'little').hex()][1]);args+=['--extern',f'{alias}={path}']
    if path.with_suffix('.rmeta').exists(): args+=['--extern',f'{alias}={path.with_suffix(".rmeta")}']
for n in index['native']:args+=['-L',f'native={n}']
env={**os.environ,'CARGO_MANIFEST_DIR':str(manifest),'CARGO_PKG_NAME':'semio-s-artifact-puzzle-3d-editor','CARGO_PKG_VERSION':'0.1.0','OUT_DIR':str(fp.parent.parent/'out'),'RUST_MIN_STACK':'134217728','ZIG_LOCAL_CACHE_DIR':str(gen/'zig-local'),'ZIG_GLOBAL_CACHE_DIR':str(gen/'zig')}
subprocess.run(args,env=env,cwd=manifest,check=True)
raise SystemExit(subprocess.run([str(binary),'--nocapture','--exact','publish_fill_count_from_original_window'],env=env,cwd=manifest).returncode)
