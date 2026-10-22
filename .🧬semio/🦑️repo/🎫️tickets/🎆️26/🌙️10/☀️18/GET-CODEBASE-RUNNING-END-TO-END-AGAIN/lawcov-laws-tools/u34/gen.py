import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
unit = real_unit("⚖laws/🔬interactivity-audit/🗂files")
src = open(os.path.join(unit, fix("🟦.ts")), encoding="utf8").read()
cases = []
patterns = [
    ("block_on(", "blocking-bridge", "ui", r"\bblock_on\s*\(", ["block_on(fut)", "x.block_on (f)", "block_on_x(f)", "unblock_on(f)"]),
    ("run_blocking(", "blocking-bridge", "ui", r"\brun_blocking\s*\(", ["run_blocking(x)", "run_blocking_x(1)"]),
    ("std::fs::", "sync-fs", "ui", r"\bstd::fs::", ["std::fs::read", "use std::fs;", "xstd::fs::y"]),
    ("std::net::", "sync-net", "ui", r"\bstd::net::", ["std::net::TcpStream", "use std::net;", "xstd::net::y"]),
    ("reqwest::blocking", "sync-net", "ui", r"\breqwest::blocking\b", ["reqwest::blocking::get", "reqwest::blockingx"]),
    ("arboard::", "sync-clipboard", "ui", r"\barboard::", ["arboard::Clipboard", "myarboard::X"]),
    ("std::process::Command", "sync-process", "ui", r"\bstd::process::Command\b", ["std::process::Command::new", "std::process::CommandExt"]),
    ("rusqlite::", "sync-db", "ui", r"\brusqlite::", ["rusqlite::Connection", "xrusqlite::"]),
    ("sqlx::", "sync-db", "ui", r"\bsqlx::", ["sqlx::query", "sqlx_x::q"]),
    ("thread::spawn(", "thread-pool", "runtime-repo-wide", r"\b(?:std::)?thread::spawn\s*\(", ["thread::spawn(f)", "std::thread::spawn (f)", "thread::spawn_x(f)", "mythread::spawn("]),
    ("thread::Builder", "thread-pool", "runtime-repo-wide", r"\b(?:std::)?thread::Builder\b", ["thread::Builder::new()", "std::thread::BuilderX"]),
    ("thread::scope(", "thread-pool", "runtime-repo-wide", r"\b(?:std::)?thread::scope\s*\(", ["thread::scope(|s| {})", "thread::scoped("]),
    ("WorkerPool::new(", "thread-pool", "runtime-repo-wide", r"\bWorkerPool::new\s*\(", ["WorkerPool::new(1)", "WorkerPool::new_x("]),
    ("rayon::", "thread-pool", "runtime-repo-wide", r"\brayon::", ["rayon::join", "xrayon::join"]),
    ("tokio::runtime::Builder", "thread-pool", "runtime-repo-wide", r"\btokio::runtime::Builder\b", ["tokio::runtime::Builder::new", "tokio::runtime::BuilderX"]),
]
for label, cat, scope, source, samples in patterns:
    results = [bool(re.search(source, s, re.A)) for s in samples]
    cases.append({"id": "pattern-" + label, "rule": "pattern", "args": [{"label": label, "category": cat, "scope": scope, "source": source, "samples": samples}], "expect": results})
cases.append({"id": "scope-roots-are-pinned", "rule": "constants", "expect": {
    "INTERACTIVITY_AUDIT_UI_ROOTS": ["🧰️framework/🔨️modules/🖱️ui/", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", "🧰️framework/🛍️products/💻️os/🖥️host/", "✏️s/🔌️plugins/"],
    "INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES": ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs"],
    "INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS": ["🧰️framework/🔨️modules/⏳️async/"], "INTERACTIVITY_AUDIT_PRODUCTION_ROOTS": ["✏️s", "🧰️framework", "🌎️hub"]}})
cases.append({"id": "allowlist-entries-are-pinned", "rule": "allowlist", "expect": [
    {"file": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs", "lineHint": 15, "pattern": "block_on", "inScope": True, "expectedNeverToMatch": False},
    {"file": "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs", "lineHint": 255, "pattern": "block_on", "inScope": False, "expectedNeverToMatch": False},
    {"file": "🧰️framework/🛍️products/💻️os/🖥️host/🔌️plugin-host/🦀️.rs", "lineHint": 62, "pattern": "block_on", "inScope": True, "expectedNeverToMatch": True},
    {"file": "🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️.rs", "lineHint": 58, "pattern": "block_on", "inScope": False, "expectedNeverToMatch": False}]})
names = sorted(re.findall(r"^export const (INTERACTIVITY_[A-Z0-9_]*FILE) = \"", src, re.M))
cases.append({"id": "every-audited-file-constant-is-listed", "rule": "fileConstants", "expect": names})
schema = val_schema("Interactivity Audit Files Corpus", ["pattern", "constants", "allowlist", "fileConstants"])
extra = '''
test("every audited file constant names a Rust leaf inside the production roots or a ticket contract", () => {
  for (const [name, value] of Object.entries(audit)) {
    if (!/^INTERACTIVITY_[A-Z0-9_]*FILE$/.test(name) || typeof value !== "string") continue;
    const production = audit.INTERACTIVITY_AUDIT_PRODUCTION_ROOTS.some((root: string) => (value as string).startsWith(root + "/"));
    expect(production || (value as string).startsWith(".\\u{1F9EC}semio/"), `${name} sits in a production root or a ticket folder`).toBe(true);
  }
  expect(Object.keys(audit).filter((name) => /^INTERACTIVITY_[A-Z0-9_]*FILE$/.test(name)).sort()).toEqual((corpus.cases.find((row: any) => row.rule === "fileConstants") as any).expect);
});
'''
ts = val_ts(
    "🗂 The interactivity-audit pattern table, scope roots and allowlist equal what their language-agnostic corpus pins, every regex matches exactly the samples the corpus names (checked by an independent Python regex engine), and every audited file constant sits in a production root.",
    'import * as audit from "../../🟦.ts";',
    '''  pattern: (_root, row: any) => { const found = audit.INTERACTIVITY_AUDIT_PATTERNS.find((p) => p.label === row.label); if (!found) throw new Error("unknown pattern " + row.label); expect({ category: found.category, scope: found.scope, source: found.re.source }).toEqual({ category: row.category, scope: row.scope, source: row.source }); return row.samples.map((s: string) => { found.re.lastIndex = 0; return found.re.test(s); }); },
  constants: () => ({ INTERACTIVITY_AUDIT_UI_ROOTS: [...audit.INTERACTIVITY_AUDIT_UI_ROOTS], INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES: [...audit.INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES], INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS: [...audit.INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS], INTERACTIVITY_AUDIT_PRODUCTION_ROOTS: [...audit.INTERACTIVITY_AUDIT_PRODUCTION_ROOTS] }),
  allowlist: () => audit.INTERACTIVITY_AUDIT_ALLOWLIST.map((e) => ({ file: e.file, lineHint: e.lineHint, pattern: e.pattern, inScope: e.inScope, expectedNeverToMatch: e.expectedNeverToMatch === true })),
  fileConstants: () => Object.keys(audit).filter((name) => /^INTERACTIVITY_[A-Z0-9_]*FILE$/.test(name)).sort(),''',
    extra,
)
folder, plen = write_test("⚖laws/🔬interactivity-audit/🗂files", "🗂files", ts, {"version": 1, "cases": cases}, schema, oracle, fixdeep=False)
print(folder, plen, len(cases))
