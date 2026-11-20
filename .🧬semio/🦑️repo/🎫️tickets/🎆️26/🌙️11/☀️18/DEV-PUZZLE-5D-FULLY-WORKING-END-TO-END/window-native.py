import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib

workspace=Path.cwd()
build=Path(sys.argv[1])
output=Path(sys.argv[2])
output.mkdir(parents=True,exist_ok=True)
rustc,=workspace.glob(".cache/b/t/m/r/*/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc")
crate=workspace/"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone"
manifest=tomllib.loads((crate/"📦️packages/🦀️rust/Cargo.toml").read_text())
package=manifest["package"]["name"]
name=package.replace("-","_")
run_unit=sys.argv[3:]==["--unit"]
run_assembly=sys.argv[3:]==["--assembly"]
probe_name="semio_s_artifact_puzzle_5d_tests"
variant_package="semio-s-artifact-puzzle-5d-tests" if run_unit else package
variant_name=probe_name if run_unit else name
variants=sorted(build.glob(f"{variant_package}/*/out/lib{variant_name}-*.rmeta"),key=lambda path:path.stat().st_mtime_ns,reverse=True)
for variant in variants:
 metadata=subprocess.check_output([rustc,"-Z","ls=root",variant],text=True)
 if not run_assembly or "semio_framework_job_testing_caller" in metadata:break
else:raise RuntimeError("Snapshot clone fixture requires testkit artifacts")
identifiers={identifier.split("-")[0]:identifier for identifier in re.findall(r"^\d+ ([\w]+-[\da-f]+) hash",metadata,re.M)}
artifacts={}
directories=set()
for path in build.glob("*/*/out/lib*"):
 if path.suffix not in (".rmeta",".rlib",".so"):continue
 identifier=path.stem.removeprefix("lib")
 directories.add(path.parent)
 if path.suffix==".rmeta" or identifier not in artifacts:artifacts[identifier]=path
def run(args,data=None):
 for directory in sorted(directories):args.extend(["-L",f"dependency={directory}"])
 result=subprocess.run([rustc,"--edition=2021","-Zthreads=1",*args],input=data,text=True,env={**os.environ,"RUSTC_ICE":str(output)})
 if result.returncode:raise SystemExit(result.returncode)
if run_unit:
 compiled=output/"preparation-test"
 original=(crate/"🦀️.rs").read_text()
 data=f"#[path={json.dumps(str(crate/'🚚️handoff/🦀️.rs'),ensure_ascii=False)}] mod handoff;\n"
 data+="""
use std::mem::size_of;
use semio_framework_value::{ErasedSnapshotRetirement, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, ValueError, retained_clone::{RetainedClone, RetainedCloneCursor}};
use semio_os_store_one_item_publication::native_source::pay;
use semio_framework_value_clone_source_testing::drive::drive;
"""
 data+=original[original.index("fn frame_birth<"):original.index("/// 🚦️ A pending child")]
 data+="""
fn main(){
    type Cursor = <String as RetainedClone>::Cursor;
    let birth = frame_birth::<Cursor>();
    let cursor=Box::new(String::retained_clone_cursor());
    let address=std::ptr::from_ref(cursor.as_ref());
    let mut handoff=handoff::RetainedCloneCursorHandoff::<String>::new(cursor);
    assert_eq!(std::ptr::from_ref(handoff.cursor().unwrap()), address);
    let mut released=0;
    for _ in 0..64 {
        if handoff.terminal_is_empty(){break;}
        let demand=handoff.next_demand(4096).unwrap();
        let work=handoff.next_computational_work(4096).unwrap().max(1);
        let exact=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        if handoff.cursor().unwrap().terminal_is_empty(){
            let denied=RetainedCloneGrant{maximum_release_bytes:exact.maximum_release_bytes-1,..exact};
            let step=drive(denied,work,|effect|handoff.close_step(4096,effect)).result.unwrap();
            assert_eq!(step.progress(),RetainedCloneProgress::default());
            assert_eq!(std::ptr::from_ref(handoff.cursor().unwrap()),address);
        }
        let step=drive(exact,work,|effect|handoff.close_step(4096,effect)).result.unwrap();
        assert!(step.progress().fits(exact));
        released+=step.progress().released_bytes;
    }
    assert!(handoff.terminal_is_empty());
    assert_eq!(released,birth.retained_capacity_bytes);
    println!("snapshot clone handoff preserves one native address and releases {released} bytes under exact grants");
}
"""
 args=["-Clinker-features=+lld","-Clink-self-contained=+linker","-Zunstable-options","--crate-name",name,"-o",str(compiled),"-"]
 for alias in ["semio_framework_value","semio_os_store_one_item_publication"]:
  path=artifacts[identifiers[alias]]
  args.extend(["--extern",f"{alias}={path.with_suffix('.rmeta')}","--extern",f"{alias}={path.with_suffix('.rlib')}"])
 alias="semio_framework_value_clone_source_testing"
 needle=identifiers["semio_framework_value_control_contract"]
 for identifier,path in artifacts.items():
  if identifier.startswith(alias+"-") and needle in subprocess.check_output([rustc,"-Z","ls=root",path.with_suffix(".rmeta")],text=True):
   args.extend(["--extern",f"{alias}={path.with_suffix('.rmeta')}","--extern",f"{alias}={path.with_suffix('.rlib')}"])
   break
 run(args,data)
 raise SystemExit(subprocess.run([compiled]).returncode)
compiled=output/("snapshot-clone-assembly-probe" if run_assembly else f"lib{name}.rmeta")
source=crate/"🦀️.rs"
if run_assembly:
 def absolute(path,root):
  return json.dumps(str((root/path).resolve()),ensure_ascii=False)
 original_effect=(crate/"🧪️effect/🦀️.rs").read_text().replace(", testing::fx","").replace("use std::","use crate::standalone_testing::fx;\nuse std::",1)
 runtime=source.read_text().replace('#[cfg(test)]\n#[path="🧪️effect/🦀️.rs"]\nmod effect_fixture;',f"mod effect_fixture {{ {original_effect[original_effect.index(chr(10))+1:]} }}")
 runtime=re.sub(r'(#\[path\s*=\s*)"([^"]+)"',lambda match:match[1]+absolute(match[2],crate),runtime)
 testing=(workspace/"🧰️framework/🔨️modules/🌱️value/🧪️testing/🧾️wire/🦀️.rs").read_text().replace("crate::","semio_framework_value::")
 test=crate/"🧪️tests/🔬️unit/🦀️.rs"
 test_name="snapshot_publication_original_build_frontiers_preserve_native_pointers_and_paid_cancellation"
 fixture=re.sub(r'(include_str!\()"([^"]+)"',lambda match:match[1]+absolute(match[2],test.parent),test.read_text())
 fixture=fixture.replace(f"#[test]\nfn {test_name}",f"pub(super) fn {test_name}",1)
 fixture=fixture.replace("semio_framework_value::testing::","crate::standalone_testing::").replace("ArtifactStoreOneItemLiveAuthority::mint(","ArtifactStoreOneItemLiveAuthority::from_recovered_outcome(")
 source=output/"snapshot-clone-assembly-probe.rs"
 source.write_text(runtime+f"\nmod standalone_testing {{ {testing[testing.index(chr(10))+1:]} }}\nmod assembly_probe {{ {fixture} }}\nfn main() {{ assembly_probe::{test_name}(); println!(\"snapshot clone assembly retained custody under all seven granted frontiers\"); }}")
args=["--crate-name",name,"-Cmetadata=window-preparation",str(source),"-o",str(compiled)]
if run_assembly:
 args.extend(["-Clinker-features=+lld","-Clink-self-contained=+linker","-Zunstable-options"])
else:
 args.extend(["--crate-type","lib","--emit","metadata"])
dependencies=set(manifest["dependencies"])
if run_assembly:dependencies.update(["serde_json","semio-framework-value-testing-effect-bank"])
for dependency in sorted(dependencies):
 alias=dependency.replace("-","_")
 selected=artifacts.get(identifiers.get(alias))
 if selected is None:selected=next(path for identifier,path in artifacts.items() if identifier.startswith(alias+"-"))
 args.extend(["--extern",f"{alias}={selected}"])
 if run_assembly and selected.with_suffix(".rlib").exists():args.extend(["--extern",f"{alias}={selected.with_suffix('.rlib')}"])
run(args)
if run_assembly:
 raise SystemExit(subprocess.run([compiled]).returncode)
