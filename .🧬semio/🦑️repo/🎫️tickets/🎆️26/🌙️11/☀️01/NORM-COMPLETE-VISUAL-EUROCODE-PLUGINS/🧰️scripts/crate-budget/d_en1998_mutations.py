"""🪚 EN 1998 mutations: `demo_mutation_cases` and the codec unit tests that consume it become `-mutations-demo`."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from d_lib import *
ART = NORM + "/🗿️artifacts/🫨️en1998"
S = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
M = S + "/🧬️schema/🧬️mutations"
P = "semio-s-artifact-norm-en1998-mutations"
N = P + "-demo"
D = M + "/🎲️demo-cases"
TEXT = S + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
BIN = S + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs"
s = rd(TEXT)
a = s.index('#[cfg(any(test, feature = "testkit"))]\npub fn demo_mutation_cases')
b = s.index("#[cfg(test)]\n#[path", a)
if a > 0 and not os.path.exists(D + "/🦀️.rs"):
    fn = s[a:b].replace('#[cfg(any(test, feature = "testkit"))]\n', "", 1).rstrip() + "\n"
    uses = "\n".join(l for l in s.split("\n") if l.startswith("use semio_s_artifact_norm_en1998_mut_"))
    t_rel = relp(S + "/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", D)
    b_rel = relp(S + "/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", D)
    lib = '''//! 🎲️ EN 1998 demo mutations: one retained mutation per variant, with the text and binary codec unit tests that consume them.

#![allow(unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_norm_en1998_mutations::En1998Mutation;
%s

/// 🎲️ The module path the text codec tests address.
pub mod text {
    pub use super::demo_mutation_cases;
    pub use semio_s_artifact_norm_en1998_mutations::text::*;
}

/// 🎲️ The module path the binary codec tests address.
pub mod binary {
    pub use semio_s_artifact_norm_en1998_mutations::binary::*;
    pub use semio_s_artifact_norm_en1998_mutations::En1998Mutation;
}

/// 🎲️ One mutation of every variant, in a valid state of the default snapshot.
%s
#[cfg(test)]
#[path = "%s"]
mod text_tests;

#[cfg(test)]
#[path = "%s"]
mod binary_tests;
''' % (uses, fn.replace("pub fn demo_mutation_cases", "pub fn demo_mutation_cases", 1), t_rel, b_rel)
    wr(D + "/🦀️.rs", lib)
    s = s[:a] + s[b:]
    s = s.replace('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n\n', "", 1)
    wr(TEXT, s)
    bs = rd(BIN)
    bs = bs.replace('//#region 🧪️Tests\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n//#endregion 🧪️Tests\n\n', "", 1)
    wr(BIN, bs)
    toml = M + "/📦️packages/🦀️rust/Cargo.toml"
    deps = [P, "semio-framework-os-kernel", "semio-s-artifact-norm-en1998-model"] + ["semio-s-artifact-norm-en1998-mut-" + x for x in ("bridges-towers", "placements", "buildings", "foundations", "members", "silos-tanks")]
    dev = 'semio-framework-async-macros = { workspace = true }\nsemio-framework-os-kernel = { workspace = true, features = ["protocol-laws", "mutation-testing"] }\nsemio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\n'
    new_crate(ART, D, N, "EN 1998 demo mutations and codec unit tests", deps, P, toml, dev)
