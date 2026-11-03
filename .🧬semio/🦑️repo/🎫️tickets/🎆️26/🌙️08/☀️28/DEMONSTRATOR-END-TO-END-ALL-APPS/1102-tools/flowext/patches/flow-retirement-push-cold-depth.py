F="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs"
s=open(F).read()
a="maximum_release_bytes:0,maximum_depth:self.queue.len()+1}).result.expect(\"cold Flow queue reservation\");}"
assert s.count(a)==1
b="maximum_release_bytes:0,maximum_depth:self.queue.reserve_demand().expect(\"cold Flow queue depth\").depth}).result.expect(\"cold Flow queue reservation\");}"
open(F,"w").write(s.replace(a,b))
