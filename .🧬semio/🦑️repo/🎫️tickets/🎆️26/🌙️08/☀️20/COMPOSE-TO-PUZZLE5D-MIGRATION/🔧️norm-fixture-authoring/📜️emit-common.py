#!/usr/bin/env python3
"""🧪️ Shared emitter plumbing for the four norm fixture trees (en1996 / en1997 / iso16757 / en1995).

Every VALUE, every case name, every prose fragment and every bespoke assertion lives in the
per-artifact table files beside this one; this module only writes the bytes to disk. It is ticket
scratch (26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION), NOT a permanent script.
"""

import json
import os

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", ".."))
NORM = os.path.join(REPO, "✏️s", "\U0001f50c️plugins", "\U0001f4d5️norm", "\U0001f5ff️artifacts")

MUT_JSON = "\U0001f9a0️mutation/\U0001f523️component.json"
DIFF_JSON = "\U0001f53a️diff/\U0001f523️component.json"
OUTCOME_JSON = "\U0001f3af️outcome/\U0001f523️component.json"
BEFORE_JSON = "\U0001f4f8️snapshot/⬅️before/\U0001f523️component.json"
AFTER_JSON = "\U0001f4f8️snapshot/➡️after/\U0001f523️component.json"
TEST_RS = "\U0001f980️component.rs"
TESTS_DIR = "\U0001f9ea️tests"
MUTATIONS_DIR = "\U0001f9ec️mutations"


def mutations_root(artifact_dir):
    return os.path.join(NORM, artifact_dir, "\U0001f3c5️standards", "\U0001f516️1", "\U0001fa86️subsets", "✳️any", "\U0001f9ec️schema", MUTATIONS_DIR)


def resolve_leaf_dir(artifact_dir, kind):
    """🔎️ Resolves the on-disk leaf directory from its kebab kind, so no emoji/variation-selector
    byte sequence is ever retyped by hand. Fails loudly if the match is not unique."""
    root = mutations_root(artifact_dir)
    hits = [entry for entry in os.listdir(root) if entry.endswith(kind) and os.path.isdir(os.path.join(root, entry))]
    if len(hits) != 1:
        raise SystemExit("leaf %r in %s resolved to %r" % (kind, artifact_dir, hits))
    return hits[0]


def dump(path, payload):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")


def write_text(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def emit_case(artifact_dir, leaf_dir, case, *, before, after, mutation, diff, outcome, rust):
    root = os.path.join(mutations_root(artifact_dir), leaf_dir, TESTS_DIR, case)
    dump(os.path.join(root, BEFORE_JSON), before)
    dump(os.path.join(root, AFTER_JSON), after)
    dump(os.path.join(root, MUT_JSON), mutation)
    dump(os.path.join(root, DIFF_JSON), diff)
    dump(os.path.join(root, OUTCOME_JSON), outcome)
    write_text(os.path.join(root, TEST_RS), rust)


def render_test(*, artifact, types, kind, case, header_note, tests):
    """🦀️ Assembles one fixture test file from seven already-written test bodies."""
    snapshot, mutation_ty, diff_ty = types
    lines = [
        "//! \U0001f9ea️ `%s` fixture — `%s`." % (kind, case),
        "//!",
        "//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket",
        "//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/",
        "//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.",
        "//!",
    ]
    for note in header_note:
        lines.append("//! " + note)
    lines += [
        "",
        "use crate::artifacts::%s::{%s, %s, %s};" % (artifact, diff_ty, mutation_ty, snapshot),
        "",
        "const BEFORE: &str = include_str!(\"\U0001f4f8️snapshot/⬅️before/\U0001f523️component.json\");",
        "const AFTER: &str = include_str!(\"\U0001f4f8️snapshot/➡️after/\U0001f523️component.json\");",
        "const MUTATION: &str = include_str!(\"\U0001f9a0️mutation/\U0001f523️component.json\");",
        "const DIFF: &str = include_str!(\"\U0001f53a️diff/\U0001f523️component.json\");",
        "const OUTCOME: &str = include_str!(\"\U0001f3af️outcome/\U0001f523️component.json\");",
        "",
        "fn before() -> %s {" % snapshot,
        "    serde_json::from_str(BEFORE).expect(\"the committed before-snapshot decodes\")",
        "}",
        "fn expected_after() -> %s {" % snapshot,
        "    serde_json::from_str(AFTER).expect(\"the committed after-snapshot decodes\")",
        "}",
        "fn mutation() -> %s {" % mutation_ty,
        "    serde_json::from_str(MUTATION).expect(\"the committed `%s` payload decodes\")" % kind,
        "}",
        "fn built_outcome() -> protocol::MutationOutcome<%s> {" % diff_ty,
        "    <%s as protocol::Mutation<%s>>::diff(&mutation(), &before())" % (mutation_ty, snapshot),
        "}",
    ]
    for body in tests:
        lines.append("")
        lines.append(body.rstrip("\n"))
    return "\n".join(lines) + "\n"
