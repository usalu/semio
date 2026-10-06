#!/usr/bin/env python3
"""⛓️ S5-STORE wave DAG (found by `viewer_head_tests`, 2026-10-05): `MutationDag::insert` classified an envelope as ready when
each dependency was merely KNOWN — applied, or buffered and itself still pending. A chain whose middle arrives first
(b waits for a; c names b) applied c before b: a replica that received `[Commit, Branch, …]` before the operations the
commit covers folded the `Branch` without its `Commit` and refused the ingest ("branch names unknown checkpoint"). The rule
is now the one `advance_ready_one` always used: an envelope is ready when every dependency is APPLIED.

Edits `📡️replication/🔗️causal/🦀️.rs` (rule + doc) and its unit tests (one law, one stale sentence). Anchors must occur
exactly once. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-dag-pending-rule.py [--check]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
CAUSAL = ROOT / "🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs"
TESTS = ROOT / "🧰️framework/🔨️modules/📡️replication/🔗️causal/🧪️tests/🔬️unit/🦀️.rs"

CAUSAL_EDITS = [
    (
        "rule",
        "        let pending = envelope.dependencies.iter().any(|dependency| !self.applied.iter().any(|applied| applied == &dependency.0) && !self.envelopes.iter().any(|known| known.mutation_id.0 == dependency.0));\n",
        "        let pending = envelope.dependencies.iter().any(|dependency| !self.applied.iter().any(|applied| applied == &dependency.0));\n",
    ),
    (
        "doc",
        "    /// `Err(Duplicate)` if it's already buffered as pending, `Pending` if any dependency is wholly\n    /// unknown to this dag, else `Applied`.\n",
        "    /// `Err(Duplicate)` if it's already buffered as pending, `Pending` while any dependency is not\n    /// applied — unknown to this dag, or buffered and itself still pending — else `Applied`.\n",
    ),
]
TEST_EDITS = [
    (
        "sentence",
        "    /// tier. True topological orders never hit the `insert`-classification quirk documented on\n    /// `MutationDag` above (every dependency is already `applied`, not merely known, by induction).\n",
        "    /// tier. In a true topological order every dependency is already `applied` when its dependent arrives.\n",
    ),
    (
        "law",
        "    #[test]\n    fn topological_order_a_b_c_d_converges() {\n",
        "    /// ⛓️ A chain whose middle arrives first (b, c, a) stays in causal order: c names b, which is buffered but still waits\n"
        "    /// for a, so c waits too, and a's arrival releases b and then c. Taking a merely buffered dependency for a met one\n"
        "    /// applied c before b — a `Branch` folded before the `Commit` it names.\n"
        "    #[test]\n"
        "    fn a_dependency_that_is_buffered_but_pending_keeps_its_dependent_pending() {\n"
        "        let mut dag = MutationDag::new();\n"
        '        assert_eq!(dag.insert(sample_envelope("b", vec!["a"])).unwrap(), InsertResult::Pending);\n'
        '        assert_eq!(dag.insert(sample_envelope("c", vec!["b"])).unwrap(), InsertResult::Pending);\n'
        '        assert_eq!(dag.insert(sample_envelope("a", vec![])).unwrap(), InsertResult::Applied);\n'
        "        let drained: Vec<String> = take_applied(&mut dag).into_iter().map(|envelope| envelope.mutation_id.0).collect();\n"
        '        assert_eq!(drained, vec!["a".to_string(), "b".to_string(), "c".to_string()]);\n'
        "        retire_dag_shell(&mut dag);\n"
        "    }\n"
        "\n"
        "    #[test]\n    fn topological_order_a_b_c_d_converges() {\n",
    ),
]


def plan(path, edits):
    """🧮️ The edited text of `path` and the names of its pending edits."""
    text = path.read_text(encoding="utf-8")
    pending = []
    for name, old, new in edits:
        if new in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"{path.parent.name}: anchor `{name}` occurs {text.count(old)} times (expected 1): re-derive the wave")
        text = text.replace(old, new)
        pending.append(name)
    return text, pending


def main():
    causal, causal_pending = plan(CAUSAL, CAUSAL_EDITS)
    tests, tests_pending = plan(TESTS, TEST_EDITS)
    pending = [f"causal:{name}" for name in causal_pending] + [f"tests:{name}" for name in tests_pending]
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--check" in sys.argv[1:] or not pending:
        return
    CAUSAL.write_text(causal, encoding="utf-8")
    TESTS.write_text(tests, encoding="utf-8")
    print("applied")


main()
