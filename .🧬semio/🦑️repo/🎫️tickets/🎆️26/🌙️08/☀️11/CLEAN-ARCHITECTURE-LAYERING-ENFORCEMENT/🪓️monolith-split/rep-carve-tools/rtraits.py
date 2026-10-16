"""🧭 Trait-in-scope scan: traits a file could call through before the split but that its new namespace no longer provides."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import rewrite

EXT_TRAITS = {
    "ToValue": ["to_value"], "FromValue": ["from_value"], "RetireOwned": ["retirement", "retirement_birth_bytes", "controlled_retirement_supported", "retirement_element_copy_bytes"],
    "RecordHasher": ["hash"], "Hasher": ["update", "finalize", "finalize_into"], "ArtifactCanonicalJsonTree": ["canonical_tree_node", "canonical_tree_child", "canonical_tree_key"],
    "PackSource": ["len", "is_empty", "read_at", "read_exact_at"], "PackSink": ["write_all", "position", "flush"], "CompressionCodec": ["compress", "decompress"],
    "Signer": ["scheme", "key_id", "sign"], "SignatureVerifier": ["verify"],
}

def trait_table(old):
    tab = {n: list(m) for n, m in EXT_TRAITS.items()}
    for node in old.nodes.values():
        for name, it in node.defs.items():
            if it["kind"] == "trait":
                tab[name] = re.findall(r"\bfn\s+([A-Za-z_]\w*)", it["text"])
    return tab

def main():
    model = rewrite.Model({})
    old = model.old
    traits = trait_table(old)
    out = []
    for key in C.KEYS:
        w = model.worlds[key]
        for f in crawl(C.libroot(key)):
            nn = w.nodes.get(f); on = old.nodes.get(f)
            if nn is None or on is None: continue
            text = read_stage_or_base(f)
            cut = text.find("\n\nmod owned_retirement {")
            body = text if cut < 0 else text[:cut]
            for t, methods in traits.items():
                if not methods: continue
                if not any(re.search(r"\.\s*" + re.escape(m) + r"\s*(?:::<[^>]*>)?\s*\(", body) for m in methods): continue
                if old.lookup(on, t) is None: continue
                if w.lookup(nn, t) is not None: continue
                out.append((key, f.replace(R, ""), t))
    for o in out: print(*o)
    print("trait candidates:", len(out))
main()
