import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
for id_, path, expect in [("production-rust-leaf", "✏s/x/🦀.rs", True), ("root-itself", "🧰framework", True), ("tests-directory", "✏s/x/🧪tests/a.rs", False), ("plain-tests-directory", "🧰framework/a/tests/t.rs", False),
                          ("benches", "🧰framework/a/benches/b.rs", False), ("examples", "🧰framework/a/examples/e.rs", False), ("build-script", "🌎hub/a/build.rs", False), ("fixtures", "🧰framework/🧫fixtures/x.rs", False),
                          ("outside-production-roots", "other/x.rs", False), ("hub-source", "🌎hub/🏗bootstrap/🦀.rs", True)]:
    cases.append({"id": "runtime-source-" + id_, "rule": "interactivityIsRuntimeSource", "args": [path], "expect": expect})
UI = "🧰framework/🔨modules/🖱ui/🔸x/🦀.rs"
ui_lines = [
    "fn a() { block_on(f); }", "// std::fs::read in a comment", "/* std::net::x", "   still comment std::fs::y */ let t = std::fs::z;", 'let s = "std::fs::in_string";', "thread::spawn(|| {});",
    "#[cfg(test)]", "mod tests {", "    fn t() { block_on(f); }", "}", "#[cfg(test)]", "fn only_test() { block_on(f); }", "use rayon::prelude::*;",
]
BRIDGE = "🧰framework/🛍products/💻os/🔨modules/🔌plugin/🖥host/🧵shard/🧵executor/🦀.rs"
rows = [["blocking-bridge", UI, 1, ui_lines[0]], ["sync-fs", UI, 4, ui_lines[3].strip()], ["thread-pool", UI, 6, ui_lines[5]], ["thread-pool", UI, 13, ui_lines[12]]]
rows += [["blocking-bridge", BRIDGE, 1, "block_on(f);"], ["thread-pool", "🌎hub/x/🦀.rs", 2, "thread::spawn(f);"]]
cases.append({"id": "interactive-sources-are-scanned-by-scope", "rule": "interactivityAuditScan", "files": {
    UI: ui_lines, BRIDGE: ["block_on(f);"], "🌎hub/x/🦀.rs": ["std::fs::read();", "thread::spawn(f);"], "🧰framework/🔨modules/⏳async/🦀.rs": ["thread::spawn(f);", "block_on(f);"],
    "🧰framework/🔨modules/🖱ui/🔸x/🧪tests/t.rs": ["block_on(f);"], "🧰framework/🔨modules/🖱ui/🔸x/notes.txt": ["block_on(f);"], "other/x.rs": ["block_on(f);"],
}, "expect": jsort(rows)})
schema = val_schema("Interactivity Audit Scan Corpus", ["interactivityIsRuntimeSource", "interactivityAuditScan"])
ts = val_ts(
    "🔎 The interactivity audit scan reports exactly the forbidden-call findings its language-agnostic corpus names (scope, comments, literals, test items), classifies runtime sources as the corpus does, and agrees with an independent Python model.",
    'import { interactivityAuditScan, interactivityIsRuntimeSource } from "../../🟦.ts";',
    '''  interactivityIsRuntimeSource: (_root, path: string) => interactivityIsRuntimeSource(path),
  interactivityAuditScan: (root) => interactivityAuditScan(root, { interactivityRuntimeSourceSelfTests: () => undefined } as never).map((f) => [f.category, f.file, f.line, f.text]).sort((a, b) => { const p = JSON.stringify(a), q = JSON.stringify(b); return p < q ? -1 : p > q ? 1 : 0; }),''',
)
folder, plen = write_test("⚖laws/🔬interactivity-audit/🔎scan", "🔎scan", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
