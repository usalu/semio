"""🪚 Crate-budget H: the inline EN 1998 formula modules (na_de, part_1..part_6) of the schema become the crate `-schema-formulas` (root `📐️formulas`); the schema re-exports them."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🫨️en1998"
SC = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
P = "semio-s-artifact-norm-en1998-schema"
N = P + "-formulas"
root = SC + "/🦀️.rs"
lines = rd(root).split("\n")
text = "\n".join(lines)
if N.replace("-", "_") in text:
    sys.exit("done")
a = next(i for i, l in enumerate(lines) if l.startswith("pub mod na_de {"))
b = next(i for i, l in enumerate(lines) if l.startswith("pub mod part_6 {"))
e = next(i for i in range(b, len(lines)) if lines[i] == "}")
body = "\n".join(lines[a : e + 1])
model = re.search(r"use semio_s_artifact_norm_en1998_model::\{([^}]*)\};", text).group(1).split(", ")
used = [m for m in model if re.search(r"\bsuper::%s\b|\b%s\b" % (m, m), body)]
head = "//! 📐️ EN 1998 formula modules of the schema: DE-NA ground combinations and the parts 1 to 6 closed-form checks. See [`super`].\n\n#![allow(async_fn_in_trait)]\n\nuse semio_s_artifact_norm_en1998_model::{%s};\n\n" % ", ".join(used)
wr(SC + "/📐️formulas/🦀️.rs", head + body + "\n")
lines[a : e + 1] = ["pub use %s::{na_de, part_1, part_2, part_3, part_4, part_5, part_6};" % N.replace("-", "_")]
wr(root, re.sub(r"\n{3,}", "\n\n", "\n".join(lines)))
toml = SC + "/📦️packages/🦀️rust/Cargo.toml"
dev = rd(toml).split("[dev-dependencies]\n")[1]
new_crate(ART, SC + "/📐️formulas", N, "EN 1998 schema formulas: DE-NA ground combinations and the closed-form checks of parts 1 to 6", ["semio-s-artifact-norm-en1998-model", "semio-s-plugin-norm-compliance"], P, toml, dev=dev)
add_dep(toml, N)
