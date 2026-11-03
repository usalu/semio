"""🔎 Associated function calls `Type::name(` on repo types (structs/enums defined once in the artifact) where no impl of Type, no trait impl for Type and no enum variant/derive provides `name` (E0599)."""
import re, sys, os, pickle
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
METH, BLANKET = pickle.load(open(S + "/../../🗑️generated/compile-review-s/impl-index.pkl", "rb"))
structs, enums, files = index_types(ART + "/" + CODE[code])
DER = set("default from into clone eq ne fmt cmp partial_cmp hash try_from try_into from_str to_string to_owned borrow as_ref deserialize serialize clone_from schema artifact_schema to_value from_value".split())
for f, src in files.items():
    for m in re.finditer(r'(?<![\w:])([A-Z]\w*)::([a-z_]\w*)\s*(?:::<[^>]*>)?\(', src):
        ty, fn = m.groups()
        if fn in DER or fn in BLANKET: continue
        if ty not in structs and ty not in enums: continue
        if len(structs.get(ty, [])) + len(enums.get(ty, [])) != 1: continue
        if fn in METH.get(ty, set()): continue
        print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `{ty}::{fn}(` no associated fn {fn} on {ty}")
