import sys
sys.path.insert(0,'.')
from rep import apply
D="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/"
apply(D+"🏭️construction/📦️backing/🦀️.rs",[
 ("fn allocate_empty_vec<T>(count:usize,actual:&mut Option<usize>)","/// 📦️ Allocates exact empty vector backing and records the physical marker of the real allocation.\npub fn allocate_empty_vec<T>(count:usize,actual:&mut Option<usize>)",1),
 ("#[cfg(test)]#[path=\"🧪️tests/🦀️.rs\"]mod tests;\n","",1),
])
apply(D+"🏭️construction/🦀️.rs",[("pub use backing::NativeBackingTurn;","pub use backing::{NativeBackingTurn,allocate_empty_vec};",1)])
apply(D+"🦀️.rs",[("NativeBackingTurn,","NativeBackingTurn,allocate_empty_vec,",1)])
s=open(D+"📦️packages/🦀️rust/Cargo.toml",encoding='utf-8').read()
s=s.rstrip('\n')+'\n\n[[test]]\nname = "native-backing"\npath = "../../🏭️construction/📦️backing/🧪️tests/🦀️.rs"\n'
open(D+"📦️packages/🦀️rust/Cargo.toml",'w',encoding='utf-8').write(s)
print("ok")
