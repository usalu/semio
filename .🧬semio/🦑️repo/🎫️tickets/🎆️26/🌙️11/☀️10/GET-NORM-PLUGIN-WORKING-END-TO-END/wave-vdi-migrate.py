import re, pathlib
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
S = pathlib.Path(str(R)+"/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any")
IO = S / "🚪️io"
model, muts, diff = S / "🧬️schema/📸️snapshot", S / "🧬️schema/🧬️mutations", S / "🧬️schema/🔺️diff"
cr = "semio_s_artifact_norm_vdi3805"
def rd(p): return p.read_text()
def wr(p, s): p.write_text(s)
def sub(p, pairs):
    s = rd(p)
    for a, b in pairs:
        assert a in s, (p, a)
        s = s.replace(a, b)
    wr(p, s)
def append_once(p, block, marker):
    s = rd(p)
    if marker not in s:
        wr(p, s.rstrip("\n") + "\n\n" + block)
def add_dep(toml, line, name):
    s = rd(toml)
    head, _, rest = s.partition("[dependencies]\n")
    if re.search(r"^" + re.escape(name) + r" =", rest.split("\n[")[0], re.M):
        return
    wr(toml, head + "[dependencies]\n" + line + "\n" + rest)
DEPS = {
    "pack": 'pack = { workspace = true }',
    "semio-framework-value-native-owned": 'semio-framework-value-native-owned = { workspace = true }',
    "semio-s-artifact-norm-contract": 'semio-s-artifact-norm-contract = { workspace = true }',
    "semio-framework-diagnostic": 'semio-framework-diagnostic = { workspace = true }',
    "semio-s-plugin-norm-compliance": 'semio-s-plugin-norm-compliance = { workspace = true }',
}
append_once(model / "🦀️.rs", '#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]\npub mod sqlite;\n\n#[path = "../../🚪️io/🪞️records/🦀️.rs"]\nmod records;\n', "🪞️records")
(IO / "🪞️records").mkdir(exist_ok=True)
wr(IO / "🪞️records/🦀️.rs", "//! 🪞️ Vdi3805 snapshot record impls, owned by the crate that owns `Vdi3805Snapshot` so the orphan rule holds.\n\nuse crate::Vdi3805Snapshot;\n\nsemio_s_plugin_norm_compliance::impl_norm_artifact_record!(Vdi3805Snapshot, extension=\"vdi3805\", envelope_id=\"norm.vdi3805\", sqlite=crate::sqlite::snapshot::sqlite_codec);\n")
for p in (IO / "🪶️sqlite").rglob("*.rs"):
    s = rd(p)
    if f"{cr}_model::" in s:
        wr(p, s.replace(f"{cr}_model::", "crate::"))
for name in ("pack", "semio-framework-value-native-owned", "semio-s-artifact-norm-contract"):
    add_dep(model / "📦️packages/🦀️rust/Cargo.toml", DEPS[name], name)
sub(IO / "💾️binary/📸️snapshot/🦀️.rs", [('\nsemio_s_plugin_norm_compliance::impl_norm_artifact_record!(@binary Vdi3805Snapshot, extension="vdi3805", envelope_id="norm.vdi3805", sqlite=crate::sqlite::snapshot::sqlite_codec);\n', "\n")])
sub(IO / "📝️text/📸️snapshot/🦀️.rs", [('\nsemio_s_plugin_norm_compliance::impl_norm_artifact_record!(@text Vdi3805Snapshot, extension="vdi3805", envelope_id="norm.vdi3805");\n', "\n")])
append_once(muts / "🦀️.rs", '#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]\npub mod text;\n\n#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]\npub mod binary;\n', "pub mod text;")
mp = [(f"use {cr}_mutations::Vdi3805Mutation;", "use crate::Vdi3805Mutation;"), (f"use {cr}_mutations::*;", "use crate::*;"), ("crate::mutations::", "crate::text::")]
for p in (IO / "📝️text/🧬️mutations").rglob("*.rs"):
    s = rd(p)
    for a, b in mp:
        s = s.replace(a, b)
    wr(p, s)
for p in (IO / "💾️binary/🧬️mutations").rglob("*.rs"):
    s = rd(p)
    for a, b in mp[:2]:
        s = s.replace(a, b)
    wr(p, s)
append_once(IO / "💾️binary/🧬️mutations/🦀️.rs", "dsl::original_pack_value_operation_receiving!(crate::Vdi3805Mutation);\n", "original_pack_value_operation_receiving")
for name in ("semio-framework-diagnostic", "semio-s-plugin-norm-compliance", "semio-s-artifact-norm-contract", "pack", "semio-framework-value-native-owned"):
    add_dep(muts / "📦️packages/🦀️rust/Cargo.toml", DEPS[name], name)
append_once(diff / "🦀️.rs", '#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]\npub mod text;\n\n#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]\npub mod binary;\n', "pub mod text;")
add_dep(diff / "📦️packages/🦀️rust/Cargo.toml", DEPS["semio-framework-diagnostic"], "semio-framework-diagnostic")
sub(IO / "📝️text/🔺️diff/🦀️.rs", [(f"{cr}_diff::Vdi3805Diff", "crate::Vdi3805Diff")])
sub(IO / "💾️binary/🔺️diff/🦀️.rs", [(f"{cr}_diff::Vdi3805Diff", "crate::Vdi3805Diff")])
for kind, mod in (("📝️text", "text"), ("💾️binary", "binary")):
    p = IO / kind / "🦀️.rs"
    s = rd(p)
    s = s.replace('#[path = "🧬️mutations/🦀️.rs"]\npub mod mutations;', f"pub use {cr}_mutations::{mod} as mutations;")
    s = s.replace('#[path = "🔺️diff/🦀️.rs"]\npub mod diff;', f"pub use {cr}_diff::{mod} as diff;")
    wr(p, s)
sub(IO / "🦀️.rs", [('#[path = "🪶️sqlite/🦀️.rs"]\npub mod sqlite;', f"pub use {cr}_model::sqlite;")])
print("ok")
