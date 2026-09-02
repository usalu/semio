# Remaining Stdio Aggregates Reconciled

Date: 2026-09-01

## Outcome

The live tree already contained the bulk aggregate-to-leaf migration from commit `67fb4216b2` and ticket `26/08/29/S-END-TO-END`. The current stdio mutation vocabulary has 87 descriptor-owning aggregates and 913 mutation leaves. A full static audit now reports zero descriptor, leaf, aggregate-variant, semantic-kind, or approved-verb failures.

The reconciliation found and corrected the following residual migration defects:

1. IFC2x3 `UpsertInstance` used `verb: "update"`, outside the task's 41 approved verbs. It now uses `verb: "set"` while retaining `kind: "upsert-instance"`, matching the established `SetSnapshot` and `DemoteShapeRepresentation` rule that the approved semantic verb and stable kind need not be identical.
2. OBJ `InsertTexcoord`, `RemoveTexcoord`, and `SetTexcoord` leaf directories used `*-tex-coord` although their derived semantic kinds are `*-texcoord`. The directories, descriptor owners, and aggregate `#[path]` attributes now use the exact `<emoji><semanticKind>` spelling.
3. The MP3 oracle declaration gate still expected the deleted `NoMutation` variant. Its variant list now starts with `SetSnapshot` and matches the production `KINDS` list.
4. The external XLSX subject harness rejected the committed `no-mutation` scenario. It now materializes that scenario as `SetSnapshot(base.clone())`, as the other migrated external harnesses do.
5. One AP214 CC1 test still constructed `SetFileSchema` with the retired struct-variant syntax, and one MP3 leaf test still matched `SetSnapshot` as a struct variant. Both now use their newtype leaf forms.
6. The semio envelope's internal tests and its separate external harness still constructed migrated nested model, document, CAD, image, video, and audio variants as struct variants. They now wrap the appropriate leaf payloads; the external harness uses fully qualified leaf paths.

No aggregate semantics were re-derived. The migrated leaves continue to delegate to the lifted aggregate functions.

## Validation

The primary Python audit checked every leaf descriptor against its filesystem and Rust aggregate:

```text
descriptors=913 aggregates=87 failures=0
```

Checks performed:

- descriptor owner equals the exact leaf directory;
- directory basename equals `<emoji><semanticKind>`;
- kind contains a hyphen;
- kind equals the derive-compatible kebab-case aggregate variant;
- leaf `🦀️.rs` exists;
- aggregate `🦀️.rs` exists and contains the variant as a newtype variant;
- leaf `SEMANTICS.kind` equals the descriptor kind;
- leaf verb belongs to the task's 41-verb allowlist.

An independent `jq` pass over the language-neutral JSON descriptors produced the same result:

```text
jq_descriptors=913 failures=0
```

The stale-reference and implementation sweeps produced:

```text
handwritten_mutation_impls=0
stale_enum_code_refs=0
stale_variant_literals=0
stale_non_drawing_struct_variant_forms=0
```

`git diff --check` passed for the reconciled paths.

## Runtime Verification State

The requested repository gates were attempted but concurrent, unrelated workspace changes prevented either command from reaching stdio compilation:

1. `bun nx run '@semio-tech/stdio-plugin:test-quick' --args='--offline'` stopped in taxonomy loading because the tracked JCO output `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs` is missing.
2. The latest `cargo build -p semio-s-plugin-stdio --offline` reached dependency compilation, then stopped because `semio-framework-ui-contract` cannot read its concurrently renamed `🦀️.rs` module and `semio-framework-graph` cannot generate while the same JCO output and its generated graph registry are missing.
3. Earlier exact Cargo attempts during the same reconciliation observed transient concurrent states: three workspace roots, then taxonomy source-path duplication plus the missing graph registry.

No `E0046` was emitted in these attempts; the compiler did not reach the stdio crate in the current live tree. The later migration ticket's `📓️status.md`, `📓️summary.md`, and `📓️verify-semantics.md` record a green stdio compile and semantic comparison immediately after the bulk migration landed. A fresh runtime gate remains required after the concurrent workspace/taxonomy work settles.

The encompassing external-oracle ticket remains open because the current workspace cannot complete the runtime gate and because the ticket covers work broader than this migration reconciliation.
