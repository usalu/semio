import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from valuecarve import *
D = V + "/🛬️decode"
N = D + "/🧪️tests/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
MOUNTS = [
 (D + "/🦀️.rs", "📏️maximum/🧪️tests/🦀️.rs", "maximum_tests", True),
 (D + "/🦀️.rs", "🧪️tests/🦀️.rs", "tests", True),
 (D + "/🦀️.rs", "⚠️refusal/🧪️tests/🦀️.rs", "literal_refusal_tests", True),
 (D + "/🦀️.rs", "🔁️ledger/🧪️tests/🦀️.rs", "original_ledger_tests", True),
 (D + "/🛂️allocation/🦀️.rs", "🧪️tests/🦀️.rs", "tests", True),
 (D + "/🫴️recipient/🔁️continuation/🦀️.rs", "🧪️tests/🦀️.rs", "tests", True),
 (D + "/🏭️construction/🦀️.rs", "🧪️tests/🦀️.rs", "tests", True),
 (D + "/♻️retirement/🦀️.rs", "🧪️tests/🦀️.rs", "tests", True),
 (D + "/📦️packages/🦀️rust/🦀️.rs", "../../../🫴️receiving/🧪️tests/🦀️.rs", "original_receiving_tests", True),
]
glue_mods = []
for f, rel, name, cfg in MOUNTS:
    s = rgl.read(f)
    pat = re.compile(r'(?:#\[cfg\(test\)\]\s*)?#\[path\s*=\s*"' + re.escape(rel) + r'"\]\s*(?:#\[cfg\(test\)\]\s*)?mod ' + name + r'\s*;\n?')
    s2, c = pat.subn("", s)
    assert c == 1, (f[-40:], rel, c)
    rgl.write(f, s2)
    base = os.path.dirname(f)
    if f.endswith("📦️packages/🦀️rust/🦀️.rs"):
        target = os.path.normpath(os.path.join(base, rel))
    else:
        target = os.path.normpath(os.path.join(base, rel))
    test_file = os.path.relpath(target, N)
    glue_mods.append((name if f.endswith("🛬️decode/🦀️.rs") and name != "tests" else None, test_file, f))
names = {}
body = ""
labels = ["maximum_tests", "root_tests", "literal_refusal_tests", "original_ledger_tests", "allocation_tests", "continuation_tests", "construction_tests", "retirement_tests", "original_receiving_tests"]
for (nm, tf, f), label in zip(glue_mods, labels):
    body += f'#[path = "{tf}"]\nmod {label};\n\n'
    p = os.path.normpath(os.path.join(N, tf))
    t = rgl.read(p)
    t2 = re.sub(r'fixture_text!\("((?:\.\./)+)', lambda m: 'fixture_text!("' + m.group(1) + '../', t)
    if t2 != t:
        rgl.write(p, t2)
rgl.write(N + "/Cargo.toml", manifest("semio-framework-value-native-decoding-tests", "semio_framework_value_native_decoding_tests", os.path.relpath(R + "/🧰️framework", N), '''semio-framework-value-native-decoding = { path = "../../../📦️packages/🦀️rust" }''',
 '''semio-framework-test-fixtures = { path = "../../../../../🧪️test/🧲️fixture-reader/📦️packages/🦀️rust" }
semio-framework-value-testing-allocator = { path = "../../../../🧪️testing/💾️allocator/📦️packages/🦀️rust" }
semio-framework-value-retirement = { path = "../../../../♻️retirement/📦️packages/🦀️rust" }
serde = { workspace = true }
serde_json = { workspace = true }''', "Companion laws of the native decoding core: the cumulative ownership ledger, maximum, allocation port, recipient continuation, construction, retirement, refusal and receiving.", "value-native-decoding-tests").replace('role = "framework"', 'role = "test"').replace("[dependencies]", "[dependencies]"))
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/value-native-decoding-tests-rs"))
rgl.write(N + "/🦀️.rs", '''//! 🧪️ Companion laws of the native decoding core, restated against its public vocabulary.
#![cfg(test)]

pub use semio_framework_value_native_decoding::*;
pub use semio_framework_value_retirement::retirement;
use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use allocation_testing::observe_retirement_allocations;

''' + body)
rgl.register(D + "/📦️packages/🦀️rust", "semio-framework-value-native-decoding", N, "semio-framework-value-native-decoding-tests", "@semio-tech/value-native-decoding-rs", "@semio-tech/value-native-decoding-tests-rs", "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/📦️packages/🦀️rust", dep_ws=[])
