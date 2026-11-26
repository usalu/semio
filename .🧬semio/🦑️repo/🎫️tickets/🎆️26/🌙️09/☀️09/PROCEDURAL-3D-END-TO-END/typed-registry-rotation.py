"""🔁 Compare native fixed-registry iteration with SQLite cyclic ordering."""
from pathlib import Path
import os
import sqlite3
import subprocess
import sys
out=Path(sys.argv[1]).resolve()
out.mkdir(exist_ok=True,parents=True)
src=Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🦀️.rs").read_text()
begin=src.index("    pub fn entries_from(")
end=src.index("\n    }",begin)+6
code="""
struct Table<T> { occupied:u64, capacity:usize, entries:[(u64,T);64] }
impl<T> Table<T> {
 fn entry(&self,i:usize)->Option<&(u64,T)>{assert!(self.occupied&(1<<i)!=0);Some(&self.entries[i])}
"""
main="""
}
fn main() {
 for cap in [1,2,4,8,16,32,64] {
  for pattern in [0u64,1,1<<63,0x1011020010010125,u64::MAX] {
   let occupied=pattern & (u64::MAX>>(64-cap));
   let t=Table{occupied,capacity:cap,entries:std::array::from_fn(|i|(1000+i as u64,()))};
   for start in 0..=2*cap {
    let got=t.entries_from(start).map(|(slot,(id,_))|{assert_eq!(*id,1000+slot as u64);slot.to_string()}).collect::<Vec<_>>().join(",");
    println!("{cap}|{occupied}|{start}|{got}");
   }
  }
 }
}
"""
rs=out/"rotate.rs";rs.write_text(code+src[begin:end]+main)
exe=out/("rotate.exe" if os.name=="nt" else "rotate")
subprocess.run([*sys.argv[2:],os.environ.get("RUSTC","rustc"),"--edition","2024",rs,"-o",exe],check=True)
db=sqlite3.connect(":memory:")
db.execute("create table slots(n integer)");db.executemany("insert into slots values(?)",[(i,) for i in range(64)])
rows=subprocess.check_output([exe],text=True).splitlines()
for line in rows:
 cap,occupied,start,got=line.split("|");cap,occupied,start=map(int,(cap,occupied,start))
 mask=occupied if occupied<2**63 else occupied-2**64
 reference=[str(n) for n, in db.execute("select n from slots where n < ? and ((? >> n) & 1) order by ((n - ? + ?) % ?)",(cap,mask,start%cap,cap,cap))]
 assert got==",".join(reference),(line,reference)
print(f"{len(rows)} native sparse rotations match SQLite across all seven supported capacities")
