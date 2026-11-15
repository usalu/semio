import re, pathlib, sys
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
ART = pathlib.Path(str(R)+"/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
ARTS = {"🌬️din16798": ("din16798", "Din16798"), "🧱️din4108": ("din4108", "Din4108"), "⚡️din18599": ("din18599", "Din18599")}
def rd(p): return p.read_text()
def wr(p, s): p.write_text(s)
def sub(p, pairs):
    s = rd(p)
    for a, b in pairs:
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
for folder, (low, pas) in ARTS.items():
    S = ART / folder / "🏅️standards/🔖️1/🪆️subsets/✳️any"
    IO = S / "🚪️io"
    cr = f"semio_s_artifact_norm_{low}"
    model, muts, diff = S / "🧬️schema/📸️snapshot", S / "🧬️schema/🧬️mutations", S / "🧬️schema/🔺️diff"
    append_once(model / "🦀️.rs", '#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]\npub mod sqlite;\n\n#[path = "../../🚪️io/🪞️records/🦀️.rs"]\nmod records;\n', "🪞️records")
    (IO / "🪞️records").mkdir(exist_ok=True)
    wr(IO / "🪞️records/🦀️.rs", f"//! 🪞️ {pas} snapshot record impls, owned by the crate that owns `{pas}Snapshot` so the orphan rule holds.\n\nuse crate::{pas}Snapshot;\n\nsemio_s_plugin_norm_compliance::impl_norm_artifact_record!({pas}Snapshot, extension=\"{low}\", envelope_id=\"norm.{low}\", sqlite=crate::sqlite::snapshot::sqlite_codec);\n")
    sub(IO / "🪶️sqlite/📸️snapshot/🦀️.rs", [(f"{cr}_model::", "crate::")])
    for name in ("pack", "semio-framework-value-native-owned", "semio-s-artifact-norm-contract"):
        add_dep(model / "📦️packages/🦀️rust/Cargo.toml", DEPS[name], name)
    append_once(muts / "🦀️.rs", '#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]\npub mod text;\n\n#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]\npub mod binary;\n', "pub mod text;")
    mp = [(f"use {cr}_mutations::{pas}Mutation;", f"use crate::{pas}Mutation;"), (f"use {cr}_mutations::*;", "use crate::*;"), ("crate::text::mutations::", "crate::text::"), ("crate::binary::mutations::", "crate::binary::"), ("crate::mutations::", "crate::text::")]
    sub(IO / "📝️text/🧬️mutations/🦀️.rs", mp)
    sub(IO / "💾️binary/🧬️mutations/🦀️.rs", mp)
    for name in ("semio-framework-diagnostic", "semio-s-plugin-norm-compliance", "semio-s-artifact-norm-contract"):
        add_dep(muts / "📦️packages/🦀️rust/Cargo.toml", DEPS[name], name)
    append_once(diff / "🦀️.rs", '#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]\npub mod text;\n\n#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]\npub mod binary;\n', "pub mod text;")
    add_dep(diff / "📦️packages/🦀️rust/Cargo.toml", DEPS["semio-framework-diagnostic"], "semio-framework-diagnostic")
    sub(IO / "📝️text/🔺️diff/🦀️.rs", [(f"{cr}_diff::{pas}Diff", f"crate::{pas}Diff")])
    sub(IO / "💾️binary/🔺️diff/🦀️.rs", [(f"{cr}_diff::{pas}Diff", f"crate::{pas}Diff")])
    print(folder, "ok")
