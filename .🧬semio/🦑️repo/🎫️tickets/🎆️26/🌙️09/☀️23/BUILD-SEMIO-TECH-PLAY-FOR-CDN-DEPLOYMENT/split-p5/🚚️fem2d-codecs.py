#!/usr/bin/env python3
"""🚚️ Places each fem2d codec beside the type it implements a foreign trait for (orphan rule): snapshot and diff codecs
(text, binary, SQLite) join the model crate, mutation codecs join the mutations crate, the analysis report bridges move
to `🚪️io/📋️reports`, and the schema/inference descriptors (the twenty-five embedded schema leaves) move to the subset
declaration in the aggregator. Module paths stay `standards::v1::subsets::any::io::…`; only the owning crate changes.

usage: 🚚️fem2d-codecs.py
"""
import os, re
REPO = "/Users/ueli/Documents/semio"
ANY = REPO + "/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any"
IO = ANY + "/🚪️io"
P = "standards::v1::subsets::any::io"
rd = lambda p: open(p, encoding="utf-8").read()
def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True); open(p, "w", encoding="utf-8").write(s)

tm = IO + "/📝️text/🧬️mutations/🦀️.rs"; s = rd(tm)
a = s.index("/// 🗂️ The three projection axes"); b = s.rindex("}\npub use mutations_codec::*;")
reports = s[a:b]
wr(tm, s[:a].rstrip() + "\n" + s[b:])
wr(IO + "/📋️reports/🦀️.rs", "//! 📋️ Fem2d analysis report bridges: JSON in, JSON out, for language-neutral test adapters and third-party solver oracles.\n\n"
   "use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;\n"
   "use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;\n"
   "use protocol::Mutation;\n\n" + reports.replace("pub(crate) fn fem2d_frame_axes", "pub fn fem2d_frame_axes"))

io = IO + "/🦀️.rs"; s = rd(io)
for m in ["binary", "text", "sqlite"]:
    s = re.sub(r'#\[path = "[^"]+"\]\npub mod ' + m + r';\n\n?', "", s, count=1)
wr(io, s)
for d in ["📝️text", "💾️binary", "🪶️sqlite"]:
    os.remove(f"{IO}/{d}/🦀️.rs")

def mount(root, block, anchor):
    s = rd(root); assert anchor in s, root; wr(root, s.replace(anchor, anchor + block, 1))
ind = " " * 16
model_io = f'''
{ind}#[path = "."]
{ind}pub mod io {{
{ind}    #[path = "."]
{ind}    pub mod text {{
{ind}        #[path = "../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
{ind}        pub mod snapshot;
{ind}        #[path = "../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
{ind}        pub mod diff;
{ind}    }}
{ind}    #[path = "."]
{ind}    pub mod binary {{
{ind}        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
{ind}        pub mod snapshot;
{ind}        #[path = "../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
{ind}        pub mod diff;
{ind}    }}
{ind}    #[path = "."]
{ind}    pub mod sqlite {{
{ind}        #[path = "../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
{ind}        pub mod snapshot;
{ind}    }}
{ind}}}'''
mount(ANY + "/🧬️schema/📦️packages/🦀️rust/🦀️.rs", model_io, "            pub mod any {")
mut_io = f'''
{ind}#[path = "."]
{ind}pub mod io {{
{ind}    #[path = "."]
{ind}    pub mod text {{
{ind}        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
{ind}        pub mod mutations;
{ind}    }}
{ind}    #[path = "."]
{ind}    pub mod binary {{
{ind}        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
{ind}        pub mod mutations;
{ind}    }}
{ind}}}'''
mount(ANY + "/🧬️schema/🧬️mutations/📦️packages/🦀️rust/🦀️.rs", mut_io, "            pub mod any {")
i2 = " " * 20
io_io = f'''
{i2}#[path = "."]
{i2}pub mod text {{
{i2}    #[path = "../../📝️text/💡️inferences/🦀️.rs"]
{i2}    pub mod inferences;
{i2}}}
{i2}#[path = "."]
{i2}pub mod binary {{
{i2}    #[path = "../../💾️binary/💡️inferences/🦀️.rs"]
{i2}    pub mod inferences;
{i2}}}
{i2}#[path = "../../📋️reports/🦀️.rs"]
{i2}pub mod reports;'''
mount(IO + "/📦️packages/🦀️rust/🦀️.rs", io_io, "                    pub use component::*;")

def files(d):
    for r, _, fs in os.walk(d):
        for f in fs:
            if f == "🦀️.rs" and "📦️packages" not in r: yield os.path.join(r, f)
for d in ["📝️text/📸️snapshot", "📝️text/🔺️diff", "💾️binary/📸️snapshot", "💾️binary/🔺️diff", "🪶️sqlite/📸️snapshot"]:
    for f in files(f"{IO}/{d}"):
        if "🪶️sqlite/📸️snapshot/🧪️tests" in f: continue
        wr(f, rd(f).replace("semio_s_artifact_fem_2d_model::", "crate::"))
for d in ["📝️text/🧬️mutations", "💾️binary/🧬️mutations"]:
    for f in files(f"{IO}/{d}"):
        wr(f, rd(f).replace("semio_s_artifact_fem_2d_mutations::", "crate::"))

sch = ANY + "/🧬️schema/🦀️.rs"; s = rd(sch)
a = s.index("//#region 🔖️Descriptor"); b = s.index("//#endregion 🔖️Descriptor") + len("//#endregion 🔖️Descriptor\n")
desc = s[a:b]; wr(sch, s[:a] + s[b:])
inf = ANY + "/🧬️schema/💡️inferences/🦀️.rs"; s = rd(inf)
a = s.index("//#region 🔖️Descriptor"); b = s.index("//#endregion 🔖️Descriptor") + len("//#endregion 🔖️Descriptor\n")
idesc = s[a:b]; wr(inf, s[:a] + s[b:])
desc = re.sub(r'include_str!\("([^"]+)"\)', lambda m: f'include_str!("🧬️schema/{m.group(1)}")', desc)
idesc = re.sub(r'include_str!\("([^"]+)"\)', lambda m: f'include_str!("🧬️schema/💡️inferences/{m.group(1)}")', idesc)
idesc = idesc.replace("//#region 🔖️Descriptor", "//#region 🔖️InferenceDescriptor").replace("//#endregion 🔖️Descriptor", "//#endregion 🔖️InferenceDescriptor")
sub = ANY + "/🦀️.rs"; s = rd(sub)
s = s.replace("schema::inferences::fem2d_artifact_inference_descriptor()", "fem2d_artifact_inference_descriptor()").replace("schema::fem2d_artifact_schema_descriptor()", "fem2d_artifact_schema_descriptor()")
wr(sub, s.rstrip() + "\n\n" + desc + "\n" + idesc)

TEXT_REPORTS = r"fem2d_(?:analysis|modal|buckling|mutated_analysis)_report_json|fem2d_frame_axes"
def rewrite(src, inside_io):
    head = r"(?:semio_s_artifact_fem_2d_io|crate)" if inside_io else r"semio_s_artifact_fem_2d_io"
    src = re.sub(head + r"::" + P + r"::text::mutations::(" + TEXT_REPORTS + ")", r"semio_s_artifact_fem_2d_io::" + P + r"::reports::\1", src)
    src = re.sub(head + r"::" + P + r"::(text|binary)::(snapshot|diff)\b", r"semio_s_artifact_fem_2d_model::" + P + r"::\1::\2", src)
    src = re.sub(head + r"::" + P + r"::sqlite\b", r"semio_s_artifact_fem_2d_model::" + P + r"::sqlite", src)
    src = re.sub(head + r"::" + P + r"::(text|binary)::mutations\b", r"semio_s_artifact_fem_2d_mutations::" + P + r"::\1::mutations", src)
    if inside_io: src = src.replace("semio_s_artifact_fem_2d_io::" + P + "::reports::", "crate::" + P + "::reports::")
    return src
roots = [REPO + "/✏️s/🔌️plugins/🏗️fem", REPO + "/🌎️hub/🧩️compositions/🏗️fem", REPO + "/✏️s/🔌️plugins/📕️norm"]
moved = ("📝️text/📸️snapshot", "📝️text/🔺️diff", "💾️binary/📸️snapshot", "💾️binary/🔺️diff", "🪶️sqlite/📸️snapshot/🦀️.rs", "📝️text/🧬️mutations", "💾️binary/🧬️mutations")
n = 0
for root in roots:
    for r, ds, fs in os.walk(root):
        ds[:] = [d for d in ds if d not in ("target", "node_modules", "🗑️generated")]
        for f in fs:
            if not f.endswith(".rs"): continue
            p = os.path.join(r, f); rel = os.path.relpath(p, IO)
            inside = p.startswith(IO + "/") and not any(rel.startswith(m) for m in moved)
            s = rd(p); t = rewrite(s, inside)
            if t != s: wr(p, t); n += 1
print("rewrote", n)
