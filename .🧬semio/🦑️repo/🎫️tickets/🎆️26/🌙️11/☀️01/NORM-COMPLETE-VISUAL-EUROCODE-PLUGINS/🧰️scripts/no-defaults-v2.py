import re, sys, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
n, folder = sys.argv[1], sys.argv[2]
art = root / folder
snap = art / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"
T = f"En{n}Snapshot"
strann = n == "1998"
ARG = '"de"' if strann else "semio_s_plugin_norm_compliance::AnnexChoice::De"
src = snap.read_text()
m = re.search(r"impl Default for " + T + r" \{\n    fn default\(\) -> Self \{\n(.*?)\n    \}\n\}\n\n", src, re.S)
assert m, "no impl"
body = m.group(1).strip()
ty = "&str" if strann else "semio_s_plugin_norm_compliance::AnnexChoice"
doc = f"    /// 📸️ Reference EN 199{n[-1]} subject under the explicitly chosen annex; the annex is never defaulted.\n"
if body.endswith("()") and "\n" not in body and body.startswith("Self::"):
    fn = f"{doc}    pub fn example(annex: {ty}) -> Self {{\n        Self {{ annex: annex{'.into()' if strann else ''}, ..{body} }}\n    }}\n\n"
    src = src[:m.start()] + src[m.end():]
    k = src.index(f"impl {T} {{\n") + len(f"impl {T} {{\n")
    src = src[:k] + fn + src[k:]
elif body.endswith("compliant_demo()"):
    fn = f"impl {T} {{\n{doc}    pub fn example(annex: {ty}) -> Self {{\n        Self {{ annex, ..compliant_demo() }}\n    }}\n}}\n\n"
    src = src[:m.start()] + fn + src[m.end():]
else:
    raise SystemExit("manual: " + body[:60])
snap.write_text(src)
pat = re.compile(r"\b" + T + r"::default\(\)")
count = 0
for p in art.rglob("*.rs"):
    if "target" in p.parts: continue
    t = p.read_text()
    t2 = pat.sub(f"{T}::example({ARG})", t)
    if t2 != t:
        count += len(pat.findall(t)); p.write_text(t2)
print("replaced", count)
EX = f"semio_s_artifact_norm_en{n}_model::{T}::example({ARG})"
for p in art.rglob("🦀️.rs"):
    t = p.read_text(); o = t
    if p.parent.name == "✏️editor":
        t = t.replace(f'norm_latency_bench!(En{n}PlayApp, variant = "en{n}", scene', f'norm_latency_bench!(En{n}PlayApp, variant = "en{n}", seed = {EX}, scene')
    if p.parent.name == "🗂️case-edits":
        t = t.replace(f"En{n}Cases>(Default::default(),", f"En{n}Cases>({EX},")
    if t != o: p.write_text(t); print("patched", p.parent.name)
