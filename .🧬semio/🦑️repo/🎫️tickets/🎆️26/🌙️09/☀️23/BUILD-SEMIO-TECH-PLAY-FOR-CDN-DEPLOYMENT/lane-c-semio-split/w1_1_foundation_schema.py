#!/usr/bin/env python3
"""🧱️ W1 step 1: the foundation tree `✉️base/🦀️.rs` takes over geometry, triples and child from the base union schema."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_uses, rewrite_linear

FOUNDATION = SUBSETS + "/✉️base/🦀️.rs"
write(FOUNDATION, '''//! 🧱️ Foundation of the semio artifact family: the vocabulary, codec primitives and editor plumbing every subset shares.

#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🪆️child/🦀️.rs"]
    pub mod child;
    #[path = "🧬️schema/🧮️geometry/🦀️.rs"]
    pub mod geometry;
    #[path = "🧬️schema/🧰️triples/🦀️.rs"]
    pub mod triples;
}
''')
root = read(ROOT_RS)
for leaf, name in (("🪆️child", "child"), ("🧮️geometry", "geometry"), ("🧰️triples", "triples")):
    old = '                    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/%s/🦀️.rs"]\n                    pub mod %s;\n' % (leaf, name)
    assert root.count(old) == 1, name
    root = root.replace(old, "")
anchor = "#[path = \".\"]\npub mod standards {"
assert root.count(anchor) == 1
root = root.replace(anchor, '#[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs"]\npub mod base_foundation;\n\n' + anchor)
write(ROOT_RS, root)

MOVED = ("geometry", "triples", "child")
def mapper(seg):
    for head in (["crate"], ["semio_s_artifact_stdio_semio"]):
        n = len(head)
        if seg[:n] == head and seg[n:n + 5] == ["standards", "v1", "subsets", "base", "schema"] and len(seg) > n + 5 and seg[n + 5] in MOVED:
            return head + ["base_foundation", "schema"] + seg[n + 5:]
    return None
linear = [(r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::base::schema::(geometry|triples|child)\b", r"\1::base_foundation::schema::\2")]
total = {}
for p in list(semio_rust_files()) + consumer_rust_files():
    s = read(p)
    s2, a = rewrite_linear(s, linear)
    s3, b = rewrite_uses(s2, mapper)
    if s3 != s:
        write(p, s3)
        total[p] = a + b
print(len(total), "files", sum(total.values()), "rewrites")
