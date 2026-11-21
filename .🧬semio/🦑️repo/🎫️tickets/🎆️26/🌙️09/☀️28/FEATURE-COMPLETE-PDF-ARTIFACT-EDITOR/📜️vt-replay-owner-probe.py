"""🦀️ Typechecks the staged owner hooks against the governed WASI crate metadata."""

from pathlib import Path
import subprocess

TICKET=Path(__file__).resolve().parent
OUT=TICKET/"🗑️generated"/"vt-replay"
cache=Path(".cache/b/r/m/u/l/pb57a72/wasm32-wasip2/prototype/build")
files=sorted(cache.glob("*/*/out/*.rmeta"))
lanes=cache.parents[3]
wasm_paths={path.parent for pattern in ("*/wasm32-wasip2/prototype/build/*/*/out/*.rmeta","*/wasm32-wasip2/prototype/build/*/*/out/*.rlib") for path in lanes.glob(pattern)}
host_paths={path.parent for pattern in ("*/prototype/build/*/*/out/*.dylib","*/prototype/build/*/*/out/*.so") for path in lanes.glob(pattern)}
externs=[]
for name in ("semio_plugin","semio_s_artifact_stdio_pdf_viewer"):
    found=[path for path in files if path.name.startswith(f"lib{name}-")]
    assert len(found)==1, (name,found)
    original=OUT/"deps"/found[0].name
    externs+=["--extern",f"{name}={original if original.is_file() else found[0]}"]

stage=(OUT/"vt-editor.rs").read_text(encoding="utf8")
factory=next(line.strip() for line in stage.splitlines() if line.strip().startswith("document_store_owners:"))
source="use semio_plugin::ArtifactEditor;\nuse semio_s_artifact_stdio_pdf_viewer::editor::pdf17vt::Pdf17VtEditor;\ntype PdfSnapshot = <Pdf17VtEditor as ArtifactEditor>::Snapshot;\ntype PdfMutation = <Pdf17VtEditor as ArtifactEditor>::Mutation;\nmacro_rules! owner {\n    (document_store_owners: $factory:path,) => {\n        pub fn admits(grant: semio_plugin::__value::retained_clone::RetainedCloneGrant) {\n            let _exact_owner = $factory(grant);\n        }\n    };\n}\nowner!(@FACTORY@);\npub fn quote() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {\n    semio_plugin::cold_replay_document_store_owners_source_demands::<PdfSnapshot, PdfMutation>()\n}\n"
path=OUT/"probe.rs"
path.write_text(source.replace("@FACTORY@",factory),encoding="utf8")
args=OUT/"rustc.args"
search=[f"-Ldependency={dir}" for dir in sorted(wasm_paths | host_paths | {OUT / "deps"})]
args.write_text("\n".join(["--edition=2021","--target=wasm32-wasip2","--crate-type=lib","-Cpanic=abort","--emit=metadata","-o",str(OUT/"probe.rmeta"),str(path),*externs,*search])+"\n",encoding="utf8")
if "--audit" in __import__("sys").argv:
    import re
    from collections import Counter
    search_paths=wasm_paths | host_paths
    available={re.sub(r"^lib|\.(rmeta|rlib|dylib|so)$","",file.name) for loc in search_paths for file in loc.iterdir()}
    for item in externs[1::2]:
        root=Path(item.split("=",1)[1])
        data=subprocess.run(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","-Zls=all",str(root)],check=True,capture_output=True,text=True)
        names={row[1] for row in re.finditer(r"^\d+ (\S+) hash ",data.stdout,re.MULTILINE)}
        print(f"[DEBUG] Missing transitive metadata for {root.name}:", sorted(names-available))
    raise SystemExit()
subprocess.run(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","@"+str(args)],check=True)
print("[DEBUG] Staged VT owner quote and catalog typechecked with the actual PDF snapshot and mutation types")
