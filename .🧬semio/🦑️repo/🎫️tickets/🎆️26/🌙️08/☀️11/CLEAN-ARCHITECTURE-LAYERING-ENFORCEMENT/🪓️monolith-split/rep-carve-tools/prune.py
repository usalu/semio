"""✂️ Drops imports no longer used by the files the carve cut down (the old root allowed unused imports crate-wide)."""
import re
from rlib import *
import rcrates as C
import usetree

TRAIT_METHODS = {
    "ToValue": ["to_value"], "FromValue": ["from_value"], "RetireOwned": ["retirement", "retirement_birth_bytes", "controlled_retirement_supported"],
    "RecordHasher": ["hash"], "Hasher": ["update", "finalize"], "ArtifactCanonicalJsonTree": ["canonical_tree_node", "canonical_tree_child", "canonical_tree_key"],
    "OpBinary": ["encode_op", "decode_op"], "OpText": ["print_op", "parse_op"], "DiffBinary": ["encode_diff", "decode_diff"], "DiffText": ["print_diff", "parse_diff"],
}

def prune(text):
    its = items(text)
    uses = [it for it in its if it["kind"] == "use" and not it["vis"] and not any("cfg" in a for a in it["attrs"])]
    if not uses: return text
    def kw_start(it):
        for kd, tx, a, b in tokenize(it["text"]):
            if kd == "id" and tx == "use": return it["start"] + a
        raise ValueError("no use keyword")
    starts = {id(it): kw_start(it) for it in uses}
    spans = [(starts[id(it)], it["end"]) for it in uses]
    rest = ""; last = 0
    for a, b in spans:
        rest += text[last:a]; last = b
    rest += text[last:]
    used = bare_ids(rest)
    def keep(leaf):
        n = usetree.local_name(leaf)
        if n is None or n in used: return True
        for m in TRAIT_METHODS.get(n, []):
            if re.search(r"\.\s*" + m + r"\s*(?:::<[^>]*>)?\(|\b" + m + r"\s*\(", rest): return True
        return False
    out = []; last = 0
    for it in uses:
        leaves = usetree.parse(use_tree_of(it))
        kept = [lf for lf in leaves if keep(lf)]
        out.append(text[last:starts[id(it)]]); last = it["end"]
        if kept:
            lines = usetree.emit(kept)
            out.append("\n".join(lines))
        else:
            if text[last:last + 1] == "\n": last += 1
    out.append(text[last:])
    return "".join(out)

def run(files):
    n = 0
    for f in files:
        t = read_stage_or_base(f)
        p = prune(t)
        if p != t: write(f, p); n += 1
    return n
