# 🔁️ The jpg source edits made after the carve was applied, replayed onto a pre-carve snapshot: usage jpg-post-apply-edits.py <snapshot-root>
import sys, glob, os
r = sys.argv[1]
D = r + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"
def edit(p, fn):
    t = open(p, encoding="utf-8").read(); n = fn(t)
    if n != t: open(p, "w", encoding="utf-8").write(n); print("edited", p[-60:])
def rep(a, b):
    def f(t):
        return t.replace(a, b) if a in t else t
    return f
S = "crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot"
edit(D + "/🧬️schema/🔺️diff/🦀️.rs", rep("#[cfg(test)]\n// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\npub(crate) fn demo_diff_cases() -> Vec<JpgDiff> {", "/// 🧪️ Representative diffs used by the conformance laws of the io facets and by the diff codec tests.\n// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\npub fn demo_diff_cases() -> Vec<JpgDiff> {"))
for p in glob.glob(D + "/🚪️io/📝️text/🧬️mutations/*/🧪️tests/🎯️direct/🦀️.rs"):
    def f(t):
        a = "use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation;\n"
        if "JpgSnapshot" in t and f"use {S};" not in t and a in t: return t.replace(a, a + f"use {S};\n")
        return t
    edit(p, f)
edit(D + "/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs", rep("crate::standards::v_jfif_1_01::subsets::document::io::sqlite::snapshot::owned_text::", "crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::"))
edit(D + "/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs", rep('#[path="🫴️receiving/🦀️.rs"]mod receiving;', '#[path="🫴️receiving/🦀️.rs"]pub mod receiving;'))
