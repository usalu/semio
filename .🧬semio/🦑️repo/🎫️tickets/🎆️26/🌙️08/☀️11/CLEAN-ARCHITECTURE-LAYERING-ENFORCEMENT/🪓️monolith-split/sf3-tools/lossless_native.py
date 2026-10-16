#!/usr/bin/env python3
"""🧮️ sf3 target 1: line-multiset proof that the shared native crate lost nothing. OLD = the three template files plus the generation2d record block, as of the pre-edit copies; NEW = the live crate files.
Lines are stripped; empty lines and pure `//` comment lines are ignored. Prints the removed and the added lines (the explanation is in the result file).
"""
import collections, re, sys

SCR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
R = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural"
NAT = f"{R}/🫀️core/🧬️generation/🪶️sqlite/🚦️native"
S2 = f"{SCR}/sf3pre/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs"
S3 = f"{SCR}/sf3pre/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🦀️.rs"


def rd(p):
    return open(p, encoding="utf-8").read()


def lines(text):
    out = []
    for l in text.split("\n"):
        s = l.strip()
        if s and not s.startswith("//"):
            out.append(s)
    return out


def block(text, art):
    if art == "2d":
        a = text.index("/// 🔒️ `ValueDsl` mirrors")
    else:
        a = text.index("#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]\npub struct ValueDsl {")
    marker = "pub fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {"
    b = text.index(marker, a)
    b = text.index("\n}\n", b) + 3
    return text[a:b]


old2 = lines(rd(f"{SCR}/sf3-native-pre/🦀️.rs")) + lines(rd(f"{SCR}/sf3-native-pre/📏️rows/🦀️.rs")) + lines(rd(f"{SCR}/sf3-native-pre/🌱️value/🦀️.rs")) + lines(block(rd(S2), "2d"))
old3 = lines(block(rd(S3), "3d"))
new = []
for f in ("🦀️.rs", "🧬️records/🦀️.rs", "🧱️document/🦀️.rs", "📏️rows/🦀️.rs", "🌱️value/🦀️.rs"):
    new += lines(rd(f"{NAT}/{f}"))
co, cn = collections.Counter(old2), collections.Counter(new)
removed, added = co - cn, cn - co
print(f"old (template + 2d records): {len(old2)} lines; new crate: {len(new)} lines; 3d records block: {len(old3)} lines (identical to 2d modulo docs: checked by g3_artifact_edits.py)")
print(f"removed {sum(removed.values())} distinct {len(removed)}; added {sum(added.values())} distinct {len(added)}")
for tag, c in (("REMOVED", removed), ("ADDED", added)):
    print("==", tag)
    for l, n in sorted(c.items(), key=lambda kv: kv[0]):
        print(f"  {n}x {l[:170]}")
