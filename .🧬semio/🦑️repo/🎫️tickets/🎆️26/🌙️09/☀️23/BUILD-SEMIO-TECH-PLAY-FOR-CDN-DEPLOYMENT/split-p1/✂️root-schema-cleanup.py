#!/usr/bin/env python3
"""✂️ Completes S2b/S2c on the artifact root (lane P1): the composition block (now owned by the snapshot crate), the
pre-migration shim modules and root re-exports, and the whole `schema` mount node leave the root; the mutation crates
are aliased.
"""
import importlib.util, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ROOT = f"{L.ART}/🦀️.rs"; root = L.rd(ROOT)
if "extern crate semio_s_artifact_energy_model_mutations" in root: sys.exit("already applied")
root = L.replace_once(root, "pub use crate::schema::EnergyModelArtifact;\n\n", "")
root, _ = L.take(root, "/// 🔖️ Document schema / DSL envelope id.\n", "//#endregion ♻️WholeDocumentLoad\n")
root, _ = L.take(root, "// ---- Shims: keep pre-migration module paths resolving for external callers ----\n", "pub use crate::snapshot::EnergyModelSnapshot;\n")
start = '                #[path = "."]\n                pub mod schema {\n'
i = root.index(start); j = root.index("\n                }\n", i) + len("\n                }\n")
root = root[:i] + root[j:].lstrip("\n") if False else root[:i] + root[j:]
cfg = L.load_domains()
dom = "".join(f"extern crate {L.ALIASES[L.domain_alias(s)].replace('-', '_')} as {L.domain_alias(s)};\n" for _, s, _, _ in cfg["domains"])
root = L.replace_once(root, "extern crate semio_s_artifact_energy_model_inferences as inferences;\n", "extern crate semio_s_artifact_energy_model_inferences as inferences;\nextern crate semio_s_artifact_energy_model_mutations as mutations;\n" + dom)
L.wr(ROOT, root)
print("ok", root.count("\n"))
