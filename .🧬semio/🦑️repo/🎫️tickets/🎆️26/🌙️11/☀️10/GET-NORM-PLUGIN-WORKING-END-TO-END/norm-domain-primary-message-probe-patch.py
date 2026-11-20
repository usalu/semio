import pathlib,sys
f=pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🦀️.rs")
s=f.read_text()
a='f.debug_struct("OriginalDomainPrimary").field("code",&self.code()).finish_non_exhaustive()'
b='f.debug_struct("OriginalDomainPrimary").field("code",&self.code()).field("message",&self.message()).finish_non_exhaustive()'
assert s.count(a)==1,"domain file changed; re-read"
if "--apply" not in sys.argv:print("dry run ok; pass --apply to write");sys.exit(0)
f.write_text(s.replace(a,b));print("patched",f)
