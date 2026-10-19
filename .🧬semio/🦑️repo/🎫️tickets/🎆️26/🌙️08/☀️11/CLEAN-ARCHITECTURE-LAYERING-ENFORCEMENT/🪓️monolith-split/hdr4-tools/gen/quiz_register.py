import os, re, sys, shutil
sys.path.insert(0, "/Users/ueli/.cache/hdr4/tools")
from h3reg import *
Q = R + "/🧰️framework/🛍️products/❓️quiz"
T = Q + "/🧪️tests"
A = T + "/🥣️companion"
S = T + "/🧷️companion-support"
B = T + "/🎞️companion-runs"
lic = '''[package]
workspace = "../../../../../../.."
name = "%s"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "%s"
publish = false

[package.metadata.semio]
role = "product"
id = "%s"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

'''
sup_man = lic % ("semio-framework-quiz-tests-support", "Shared fixtures of the quiz companion tests: the quiz builders and the learner-lifecycle kit", "quiz-tests-support") + '''[dependencies]
semio-framework-quiz = { path = "../../../../📦️packages/🦀️rust", features = ["testkit"] }
semio-framework-quiz-schema = { path = "../../../../🧬️schema/📦️packages/🦀️rust", features = ["testkit"] }
serde_json = { workspace = true, features = ["float_roundtrip"] }
'''
run_man = lic % ("semio-framework-quiz-tests-runs", "Companion tests of the quiz product: the learner lifecycle and the proctor read models", "quiz-tests-runs") + '''[dependencies]
semio-framework-quiz = { path = "../../../../📦️packages/🦀️rust", features = ["testkit"] }

[dev-dependencies]
semio-framework-quiz-schema = { path = "../../../../🧬️schema/📦️packages/🦀️rust", features = ["testkit"] }
semio-framework-quiz-tests-support = { path = "../../../🧷️companion-support/📦️packages/🦀️rust" }
serde_json = { workspace = true, features = ["float_roundtrip"] }
'''
for d, man in ((S, sup_man), (B, run_man)):
    pk = d + "/" + PKG
    os.makedirs(pk, exist_ok=True)
    assert not os.path.exists(pk + "/Cargo.toml")
    open(pk + "/Cargo.toml", "w", encoding="utf8").write(man)
# A: dev-dep on the support crate
ap = A + "/" + PKG + "/Cargo.toml"
at = rd(ap)
at = at.replace("[dev-dependencies]\n", "[dev-dependencies]\nsemio-framework-quiz-tests-support = { path = \"../../../🧷️companion-support/📦️packages/🦀️rust\" }\n", 1)
wr(ap, at)
# B sources
os.makedirs(B + "/🧾️lifecycle", exist_ok=True); os.makedirs(B + "/👁️views", exist_ok=True)
for m in ("🧾️lifecycle", "👁️views"):
    t = rd(A + "/%s/🦀️.rs" % m)
    open(B + "/%s/🦀️.rs" % m, "w", encoding="utf8").write(t)
    backup(A + "/%s/🦀️.rs" % m)
    os.remove(A + "/%s/🦀️.rs" % m); os.rmdir(A + "/%s" % m)
open(B + "/🦀️.rs", "w", encoding="utf8").write('''//! 🎞️ Companion tests of the quiz product: the learner lifecycle decide/evolve laws and the proctor read-model laws.
//!
//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name; the fixtures both share live in `semio-framework-quiz-tests-support`.
#![cfg(test)]

pub use quiz::*;

#[path = "🧾️lifecycle/🦀️.rs"]
pub mod lifecycle;

#[path = "👁️views/🦀️.rs"]
pub mod views;
''')
ar = A + "/🦀️.rs"
t = rd(ar)
t = t.replace("//! 🥣️ Companion tests of the quiz product: sheet assembly, validation, scoring, badges, the learner lifecycle, the read models and the challenge rules.\n//!\n//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name, so the laws that share fixtures through `crate::sheet::tests` and `crate::lifecycle::tests` keep their paths.",
 "//! 🥣️ Companion tests of the quiz product: sheet assembly, validation, scoring, badges and the challenge rules.\n//!\n//! 🧭️ Each module's unit file is mounted below a wrapper module of the same name; the fixtures shared with the run laws live in `semio-framework-quiz-tests-support`.")
t = t.replace('#[path = "🧾️lifecycle/🦀️.rs"]\npub mod lifecycle;\n\n', "").replace('\n#[path = "👁️views/🦀️.rs"]\npub mod views;\n', "")
wr(ar, t)
for wp in (A + "/🃏️sheet/🦀️.rs", A + "/🧾️lifecycle/🦀️.rs"):
    pass
add_member(R, S + "/" + PKG, A + "/" + PKG)
add_member(R, B + "/" + PKG, S + "/" + PKG)
project_json(B + "/" + PKG, "quiz-tests-runs")
add_owner("semio-framework-quiz-tests-support", S + "/" + PKG)
add_owner("semio-framework-quiz-tests-runs", B + "/" + PKG, project="@semio-tech/quiz-tests-runs-rs")
routes("semio-framework-quiz-tests-runs", "quiz-tests-runs", next_route_index(), "semio-framework-quiz-tests")
print("ok")
