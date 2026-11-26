"""🔤 Measure actual borrowed JSON character decoding from the paged and contiguous source interface."""
from pathlib import Path
import subprocess,sys
g=Path(__file__).resolve().parent/"🗑️generated/bridge-current/character"
g.mkdir(exist_ok=True)
source=Path("🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🫳️borrowed/🦀️.rs").read_text()
character=source[source.index("pub(super) fn character<"):source.index("fn hex4<")].replace("pub(super) fn","fn")
code="""
use std::cell::Cell;
#[derive(Debug)]enum JsonError{UnexpectedEof,InvalidUtf8}
trait JsonReadSource{fn byte_at(&self,i:usize)->Option<u8>;}
struct Source<'a>{bytes:&'a[u8],reads:Cell<usize>}
impl JsonReadSource for Source<'_>{fn byte_at(&self,i:usize)->Option<u8>{self.reads.set(self.reads.get()+1);self.bytes.get(i).copied()}}
"""+character+"""
fn main(){
 let bytes=b"setContributions";
 let src=Source{bytes,reads:Cell::new(0)};let start=std::time::Instant::now();
 for _ in 0..50_000{for i in 0..bytes.len(){std::hint::black_box(character(std::hint::black_box(&src),i).unwrap());}}
 println!("{} chars {} original source reads elapsed {:?}",bytes.len()*50000,src.reads.get(),start.elapsed());
 let mut total=0;
 for value in 0..=0x10ffff{if let Some(c)=char::from_u32(value){let mut row=[0;4];let text=c.encode_utf8(&mut row);let src=Source{bytes:text.as_bytes(),reads:Cell::new(0)};assert_eq!(character(&src,0).unwrap(),c);total+=1;}}
 println!("{} Unicode scalar values round-tripped",total);
 assert_eq!(src.reads.get(),bytes.len()*50000);
}
"""
(g/"run.rs").write_text(code)
subprocess.run(["rustc",str(g/"run.rs"),"-o",str(g/"run")],check=True)
sys.exit(subprocess.call([str(g/"run")]))
