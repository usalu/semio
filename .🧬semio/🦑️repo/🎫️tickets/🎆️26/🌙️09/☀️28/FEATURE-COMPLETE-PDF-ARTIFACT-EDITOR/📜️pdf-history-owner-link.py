"""🔬 Reuse one governed compiler cohort to check the exact PDF history source overlay."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import unquote

ROOT=Path.cwd()
TICKET=Path(__file__).resolve().parent
LANE=Path(sys.argv[1]).resolve()
OUT=TICKET/"🗑️generated"/("pdf-history-owner-"+LANE.name)
OUT.mkdir(parents=True, exist_ok=True)
UNITS=Path(sys.argv[2])
CRATES=sys.argv[3:]
REMAP={}
POST={}
for argument in (os.environ.get("SEMIO_PDF_TYPE_OVERLAY", "pdf-history-target-overlay.json"),):
    evidence = json.loads((TICKET/"🗑️generated"/argument).read_text())
    records = evidence["data"] if isinstance(evidence,dict) else evidence
for file,before,source in records:
    path=(ROOT/file).resolve()
    if before is not None:
        original=path.read_bytes()
        assert hashlib.sha256(original).hexdigest() in (before,hashlib.sha256(source.encode()).hexdigest()), path
    POST[path]=source
graph=json.loads(UNITS.read_text())["units"]
direct_syntax=re.compile(r'(#\[path\s*=\s*|include!\(\s*)"(.*?)"')
embedded=re.compile(r'(include_bytes!\(\s*|include_str!\(\s*)"(.*?)"')
metadata=OUT/"metadata"
metadata.mkdir(exist_ok=True)
physical={}
for kind,family in ((None,LANE/"prototype/build"),("wasm32-wasip2",LANE/"wasm32-wasip2/prototype/build")):
    for directory in family.glob("*/*/out"):
        name=directory.parent.parent.name
        for file in directory.iterdir():
            if file.suffix not in (".rmeta",".so",".dll",".dylib"):
                continue
            selected=metadata/file.name
            if not selected.exists():
                try: selected.symlink_to(file)
                except FileExistsError: pass
            elif selected.resolve()!=file.resolve():
                assert selected.read_bytes()==file.read_bytes(),selected
            physical.setdefault((kind,name),[]).append(file)

def place(path,nesting=None):
    """🧭 Point unchanged modules directly at the actual worktree and stage only its affected ancestors."""
    path=path.resolve()
    if path in REMAP:
        return REMAP[path]
    if not path.is_file() and path not in POST:
        return path
    before=POST.get(path,path.read_text() if path.is_file() else "")
    affected=path in POST
    visiting=frozenset() if nesting is None else nesting
    if path in visiting:
        return path
    next_visiting=visiting|{path}
    def route(match):
        nonlocal affected
        selected=(path.parent/match[2]).resolve()
        responsible=place(selected,next_visiting)
        affected |= selected!=responsible
        return match[1]+json.dumps(responsible.as_posix(),ensure_ascii=False)
    body=direct_syntax.sub(route,before)
    if not affected:
        return path
    body=embedded.sub(lambda m:m[1]+json.dumps((path.parent/m[2]).resolve().as_posix(),ensure_ascii=False),body)
    safe=hashlib.sha256(str(path).encode()).hexdigest()[:12]
    staged=OUT/(safe+".rs")
    staged.write_text(body)
    REMAP[path]=staged
    return staged

for name in CRATES:
    unit=next(u for u in graph if u["target"]["name"]==name and u["platform"]=="wasm32-wasip2" and u["mode"]=="build")
    target=unit["target"]
    manifest=Path(unquote(unit["pkg_id"].split("#")[0].removeprefix("path+file://")))
    original_root=Path(target["src_path"])
    source=place(original_root)
    assert source.is_relative_to(OUT),name
    flags=["--target=wasm32-wasip2","--crate-type","rlib","--crate-name",name,"--edition",target["edition"],"--emit","metadata",str(source),"-o",str(OUT/("lib"+name+".rmeta")),"-Ldependency="+str(metadata)]
    for feature in unit["features"]:
        flags.extend(("--cfg",'feature="'+feature+'"'))
    for edge in unit["dependencies"]:
        neighbor=graph[edge["index"]]
        package=neighbor["pkg_id"].split("#")[-1].split("@")[0]
        if package[0].isdigit():package=neighbor["pkg_id"].split("#")[0].rsplit("/",1)[-1].split("@")[0]
        direct=neighbor["target"]["name"]
        candidates=physical.get((neighbor["platform"],package),[])
        extensions=(".dylib",".so",".dll") if neighbor["platform"] is None else (".rmeta",)
        chosen=[path for path in candidates if path.suffix in extensions and path.name.startswith("lib"+direct+"-")]
        assert len(chosen)==1,(name,package,direct,chosen)
        if direct in CRATES[:CRATES.index(name)]:
            selected=OUT/("lib"+direct+".rmeta")
        else:
            selected=chosen[0]
        flags.extend(("--extern",edge["extern_crate_name"]+"="+str(selected)))
    reply=OUT/(name+".rsp")
    reply.write_text("\n".join(flags)+"\n")
    env={**os.environ,"CARGO_MANIFEST_DIR":str(manifest),"CARGO_PKG_VERSION":"0.1.0"}
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+str(reply)],env=env,check=True)
    print("[DEBUG] Exact staged PDF history crate typechecked for WASI",name,flush=True)
