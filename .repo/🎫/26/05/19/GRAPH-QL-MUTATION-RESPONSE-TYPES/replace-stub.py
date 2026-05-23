from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")
old = "crate::operation::CommandResponse::stub_ok().await"
new = 'crate::operation::CommandResponse::fail_msg("not implemented").await'
n = t.count(old)
t = t.replace(old, new)
p.write_text(t, encoding="utf-8", newline="\n")
print("replaced", n)
