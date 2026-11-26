"""Recheck changed native history providers in the previously compiled dependency order."""
from pathlib import Path
import os,subprocess
p=Path(__file__).resolve().parent
g=p/'🗑️generated'
o=g/'overlay-history25'
packages=[l.split()[1] for l in (g/'overlay-history25-build.log').read_text().splitlines() if l.startswith('Compiling ')]
manifests={}
for f in next(Path.cwd().glob('🧰️framework/*products/*os/*modules/*plugin')).rglob('Cargo.toml'):
    import re
    name=re.search(r'^name = "(.*?)"',f.read_text(),re.M)
    if name:manifests[name[1]]=f.parent
changed=set()
env=dict(os.environ,RUST_MIN_STACK='134217728',SEMIO_BUILD_MODE='prototype',CARGO_PKG_VERSION='0.1.0')
def build(args,directory,name):
    subprocess.run(['repo','tools','exec','--light','rustup','--','run','nightly-2026-07-20','rustc','@'+str(args)],env=dict(env,CARGO_MANIFEST_DIR=str(directory),CARGO_PKG_NAME=name),check=True)
for pkg in packages:
    crate=pkg.replace('-','_')
    args=o/(crate+'.args')
    lines=args.read_text().splitlines()
    if crate!='semio_plugin_instance_dispatch' and not any(line.partition('=')[2] in changed for line in lines):continue
    print(pkg,flush=True)
    build(args,manifests[pkg],pkg)
    changed.add(str(o/('lib'+crate+'.rlib')))
manifest=next(Path.cwd().glob('✏️s/*plugins/*draw/*artifacts/*drawing/*standards/*1/*subsets/*any/*editor/*tests/*companion/*packages/*rust'))
build(g/'history-editor25.args',manifest,'semio-s-artifact-draw-drawing-editor-tests')
subprocess.run([g/'history-editor25','registered_clipboard_copy_paste_cut_preserve_atomic_inverse_and_live_selection','--nocapture'],env=dict(env,CARGO_MANIFEST_DIR=str(manifest)),check=True)
