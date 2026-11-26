"""🔏 Reconcile local Cargo path dependencies using the pinned offline resolver."""
from pathlib import Path
import json
import os
import shutil
import subprocess

root=Path.cwd()
generated=Path(__file__).resolve().parent/"🗑️generated"
lock=root/"Cargo.lock"
if not (generated/"root-lock-before.toml").exists():
    shutil.copyfile(lock,generated/"root-lock-before.toml")
registry=json.loads(subprocess.check_output(["repo","tools","list","--json"]))
rustup=next(row["command"] for row in registry if row["id"]=="rustup")
channel=next(line.split('"')[1] for line in Path("rust-toolchain.toml").read_text().splitlines() if line.startswith("channel"))
command=[rustup,"run",channel,"cargo","tree","--offline","--depth","0","-p","semio-os-mcp"]
compiler=subprocess.check_output([rustup,"which","--toolchain",channel,"rustc"],text=True).strip()
subprocess.run(command,check=True,env=dict(os.environ,CARGO_HOME=str(root/".cache/b/r/m/cargo/home"),RUSTC=compiler,RUSTC_WRAPPER="",RUSTC_WORKSPACE_WRAPPER=""))
shutil.copyfile(lock,generated/"root-lock-after.toml")
bun=next(row["command"] for row in registry if row["id"]=="bun")
check='''const [old,next]=await Promise.all(process.argv.slice(1).map(async p=>Bun.TOML.parse(await Bun.file(p).text()).package));
const registry=rows=>rows.filter(p=>p.source).map(p=>[p.name,p.version,p.source,p.checksum]).sort();
if(JSON.stringify(registry(old))!==JSON.stringify(registry(next)))throw Error("Offline resolution changed the third-party graph");
const keyed=rows=>new Map(rows.filter(p=>!p.source).map(p=>[p.name,JSON.stringify(p)]));
const a=keyed(old),b=keyed(next);console.log(JSON.stringify({changed:[...b.keys()].filter(k=>a.get(k)!==b.get(k)),removed:[...a.keys()].filter(k=>!b.has(k)),registryPins:registry(old).length}));'''
subprocess.run([bun,"-e",check,generated/"root-lock-before.toml",generated/"root-lock-after.toml"],check=True)
