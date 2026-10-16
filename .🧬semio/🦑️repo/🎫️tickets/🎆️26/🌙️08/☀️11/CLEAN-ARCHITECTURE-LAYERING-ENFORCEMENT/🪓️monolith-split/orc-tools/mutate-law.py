"""Seeded mutations of the single-owner law (orc): copies the framework pack tree and the os-pack tree into a sandbox under orc-work, applies one mutation, runs the os-pack single-owner law from inside the sandbox and reports whether it failed.
Usage: mutate-law.py none|m1|m2|m3|m4|m5|m6   (writes only under orc-work/law-*; the live tree is only read)"""
import json, os, shutil, subprocess, sys
from orclib import *
W = G + "/orc-work"
which = sys.argv[1]
sb = W + "/law-" + which
def under(p):
    rp = os.path.realpath(p)
    assert rp.startswith(os.path.realpath(W) + os.sep), p
under(sb)
if os.path.exists(sb): shutil.rmtree(sb)
def copy(rel):
    base = ROOT + "/" + rel
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in ("node_modules", "target", "🗑️generated")]
        for f in fn:
            p = os.path.join(dp, f)
            if os.path.islink(p) or os.path.getsize(p) > 300_000: continue
            if not f.endswith((".rs", ".json", ".toml", ".ts")): continue
            dst = os.path.join(sb, os.path.relpath(p, ROOT)); under(dst)
            os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(p, dst)
copy("🧰️framework/🔨️modules/🎒️pack"); copy("🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack")
os.symlink(ROOT + "/node_modules", sb + "/node_modules")
osr = sb + "/🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack"
fwv = sb + "/🧰️framework/🔨️modules/🎒️pack"
def append(path, text):
    under(path)
    with open(path, "a", encoding="utf8") as f: f.write(text)
if which == "m1": append(osr + "/🦀️.rs", "\nfn encode_string_inline(s: &str, out: &mut Vec<u8>) { out.push(7); }\n")
if which == "m2": append(osr + "/🌉️bridge/🦀️.rs", "\npub const TAG_NULL: u8 = 0x12;\n")
if which == "m3":
    d = fwv + "/🧬️schema/📡️record-token"; under(d); os.makedirs(d, exist_ok=True); open(d + "/🦀️.rs", "w").write("pub const TAG_ABSENT_COPY: u8 = 0;\n")
if which == "m4":
    p = fwv + "/🌱️value/🧬️schema/🦀️.rs"; under(p); t = open(p, encoding="utf8").read()
    line = [l for l in t.split("\n") if l.lstrip().startswith("pub const TAG_EXPR:")][0]
    open(p, "w", encoding="utf8").write(t.replace(line + "\n", "", 1))
if which == "m6": append(osr + "/🌉️bridge/🦀️.rs", "\npub const TAG_EXTRA_VARIANT: u8 = 0x30;\n")
if which == "m5": append(osr + "/🦀️.rs", "\n#[path = \"🌱️value/🦀️.rs\"]\npub mod value;\n")
law = osr + "/🧪️tests/🪪️single-owner/🟦️.ts"
code = 'const m = await import(' + json.dumps(law, ensure_ascii=False) + '); m.testPackSingleOwner();'
p = subprocess.run(["bun", "-e", code], cwd=sb, capture_output=True, text=True)
out = p.stdout + p.stderr
fail = [l.strip() for l in out.split("\n") if "AssertionError" in l or "assert" in l.lower() and "Error" in l]
print(f"mutation {which}: exit={p.returncode} ->", "CAUGHT" if p.returncode != 0 else "NOT CAUGHT" if which != "none" else "clean")
for l in fail[:3]: print("   ", l[:200])
