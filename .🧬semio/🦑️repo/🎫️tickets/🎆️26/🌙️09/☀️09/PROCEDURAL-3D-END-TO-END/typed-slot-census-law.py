"""🧮 Compare the production typed-slot census with SQLite distinct ownership."""
import os
import sqlite3
import subprocess
import sys
from pathlib import Path
out = Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=True)
source = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🧾️typed-operations/🦀️.rs").read_text()
a=source.index("    pub fn typed_operation_slot_is_vacant")
b=source.index("    /// 🎫️",a)
methods=source[a:b].replace("PendingReserved<A>", "PendingReserved").replace("ToolOperations<A>", "ToolOperations")
prefix="""
use std::cell::Cell;
const ARTIFACT_LIVE_OUTPUT_SLOTS:usize=64;
thread_local!{static READS:Cell<usize>=const{Cell::new(0)};}
struct Table{occupied:u64,allocation_admitted:bool}
impl Table{
 fn slot_is_vacant(&self,i:usize)->bool{READS.with(|r|r.set(r.get()+1));self.allocation_admitted && self.occupied&(1<<i)==0}
}
struct Id(u64);struct Op{operation:Id}struct Permit{operation:Op}struct Commit{permit:Permit}
struct PendingReserved{pending_reserved:Table,reserved_commits:Vec<Commit>}
struct ToolOperations{tool_operations:Table,latest_wins_commands:Table,typed_operation_reservations:[Option<u64>;64]}
struct Media{segmented_downloads:Table,segmented_closures:Table}
impl Media{
"""
main="""
}
fn main(){
 for n in 0..90u64{
  let table=|k:u64|Table{occupied:if n<5{0}else{(0x12412891u64*n).rotate_left((k*11)as u32)},allocation_admitted:n!=k};
  let tools=ToolOperations{tool_operations:table(0),latest_wins_commands:table(1),typed_operation_reservations:std::array::from_fn(|i|(n>7 && (i as u64+n)%7==0).then_some(192+i as u64))};
  let pending=PendingReserved{pending_reserved:table(2),reserved_commits:(0..(n%9)).map(|i|Commit{permit:Permit{operation:Op{operation:Id(64+(n+i)%64)}}}).collect()};
  let media=Media{segmented_downloads:table(3),segmented_closures:table(4)};
  READS.with(|r|r.set(0));let count=media.live_typed_operation_slots(&pending,&tools);
  println!("{} {} {}",n,count,READS.with(Cell::get));
 }
}
"""
rs=out/"census.rs"
rs.write_text(prefix+methods+main)
exe=out/("census.exe" if os.name=="nt" else "census")
subprocess.run([*sys.argv[2:], os.environ.get("RUSTC","rustc"), "--edition","2024",rs,"-o",exe],check=True)
db=sqlite3.connect(":memory:")
db.execute("create table claims(slot integer)")
for row in subprocess.check_output([exe],text=True).splitlines():
 n,count,reads=map(int,row.split()); claims=[]
 for k in range(5):
  bits=0 if n<5 else 0x12412891*n
  bits=((bits<<(k*11))|(bits>>(64-k*11)))&((1<<64)-1)
  claims.extend(i for i in range(64) if n==k or (bits>>i)&1)
 claims.extend(i for i in range(64) if n>7 and (i+n)%7==0)
 claims.extend((n+i)%64 for i in range(n%9))
 db.execute("delete from claims");db.executemany("insert into claims values(?)",((i,) for i in claims))
 assert count==db.execute("select count(distinct slot) from claims").fetchone()[0],(n,count)
 assert reads==0,(n,reads,"census scanned concrete registry slots")
print("90 production census cases match SQLite distinct slot ownership without probing registry entries")
