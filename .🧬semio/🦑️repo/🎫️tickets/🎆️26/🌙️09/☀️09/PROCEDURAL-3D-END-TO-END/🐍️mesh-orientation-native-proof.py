from pathlib import Path
import json
import os
import subprocess

ticket=Path(__file__).resolve().parent
out=ticket/"🗑️generated"/"bevel-native"
mesh=Path("🧰️framework/🔨️modules/🧊️3d/🥽️mesh")
fixture=mesh/"🧪️tests/🧫️fixtures/🛠️modeling/🎟️owners.json"
cache=Path(".cache/b/r/m/u/l")
artifacts={p.stem.removeprefix("lib"):p for lane in cache.glob("*/prototype/build") for p in lane.glob("*/*/out/*") if p.suffix in (".rlib",".dylib")}
engine=next(p for name,p in artifacts.items() if name.startswith("semio_framework_mesh_engine-"))
lineage=dict(json.loads(engine.with_suffix(".rmeta.semio-deps").read_text())["dependencies"])
source=out/"orientation.rs"
source.write_text("use semio_bevel_modeling::*;\nuse semio_framework_value::{AuthoringWallet,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};\nuse serde_json::Value;\n\nfn step(job:&mut MeshModelingJob,grant:RetainedCloneGrant)->RetainedCloneProgress {\n    let mut receipt=RetainedCloneProgress::default();\n    let turn=AuthoringWallet::new(grant,1<<20).run(grant,1<<16,|effect|job.step(1,effect,&mut receipt)).unwrap().unwrap();\n    assert!(matches!(turn.output.unwrap(),MeshModelingStep::Working(_)));\n    receipt\n}\n\n#[test]\nfn orientation_component_copy_receipts_match_the_ownership_contract() {\n    let fixture=std::fs::read_to_string(std::env::var(\"MESH_ORIENTATION_FIXTURE\").unwrap()).unwrap();\n    let law:Value=serde_json::from_str(&fixture).unwrap();\n    let policy=&law[\"orientationComponents\"];\n    let count=|value:&Value|value.as_u64().unwrap() as usize;\n    for (name,reverse) in policy[\"sources\"].as_array().unwrap().iter().map(|name|(name.as_str().unwrap(),false))\n        .chain(std::iter::once((\"snapshotClosedPreparation\",true))) {\n        let source=&law[name];\n        let positions:Vec<[f32;3]>=serde_json::from_value(source[\"positions\"].clone()).unwrap();\n        let mut faces:Vec<Vec<u32>>=serde_json::from_value(source[\"faces\"].clone()).unwrap();\n        if reverse {for face in &mut faces {face.reverse();}}\n        let mut job=HalfedgeMesh::from_faces(&positions,&faces).unwrap().orient_faces_job_owned().unwrap();\n        let mut turns=0;\n        while job.orientation_components_demands().unwrap().is_none() {\n            let demand=job.snapshot_demands().unwrap().or(job.orientation_initialization_demands().unwrap())\n                .or(job.orientation_incidence_demands(84).unwrap()).or(job.orientation_adjacency_demands().unwrap())\n                .unwrap_or(RetirementDemand{depth:3,..Default::default()});\n            step(&mut job,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:84,maximum_capacity_bytes:demand.capacity_bytes,\n                maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth});\n            turns+=1;\n            assert!(turns<10000);\n        }\n        let mut copied=0;\n        let mut quotes=Vec::new();\n        while let Some(demand)=job.orientation_components_demands().unwrap() {\n            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,\n                maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};\n            if demand.copy_bytes>0 {\n                let before=job.progress();\n                let denied=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant};\n                assert_eq!(step(&mut job,denied),Default::default());\n                assert_eq!(job.progress(),before);\n                quotes.push(demand.copy_bytes);\n            }\n            let receipt=step(&mut job,grant);\n            assert_eq!(receipt.copied_bytes,demand.copy_bytes);\n            assert_eq!(receipt.retained_capacity_bytes,demand.capacity_bytes);\n            copied+=receipt.copied_bytes;\n            turns+=1;\n            assert!(turns<10000);\n        }\n        assert_eq!(quotes[0],count(&policy[\"rootCopyBytes\"]));\n        assert!(quotes.iter().skip(1).all(|&bytes|bytes==count(&policy[\"neighborCopyBytes\"])||(reverse&&bytes==1)));\n        assert_eq!(copied,count(&policy[\"outputs\"][name][\"copyBytes\"])+usize::from(reverse));\n        assert_eq!(job.orientation_flip_count(),Some(if reverse {faces.len()}else{count(&policy[\"outputs\"][name][\"flipCount\"])}));\n        println!(\"[DEBUG] {name} reverse={reverse} copied={copied} work_turns={turns}\");\n    }\n}\n")
binary=out/"orientation_proof"
command=["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc",str(source),"--edition","2024","--test","-o",str(binary)]
for path in sorted({p.parent for p in artifacts.values()})+[out]:
    command+=["-L","dependency="+str(path)]
library=out/"libsemio_bevel_modeling.rlib"
for name,library in [("semio_bevel_modeling",library),*((name,artifacts[next(crate for crate in lineage if crate.startswith(name+"-"))]) for name in ("serde_json","semio_framework_value"))]:
    command+=["--extern",name+"="+str(library)]
    if library.with_suffix(".rmeta").exists():
        command+=["--extern",name+"="+str(library.with_suffix(".rmeta"))]
status=subprocess.run(command).returncode
if status:
    raise SystemExit(status)
raise SystemExit(subprocess.run([str(binary),"--nocapture"],env={**os.environ,"MESH_ORIENTATION_FIXTURE":str(fixture)}).returncode)
