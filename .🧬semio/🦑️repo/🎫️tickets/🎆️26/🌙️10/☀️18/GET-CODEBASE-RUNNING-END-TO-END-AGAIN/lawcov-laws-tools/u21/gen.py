import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
CF = "stdio-artifacts/codec-fidelity"
f1, f2, f3 = "✏s/🔌plugins/🔭p/x.rs", "🧰framework/🔨modules/🔺mesh/m.rs", "🧰framework/🛍products/💻os/h/🖥host/y.rs"
cases.append({"id": "banned-stub-markers-in-framework-and-plugin-codecs", "rule": "policyCodecFidelityBreaches", "files": {
    f1: ['let a = "SRAS";'], f2: ['const B: &[u8] = b"minimal";', "// IFCCARTOONMESH"], f3: ["a minimal stub codec lives here"], "other/z.rs": ["SRAS"], "✏s/🔌plugins/🔭p/notes.txt": ["SRAS"], "✏s/🔌plugins/🔭p/clean.rs": ["fn ok() {}"],
}, "expect": jsort([["stdio-codec-ban-%s-SRAS" % f1, CF, "high", 0], ["stdio-codec-ban-%s-IFCCARTOONMESH" % f2, CF, "high", 0], ["stdio-codec-ban-%s-b\"minimal\"" % f2, CF, "high", 0],
                    ["stdio-codec-ban-%s-stub codec" % f3, CF, "high", 0], ["stdio-codec-ban-%s-minimal stub codec" % f3, CF, "high", 0]])})
for rule in ("policyIoMatrixMigratedBreaches", "policyIoSerializerMatrixBreaches", "policyIoTerminalityBreaches"):
    cases.append({"id": "without-a-stdio-ledger-" + rule, "rule": rule, "files": {"README.md": ["x"]}, "expect": []})
rules = ["policyCodecFidelityBreaches", "policyIoMatrixMigratedBreaches", "policyIoSerializerMatrixBreaches", "policyIoTerminalityBreaches"]
schema = val_schema("Io Matrix Law Corpus", rules)
ts = val_ts(
    "🔌 The stdio io-matrix laws report exactly the banned codec-stub markers their language-agnostic corpus names, agree with an independent Python model, and stay silent without a stdio ledger.",
    "import { policyCodecFidelityBreaches, policyIoMatrixMigratedBreaches, policyIoSerializerMatrixBreaches, policyIoTerminalityBreaches } from \"../../🟦.ts\";",
    "\n".join("  %s: (root) => brs(%s(root))," % (r, r) for r in rules),
)
ts = ts.replace("const rules:", SORT_TS + "\nconst rules:", 1)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🚪artifact-io/🔌io-matrix", "🔌matrix", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
