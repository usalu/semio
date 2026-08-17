from pathlib import Path
import json
import re

TICKET = next(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
state = json.loads((TICKET / "generators" / "w5_cad_state.json").read_text(encoding="utf-8"))
CAD = Path(state["cad"])
TEXT = state["text"]

schema_dir = next(p for p in CAD.iterdir() if p.is_dir() and "".join(c for c in p.name if c.isascii()) == "schema")
schema_rs = next(schema_dir.glob("*component.rs"))
snap_dir = next(p for p in schema_dir.iterdir() if p.is_dir() and "snapshot" in "".join(c for c in p.name if c.isascii()))
diff_dir = next(p for p in schema_dir.iterdir() if p.is_dir() and "diff" in "".join(c for c in p.name if c.isascii()))

t = schema_rs.read_text(encoding="utf-8")

def repl_snap(m):
    return 'include_str!("%s/%s")' % (snap_dir.name, m.group(1))

def repl_diff(m):
    return 'include_str!("%s/%s")' % (diff_dir.name, m.group(1))

t2 = re.sub(r'include_str!\("\.\./[^"]*snapshot[^"]*schema/([^"]+)"\)', repl_snap, t)
t2 = re.sub(r'include_str!\("\.\./[^"]*diff[^"]*schema/([^"]+)"\)', repl_diff, t2)
schema_rs.write_text(t2, encoding="utf-8")
print("schema fixed", t != t2)

text_dir = snap_dir / TEXT
text_rs = next(text_dir.glob("*component.rs"))
examples = next(p for p in CAD.iterdir() if p.is_dir() and "".join(c for c in p.name if c.isascii()) == "examples")
demo = next(p for p in examples.iterdir() if p.is_dir())
assets = next(p for p in demo.iterdir() if p.is_dir() and "assets" in "".join(c for c in p.name if c.isascii()))
example = next(p for p in assets.iterdir() if "example.dsl" in p.name)
rel = "../../../%s/%s/%s/%s" % (examples.name, demo.name, assets.name, example.name)
tt = text_rs.read_text(encoding="utf-8")
tt2 = re.sub(r'include_str!\("([^"]*example\.dsl\.semio)"\)', 'include_str!("%s")' % rel, tt)
text_rs.write_text(tt2, encoding="utf-8")
print("example ->", rel, "exists", (CAD / examples.name / demo.name / assets.name / example.name).exists())
