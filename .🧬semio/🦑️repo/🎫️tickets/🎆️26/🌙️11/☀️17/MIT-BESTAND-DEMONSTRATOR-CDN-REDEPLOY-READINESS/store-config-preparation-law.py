import os
import re
import subprocess
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated" / "store-source"
lane = Path(sys.argv[1]).resolve()
rustc = Path(sys.argv[2])
store = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store").resolve()
dependency = output / (f"wasi-app-{lane.parents[2].name}" if "--application" in sys.argv[3:] else "wasi-deps")
dependency.mkdir(exist_ok=True)
for layout in (lane, lane.parents[2] / "release/build"):
    for artifact in layout.glob("*/*/out/*"):
        if artifact.suffix not in (".rmeta", ".rlib", ".dylib"):
            continue
        link = dependency / artifact.name
        if link.is_symlink() and not link.exists():
            link.unlink()
        if not link.exists():
            link.symlink_to(artifact)

publication = "semio_os_store_one_item_publication"
snapshot = "semio_os_store_snapshot_clone"

def crate_metadata(name):
    return next(lane.glob(f"{name.replace('_','-')}/*/out/lib{name}-*.rmeta"))

def dependency_names(metadata):
    text = subprocess.check_output([rustc, "-Z", "ls=all", metadata], text=True)
    return dict(re.findall(r"^\d+ (\w+)-([a-f0-9]+) hash ", text, re.M))

def compile(arguments, deps, names, overrides=None, environment=None):
    overrides = overrides or {}
    externs=[]
    for name in names:
        path = overrides.get(name, dependency / f"lib{name}-{deps[name]}.rlib")
        if path.with_suffix(".dylib").exists():
            externs += ["--extern",f"{name}={path.with_suffix('.dylib')}"]
            continue
        if path.with_suffix(".rmeta").exists():
            externs += ["--extern", f"{name}={path.with_suffix('.rmeta')}"]
            if "--metadata-only" in sys.argv[3:]:
                continue
        externs += ["--extern", f"{name}={path}"]
    subprocess.run([rustc,"--edition=2021","--target=wasm32-wasip2","-Cdebuginfo=0","-L",f"dependency={dependency}",*externs,*arguments],env=environment,check=True)

paths = {}
if "--application" in sys.argv[3:]:
    index = sys.argv.index("--application")
    runner = Path(sys.argv[index+1]).resolve()
    package_names = sys.argv[index+2:]
    origins = [crate_metadata(name) for name in package_names]
    deps = dict(pair for origin in origins for pair in dependency_names(origin).items())
    deps.update((name, origin.stem.rsplit("-",1)[1]) for origin, name in zip(origins,package_names))
    mode = ["--emit=metadata","-o",output/"store-config-application.rmeta"] if "--metadata-only" in sys.argv[3:] else ["-Copt-level=1","-Clink-arg=--threads=1","-o",output/"store-config-application.wasm"]
    source_cfg=[] if "--port-audit" in sys.argv[3:] else ["--cfg=generator_production"]
    compile(["--crate-name","store_config_application_law",*source_cfg,*mode,runner],deps,package_names)
    sys.exit(0)

if "--batch" in sys.argv[3:]:
    editor_config = "semio_s_artifact_procedural_3d_editor_config"
    engine = "semio_os_store_engine"
    config = "semio_os_store_config"
    names = ["semio_framework_artifact_flow_flow","semio_plugin_retained_command","semio_framework_job","semio_framework_value","semio_os_spr","semio_os_store_command",editor_config,engine,config,"semio_os_store_engine_owners","semio_os_store_initialization_runtime","semio_os_store_one_item",publication,snapshot]
    origins = [crate_metadata(name) for name in (engine, config, "semio_os_kernel", "semio_plugin", "semio_framework_artifact_flow_flow")]
    deps = dict(pair for origin in origins for pair in dependency_names(origin).items())
    editor = list(lane.glob(f"{editor_config.replace('_','-')}/*/out/lib{editor_config}-*.rmeta"))
    if editor:
        deps.update(dependency_names(editor[0]))
    else:
        manifest = Path(sys.argv[sys.argv.index("--config-manifest")+1]).resolve()
        entries = manifest.read_text().split("[dependencies]",1)[1].split("\n[",1)[0]
        required = [entry.replace("-","_") for entry in re.findall(r"^([a-z0-9-]+)\s*=",entries,re.M)]
        deps.update((name,crate_metadata(name).stem.rsplit("-",1)[1]) for name in required if name not in deps)
        target=output/f"lib{editor_config}.rlib"
        compile(["--crate-name",editor_config,"--crate-type","rlib","-Copt-level=1","-o",target,manifest.parent/"../../🦀️.rs"],deps,required,environment={**os.environ,"CARGO_MANIFEST_DIR":str(manifest.parent),"CARGO_PKG_NAME":editor_config.replace("_","-"),"CARGO_PKG_VERSION":"0.1.0"})
        paths[editor_config]=target
        deps[editor_config]=""
    deps.update((name,crate_metadata(name).stem.rsplit("-",1)[1]) for name in names if name not in deps)
    compile(["--crate-name","store_config_batch_law","-Copt-level=1","-o",output/"store-config-batch.wasm",ticket/"store-config-batch-law.rs"],deps,names,paths)
    sys.exit(0)

if "--green" in sys.argv[3:]:
    package_roots = [(publication, store / "🎮️one-item/📬️publication"), (snapshot, store / "🧬️snapshot-clone")]
    for name, entry in package_roots:
        metadata=crate_metadata(name)
        manifest=entry / "📦️packages/🦀️rust"
        section=(manifest/"Cargo.toml").read_text().split("[dependencies]",1)[1].split("\n[",1)[0]
        declared=re.findall(r"^([a-z0-9-]+)\s*=",section,re.M)
        aliases={"semio-framework-replication":"protocol"}
        names=[aliases.get(c,c.replace("-","_")) for c in declared]
        deps=dependency_names(metadata)
        target=output/f"lib{name}.rlib"
        compile(["--crate-name",name,"--crate-type","rlib","-Copt-level=1","-o",target,entry/"🦀️.rs"],deps,names,paths,{**os.environ,"CARGO_MANIFEST_DIR":str(manifest),"CARGO_PKG_NAME":name.replace("_","-"),"CARGO_PKG_VERSION":"0.1.0"})
        paths[name]=target
for name in (publication,snapshot):
    paths.setdefault(name,crate_metadata(name).with_suffix(".rlib"))
deps=dependency_names(crate_metadata(snapshot))
deps[snapshot]=crate_metadata(snapshot).stem.rsplit("-",1)[1]
names=[publication,snapshot,"semio_framework_job","semio_framework_value","semio_framework_pack_json","semio_framework_value_snapshot_read","semio_os_spr","semio_os_store_command","semio_os_store_one_item"]
compile(["--crate-name","store_config_preparation_law","-o",output/"store-config-preparation.wasm",ticket/"store-config-preparation-law.rs"],deps,names,paths)
