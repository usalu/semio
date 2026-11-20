"""🔁 Exercise the production bounded history fold with the captured Play archive in WASI."""
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
work = ticket / "🗑️generated"
workspace = ticket.parents[6]
lane = workspace / ".cache/b/r/m/u/l" / sys.argv[1]
fold = next(workspace.glob("🧰️framework/*modules/*replication/*causal/*transition/*fold/*birth/*original/*rs")).read_text()
drivers = fold[fold.index(" use semio_framework_value_native_owned::native_decoding::{OriginalWorkAuthority", fold.index("mod rejection_laws")):fold.index(" fn policy(", fold.index("mod rejection_laws"))]
source = work / "original_fold.rs"
source.write_text('''
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
''' + drivers + '''
fn main(){
 let spr = include_bytes!("din4108-observed.spr");
 let source = std::sync::Arc::new(semio_framework_async::poll::resolve_ready(semio_os_spr::decode_history(spr,&Default::default())).unwrap());
 let mut runner = Some(move |control| async move {source.fold_controlled(semio_os_spr::HistoryShape::Document,&control).await});
 let stock = RetainedCloneGrant{maximum_items: 1<<24, maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:256};
 let mut physical=Physical{left:stock,submitted:Default::default()};
 let mut accept=|_|true;
 let mut native=NativeDecodeControl::new(stock.maximum_capacity_bytes,&mut accept);
 let mut job=None;
 let turn=receiving(&mut native,&mut physical,&mut Work(1<<24),|native|semio_os_spr::HistoryFoldJob::admit_original(&mut runner,&mut job,native));
 assert!(turn.receipt_refusal.is_none(),"{:?}",turn.receipt_refusal);
 assert!(turn.result.unwrap().unwrap());
 let mut job=job.unwrap();
 for index in 0..100000 {
   if job.original_rejection_ready() {
     let mut error=None;
     let turn=receiving(&mut native,&mut physical,&mut Work(1<<24),|native|job.take_original_rejection(&mut error,native));
     panic!("rejected after {index} turns. {:?} {:?}",turn.receipt_refusal,error);
   }
   if job.original_output_ready(){
     let mut output=None;
     let turn=receiving(&mut native,&mut physical,&mut Work(1<<24),|native|job.take_original_output(&mut output,native));
     assert!(turn.result.unwrap().unwrap() && turn.receipt_refusal.is_none() && job.terminal_is_empty());
     println!("Bounded original history fold completed after {index} turns.");
     return;
   }
   let turn=receiving(&mut native,&mut physical,&mut Work(1<<24),|native|job.step_original(native,4096));
   assert!(turn.receipt_refusal.is_none(),"turn {index} receipt {:?}",turn.receipt_refusal);
   if let Some(Err(error))=turn.result {panic!("turn {index} {error:?}");}
 }
 panic!("fold timed out");
}
''')
args = ["--edition=2021","--target=wasm32-wasip2","-Copt-level=1","-Cpanic=abort","-Cstrip=debuginfo",str(source),"-o",str(work/"original-fold.wasm")]
args += ["-Ldependency="+str(d) for d in lane.glob("**/out") if "/build/" in str(d)]
for crate in ["semio-framework-async","semio-os-spr","semio-framework-value-native-owned","semio-framework-value"]:
    libs=(lane/"wasm32-wasip2/prototype/build"/crate).glob("*/out/*.rlib")
    lib=max(libs,key=lambda p:p.stat().st_mtime)
    for suffix in [".rlib",".rmeta"]:
        args+=["--extern",crate.replace("-","_")+"="+str(lib.with_suffix(suffix))]
flags=work/"original-fold.args"
flags.write_text("\n".join(args)+"\n")
subprocess.run(["rustc","@"+str(flags)],check=True)
subprocess.run(["node_modules/.bin/jco","run","--jco-dir",work/"original-fold-jco",work/"original-fold.wasm"],cwd=workspace,check=True)
