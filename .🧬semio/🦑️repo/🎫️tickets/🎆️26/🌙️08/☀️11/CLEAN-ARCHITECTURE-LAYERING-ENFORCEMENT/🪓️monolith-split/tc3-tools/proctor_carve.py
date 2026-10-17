"""Carves teaching-proctor into base / domain / main crates. usage: proctor_carve.py <step>
steps: fixtures | rewrite | roots | all.  Every step asserts unique anchors; originals are in tc3-tools/orig (snap.py)."""
import re, sys, os

P = "/Users/ueli/Documents/semio/🎓️teaching/🛂️proctor"
M = P + "/🔨️modules"
BASE = ["🗄️storage", "📚️catalog", "🎚️config", "👪️crowd", "👥️presence"]
DOMAIN = ["🎭️actors", "🔭️projections", "❓️queries"]
MAIN = ["🧩️instance", "⌨️cli"]
NAME = {"🗄️storage": "storage", "📚️catalog": "catalog", "🎚️config": "config", "👪️crowd": "crowd", "👥️presence": "presence",
        "🎭️actors": "actors", "🔭️projections": "projections", "❓️queries": "queries", "🧩️instance": "instance", "⌨️cli": "cli"}
BASE_NAMES = [NAME[m] for m in BASE]
DOMAIN_NAMES = [NAME[m] for m in DOMAIN]


def read(p):
    return open(p, encoding="utf-8").read()


def write(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def tests_path(m):
    return f"{M}/{m}/🧪️tests/🔬️unit/🦀️.rs"


def src_path(m):
    return f"{M}/{m}/🦀️.rs"


def item_span(text, header_regex):
    """Return (start, end) character span of an item: its leading doc/attr lines, its signature and its body (brace matched) or its terminating semicolon."""
    m = re.search(header_regex, text, re.M)
    assert m, header_regex
    assert len(re.findall(header_regex, text, re.M)) == 1, header_regex
    start = m.start()
    lines_before = text[:start].split("\n")
    i = len(lines_before) - 1
    while i > 0 and (lines_before[i - 1].lstrip().startswith("///") or lines_before[i - 1].lstrip().startswith("#[")):
        start -= len(lines_before[i - 1]) + 1
        i -= 1
    pos = m.end()
    head = m.group(0)
    if head.rstrip().endswith(";"):
        while pos < len(text) and text[pos] == "\n":
            pos += 1
        return start, pos
    depth = head.count("{") - head.count("}")
    seen_brace = head.count("{") > 0
    in_str = False
    while pos < len(text):
        ch = text[pos]
        if in_str:
            if ch == "\\":
                pos += 1
            elif ch == '"':
                in_str = False
        elif ch == '"':
            in_str = True
        elif ch == "{":
            depth += 1
            seen_brace = True
        elif ch == "}":
            depth -= 1
            if seen_brace and depth == 0:
                pos += 1
                break
        elif ch == ";" and not seen_brace and depth == 0:
            pos += 1
            break
        pos += 1
    while pos < len(text) and text[pos] == "\n":
        pos += 1
    return start, pos


def extract(path, headers):
    text = read(path)
    blocks = []
    spans = []
    for h in headers:
        s, e = item_span(text, h)
        spans.append((s, e))
        blocks.append(text[s:e].rstrip("\n") + "\n")
    for s, e in sorted(spans, reverse=True):
        text = text[:s] + text[e:]
    return text, blocks


def mount_fixtures(module, extra_doc):
    p = src_path(module)
    s = read(p)
    anchor = re.search(r"#\[cfg\(test\)\]\n#\[path = \"🧪️tests/🔬️unit/🦀️\.rs\"\]\n(?:pub\(crate\) )?mod tests;", s)
    assert anchor, module
    new = '#[cfg(any(test, feature = "testkit"))]\n#[doc(hidden)]\n#[path = "🧰️fixtures/🦀️.rs"]\npub mod fixtures;\n\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'
    s = s[: anchor.start()] + new + s[anchor.end():]
    write(p, s)


def step_fixtures():
    # catalog: fixture_path, fixture
    tp = tests_path("📚️catalog")
    text, blocks = extract(tp, [r"^pub\(crate\) fn fixture_path\(\)", r"^pub\(crate\) fn fixture\(\)"])
    body = "".join(b.replace("pub(crate) fn", "pub fn").replace('"../../🧫️fixtures/📚️catalog/🔣️.json"', '"../../../../🧫️fixtures/📚️catalog/🔣️.json"') + "\n" for b in blocks)
    write(f"{M}/📚️catalog/🧰️fixtures/🦀️.rs", "//! 🧰️ The catalog file the proctor's own tests play and the catalog loaded from it, shared with the tests of the crates above (`testkit`).\n\nuse super::*;\n\n" + body.rstrip("\n") + "\n")
    text = text.replace("use crate::storage::tests::scratch;\n", "use super::fixtures::{fixture, fixture_path};\nuse crate::storage::fixtures::scratch;\n")
    write(tp, text)
    mount_fixtures("📚️catalog", "")

    # storage: NEXT_SCRATCH, Scratch (+ Drop impl), scratch
    tp = tests_path("🗄️storage")
    text, blocks = extract(tp, [r"^static NEXT_SCRATCH: AtomicU64", r"^pub\(crate\) struct Scratch\(pub PathBuf\);", r"^impl Drop for Scratch \{", r"^pub\(crate\) fn scratch\(label: &str\) -> Scratch"])
    body = "\n".join(b.replace("pub(crate) fn", "pub fn").replace("pub(crate) struct", "pub struct") for b in blocks)
    write(f"{M}/🗄️storage/🧰️fixtures/🦀️.rs", "//! 🧰️ A fresh scratch directory under the system temp dir, removed when dropped, shared with the tests of the crates above (`testkit`).\n\nuse std::path::PathBuf;\nuse std::sync::atomic::{AtomicU64, Ordering};\n\n" + body.rstrip("\n") + "\n")
    text = text.replace("use super::*;\n", "use super::fixtures::{scratch, Scratch};\nuse super::*;\n", 1)
    write(tp, text)
    mount_fixtures("🗄️storage", "")

    # actors
    tp = tests_path("🎭️actors")
    text, blocks = extract(tp, [r"^pub\(crate\) const TENANT: &str", r"^pub\(crate\) const ADA: &str", r"^pub\(crate\) const BOB: &str",
                                r"^pub\(crate\) fn envelope\(", r"^pub\(crate\) fn id\(", r"^pub\(crate\) fn admission\(", r"^pub\(crate\) fn perfect\("])
    body = "\n".join(b.replace("pub(crate) fn", "pub fn").replace("pub(crate) const", "pub const") for b in blocks)
    write(f"{M}/🎭️actors/🧰️fixtures/🦀️.rs", "//! 🧰️ The ids, envelopes, admission and perfect answers the proctor's own tests play, shared with the tests of the crates above (`testkit`).\n\nuse super::*;\nuse server::contract::HybridLogicalClock;\n\n" + body.rstrip("\n") + "\n")
    text = text.replace("use crate::catalog::tests::fixture;\n", "use super::fixtures::{admission, envelope, id, perfect, ADA, BOB, TENANT};\nuse teaching_proctor_base::catalog::fixtures::fixture;\n")
    write(tp, text)
    mount_fixtures("🎭️actors", "")

    # projections
    tp = tests_path("🔭️projections")
    text, blocks = extract(tp, [r"^pub\(crate\) async fn bus\(", r"^pub\(crate\) async fn submit\(", r"^fn command_id\(", r"^pub\(crate\) async fn answer\(", r"^pub\(crate\) async fn play\("])
    body = "\n".join(b.replace("pub(crate) async fn", "pub async fn") for b in blocks)
    write(f"{M}/🔭️projections/🧰️fixtures/🦀️.rs", "//! 🧰️ A command bus over a database with the proctor's deciders, and the perfect runs it plays, shared with the tests of the crates above (`testkit`).\n\nuse super::*;\nuse crate::actors::fixtures::{admission, envelope, perfect};\nuse crate::actors::{deciders, enrollment, ProctorDeciders};\nuse crate::storage::Database;\nuse quiz::{Challenge, Command};\nuse server::authority::{AuthorityDirectory, CommandBus};\nuse server::contract::{CommandOutcome, HybridLogicalClock, PolicyDecision};\n\n" + body.rstrip("\n") + "\n")
    text = text.replace("use super::*;\n", "use super::fixtures::{answer, bus, play, submit};\nuse super::*;\n", 1)
    write(tp, text)
    mount_fixtures("🔭️projections", "")
    print("fixtures extracted")


def rewrite_crate_paths(text, module):
    if module in BASE:
        text = text.replace("crate::catalog::tests::", "crate::catalog::fixtures::").replace("crate::storage::tests::", "crate::storage::fixtures::")
        return text
    base_pat = re.compile(r"\bcrate::(" + "|".join(BASE_NAMES) + r")\b")
    dom_pat = re.compile(r"\bcrate::(" + "|".join(DOMAIN_NAMES) + r")\b")
    text = base_pat.sub(lambda m: "teaching_proctor_base::" + m.group(1), text)
    text = text.replace("teaching_proctor_base::catalog::tests::", "teaching_proctor_base::catalog::fixtures::").replace("teaching_proctor_base::storage::tests::", "teaching_proctor_base::storage::fixtures::")
    if module in MAIN:
        text = dom_pat.sub(lambda m: "teaching_proctor_domain::" + m.group(1), text)
        text = text.replace("teaching_proctor_domain::actors::tests::", "teaching_proctor_domain::actors::fixtures::").replace("teaching_proctor_domain::projections::tests::", "teaching_proctor_domain::projections::fixtures::")
    else:
        text = text.replace("crate::actors::tests::", "crate::actors::fixtures::").replace("crate::projections::tests::", "crate::projections::fixtures::")
    return text


def step_rewrite():
    for module in BASE + DOMAIN + MAIN:
        for p in (src_path(module), tests_path(module), f"{M}/{module}/🧰️fixtures/🦀️.rs"):
            if not os.path.exists(p):
                continue
            s = read(p)
            n = rewrite_crate_paths(s, module)
            if n != s:
                write(p, n)
    # integration tests of the main crate
    repl = {
        f"{P}/🧪️tests/🔬️conformance/🦀️.rs": [("use proctor::storage::", "use teaching_proctor_base::storage::")],
        f"{P}/🧪️tests/🌐️end-to-end/🦀️.rs": [("use proctor::catalog::", "use teaching_proctor_base::catalog::"), ("use proctor::config::", "use teaching_proctor_base::config::")],
        f"{P}/🧪️tests/🌐️end-to-end/🛡️edge/🦀️.rs": [("proctor::actors::enrollment(", "teaching_proctor_domain::actors::enrollment(")],
        f"{P}/🧪️tests/🌐️end-to-end/🏆️challenge-sheets/🦀️.rs": [("proctor::storage::Database::open_read_only", "teaching_proctor_base::storage::Database::open_read_only"), ("proctor::actors::learner_key(", "teaching_proctor_domain::actors::learner_key(")],
    }
    for p, pairs in repl.items():
        s = read(p)
        for a, b in pairs:
            assert s.count(a) == 1, (p, a)
            s = s.replace(a, b)
        write(p, s)
    # fixtures directory of the end-to-end harness and the catalog fixture path
    print("paths rewritten")


def step_roots():
    base_root = '''//! 🧱️ The proctor's base: the SQLite storage roles, the quiz catalog, the process configuration, the crowd tally and the presence rules. Wiring only; the domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../🗄️storage/🦀️.rs"]
pub mod storage;

#[path = "../📚️catalog/🦀️.rs"]
pub mod catalog;

#[path = "../🎚️config/🦀️.rs"]
pub mod config;

#[path = "../👥️presence/🦀️.rs"]
pub mod presence;

#[path = "../👪️crowd/🦀️.rs"]
pub mod crowd;
'''
    domain_root = '''//! 🧠️ The proctor's domain: the learner and handle deciders, the read-model projections and the query handler. Wiring only; the domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../🎭️actors/🦀️.rs"]
pub mod actors;

#[path = "../🔭️projections/🦀️.rs"]
pub mod projections;

#[path = "../❓️queries/🦀️.rs"]
pub mod queries;
'''
    write(f"{M}/🧱️base/🦀️.rs", base_root)
    write(f"{M}/🧠️domain/🦀️.rs", domain_root)
    main_root = '''//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.

#![allow(async_fn_in_trait)]

#[path = "../../🔨️modules/🧩️instance/🦀️.rs"]
pub mod instance;

#[path = "../../🔨️modules/⌨️cli/🦀️.rs"]
pub mod cli;
'''
    write(f"{P}/📦️packages/🦀️rust/🦀️.rs", main_root)
    print("roots written")


if __name__ == "__main__":
    step = sys.argv[1]
    if step in ("fixtures", "all"): step_fixtures()
    if step in ("rewrite", "all"): step_rewrite()
    if step in ("roots", "all"): step_roots()
