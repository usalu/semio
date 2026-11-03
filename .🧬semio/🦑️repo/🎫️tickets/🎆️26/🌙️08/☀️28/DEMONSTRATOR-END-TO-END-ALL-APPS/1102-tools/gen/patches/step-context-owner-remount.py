import sys
p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs"
s=open(p).read()
a='mod original_failure_context;\n'
assert s.count(a)==1 and "StepContextOwner" not in s
open(p,"w").write(s.replace(a,a+'\n#[path="📦️owner/🦀️.rs"]\nmod step_context_owner;\npub use step_context_owner::StepContextOwner;\n'))
