import pathlib,sys
root=pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️backing/📦️storage/🦀️.rs")
s=root.read_text()
a="if self.binding.is_some_and(|binding|!binding.matches_effect(effect)){Err(invalid())}"
b="if self.binding.is_some_and(|binding|!binding.matches_source(effect)){Err(invalid())}"
c="if self.binding.is_some_and(|binding|binding!=work){Err(invalid())}"
d="if self.binding.is_some_and(|binding|binding.source_turn()!=work.source_turn()){Err(invalid())}"
assert s.count(a)==1 and s.count(c)==1,"storage file changed; re-read before patching"
if "--apply" not in sys.argv:print("dry run ok; pass --apply to write");sys.exit(0)
root.write_text(s.replace(a,b).replace(c,d))
print("patched",root)
