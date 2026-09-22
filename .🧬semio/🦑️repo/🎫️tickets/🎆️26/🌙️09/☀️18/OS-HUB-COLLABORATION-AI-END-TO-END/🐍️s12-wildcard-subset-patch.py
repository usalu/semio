#!/usr/bin/env python3
"""🩹️ Anchored, idempotent: a dialect's SUBSET on the creation-catalog wire admits the canonical
wildcard `*` (`SubsetId::ANY`), which the identity grammar rejects (ticket 26/09/18, S12 §2.3)."""
import pathlib
ROOT = pathlib.Path("/Users/ueli/Documents/semio")
OS = ROOT / "🧰️framework/🛍️products/💻️os/🟦️.ts"
text = OS.read_text()

ANCHOR = '''function workerWireCreationIdentityV1(value: unknown): string | null {
  return typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value) ? value : null;
}
'''
INSERT = '''
/** 🪆️ A DIALECT component on the same wire. `subset` is the one component whose canonical value is
 * not an identity at all: `SubsetId::ANY` is the literal `*` (`🚪️io/🧬️schema/🦀️.rs`), the
 * unconstrained base subset every standard carries and the subset of every kind this repository
 * ships. Validating it with `workerWireCreationIdentityV1` therefore rejected EVERY catalog the hub
 * can send: the main thread threw on decode, the shell kept `null`, and `createArtifact`'s kind
 * chooser rendered disabled with zero options on every space of every hub (measured 2026-09-22 on
 * hub 7651, whose own `GET /spaces/{id}/artifact-creations` answers two kinds, both `subset: "*"`). */
function workerWireDialectComponentV1(value: unknown): string | null {
  return value === "*" ? value : workerWireCreationIdentityV1(value);
}
'''
OLD = '''      artifactKind = workerWireCreationIdentityV1(dialect.artifactKind),
      standard = workerWireCreationIdentityV1(dialect.standard),
      subset = workerWireCreationIdentityV1(dialect.subset),'''
NEW = '''      artifactKind = workerWireCreationIdentityV1(dialect.artifactKind),
      standard = workerWireDialectComponentV1(dialect.standard),
      subset = workerWireDialectComponentV1(dialect.subset),'''

changed = 0
if "workerWireDialectComponentV1" not in text:
    assert text.count(ANCHOR) == 1, "identity anchor not unique"
    text = text.replace(ANCHOR, ANCHOR + INSERT)
    changed += 1
if NEW not in text:
    assert text.count(OLD) == 1, "dialect anchor not unique"
    text = text.replace(OLD, NEW)
    changed += 1
OS.write_text(text)
print(f"edits applied: {changed}")
