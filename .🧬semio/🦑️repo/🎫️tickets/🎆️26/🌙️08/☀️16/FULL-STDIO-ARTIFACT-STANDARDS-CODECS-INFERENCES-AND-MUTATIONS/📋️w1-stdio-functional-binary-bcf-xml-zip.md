# W1 Stdio Functional Binary, BCF, XML, and ZIP Audit

Date: 2026-08-16  
Scope: source-first repairs for the Binary raw extent inference, BCF 2.1 fixture honesty, XML 1.0 grammar/validity/fixture behavior, and ZIP 2.0 ordering, grammar, fixture, and default behavior.

## Execution boundary

This pass was intentionally static. No Cargo or Nx command was run because the serialized runtime/compiler lane had not released its lock. Existing user and agent changes were preserved. The report records implementation evidence and remaining gates without claiming unrun tests.

Reference baseline: `📋️w1-stdio-full-lib-baseline.md` (3436 passed, 75 failed, 3 ignored at capture time). The relevant baseline failures were Binary empty-extent default behavior, BCF stale deterministic fixture bytes, XML canonical/grammar/validity/fixture paths, and ZIP order/grammar/fixture/default paths.

## Findings

### P0 — none

No source change in this bounded lane introduced a data-loss, silent-diagnostic, or unsafe acceptance path. The BCF fixture mismatch remains visible and is not masked.

### P1 — repaired

1. Binary raw extent defaults now derive from the authoritative empty snapshot through `compute_binary_extent`, so the default has the same semantic result as a cold computation rather than fabricating a zero digest or unrelated placeholder.
2. ZIP decode preserves central-directory entry order. The decoder no longer alphabetically reorders entries, which is required for source-order retention and deterministic semantic reconstruction. Canonical encoding retains its separate deterministic name ordering policy.
3. ZIP `ZipEntries::default()` now computes the inference from `ZipSnapshot::default()`, keeping default and cold inference behavior equivalent.
4. ZIP snapshot DSL and pack codecs now use the native ZIP codec. They parse/emit the native ZIP payload inside the Semio envelope, validate the artifact marker, and preserve the archive’s typed/opaque records instead of routing through a generic record serializer.
5. XML snapshot canonical printing now uses XML-compliant double-quoted declarations, identifiers, entity values, and attributes, and emits empty elements as `/>` without a preceding space. External identifiers and entity values use attribute escaping. This matches the committed XML DSL fixture while retaining comments, processing instructions, CDATA, and unknown structure in the snapshot model.

### P1 — still open and explicit

1. The committed BCF 2.1 `.dsl.semio` and `.pack.semio` fixtures encode an older ZIP policy: all entries are stored, flags/versions/timestamps differ from the current deterministic writer, and therefore exact print/encode fixture laws cannot pass until fixtures are regenerated from the accepted canonical encoder under the serialized test lock. `example.bcf` is an empty placeholder and is not treated as evidence of a valid BCF package.
2. ZIP’s inference grammar declares `OCTET+`, but the shared grammar terminal vocabulary has no `OCTET` terminal. This is a grammar-source defect if the broader inference grammar gate parses that file; it must be resolved with the repository’s established token vocabulary or a ZIP-local serialized inference grammar, not by weakening the gate. No shared grammar framework was changed in this lane.

### P2 — follow-up gates

1. Run the targeted fixture regeneration/verification after the runtime lock is released. Do not hand-edit binary fixture hex or relax byte-identity assertions.
2. Re-run the XML grammar, XML valid-negative, XML snapshot retention, ZIP grammar/order/fixture/default, Binary extent, and BCF codec/fixture laws on the clean integration target. Record diagnostics and checksums in the umbrella ticket.

## Files changed in this lane

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/📏extent/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃entries/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`

BCF source was inspected for fixture behavior but was not changed in this pass; its encoder correctly routes through the current ZIP codec and therefore exposes the stale fixture policy rather than hiding it.

## Static verification

The following read-only checks completed successfully (exit 0):

```text
rustfmt --edition 2021 --check [the five changed Rust files]
git diff --check [the five changed Rust files]
```

No targeted test result is asserted here. The pending test names are the Binary raw `inference_default_law`/extent tests; ZIP rich-order, inference-default, grammar, fixture-honesty, snapshot, and mutation laws; XML committed-facet, grammar, operations grammar, fixture-honesty, retention, and valid-negative laws; and BCF grammar, fixture-honesty, and typed round-trip laws.
