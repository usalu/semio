# TypeScript Reference Byte Contract 81

## Frozen Data Packet

This adds only the closed [schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-reference-byte-contract-81/🧬️schema/🔣️.json), concrete [records](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-reference-byte-contract-81/🔣️.json), and this report. Packet 75 and earlier reports/fixtures are unchanged. No controller or permanent script was added.

All 23 IDs, families, and expected statuses match the complete 75 roster: 21 manufactured cases (5 bound, 15 rejected, 1 unsupported) and 2 capture-only cases. These are **authored expectations, not executed semantic results**. The manufactured cases contain 44 actual UTF-8 TypeScript file strings; every files-plus-query subject is distinct. Case IDs do not construct sources or choose binding behavior.

The records explicitly include `nonexported-aggregate` and `nonexported-leaf`, both missing from report 80's table. Union, named alias, shadowed generic, optional literal/keyed property, wrong export, wrong provider, and malformed syntax each have concrete differing bytes. The default-import case also supplies a valid default type export, isolating import-form rejection from a missing-default-export error.

## Query and Future Reference Obligations

Manufactured query provenance is explicitly owner-relative under `virtual/vcs` or `virtual/gis`; its canonical aggregate/leaf locations are test inputs, not production descriptor path overrides. The query roles distinguish declared-surface binding, physical undeclared surface, and canonical capture. Literal and keyed declarations are separate closed shapes.

- `query.syntaxFactsComplete` is a **declared expectation to verify against actual compiler parse diagnostics before binding**. A future reference must parse the supplied bytes even when this flag is false; it must not short-circuit the `incomplete-syntax` case before observing its unfinished `RenameVcsMutation |;` union. A disagreement between declared completeness and observed diagnostics is not a successful binding.
- Preserve the authored AST union-member sequence and multiplicity before TypeChecker normalization. `First | Second` has two authored members resolving to the same canonical leaf; collapsing its semantic union to one type would invalidate `ambiguous-competing-alias`.
- Resolve the selected use-site symbol before resolving its named import alias. This distinguishes the shadowing generic parameter from the same-spelled import. Canonical provider path and exported symbol must both match; shape equivalence is insufficient.
- Expected reasons describe the contract failure, not arbitrary compiler diagnostic order. Canonical file/declaration export checks precede unresolved-import fallback, so the nonexported leaf remains `leaf-not-exported`. Optional properties and member multiplicity remain explicit checks.

## Actual Canonical Captures

`captured-vcs-six-member` names the real `VcsMutation` aggregate and its six original named imports. `captured-gis-two-member` names `GisTerrainMutation` and its two original named imports, preserving GIS's explicit `.ts` specifiers. All provider and descriptor paths remain repository-relative canonical paths; no fictional `Aggregate` or `leaf-i` remapping exists.

The packet includes 10 source files (3290 UTF-8 bytes), 8 matching direct-leaf descriptors (5307 bytes), and 8 explicit import edges. Every captured file carries its original bytes, byte count, and SHA-256. All eight captured descriptors omit both the TypeScript required surface and a `typescriptDeclaration` field. Capture queries therefore keep that binding null, with expected `capture-only` status and a separate `unsupported` binding disposition. Capturing exports is not descriptor-owned binding proof or a wider census.

## Schema Delta After Root's First Read

The final schema adds four structural tightenings; the record file is unchanged since its initial authoring:

1. Manufactured leaf paths also obey the common relative-path guard, rejecting parent/dot segments.
2. Captured relative import specifiers reject parent/dot and Compose segments as strings.
3. Declared case family must agree with its literal versus keyed declaration shape.
4. Captured ID/family pairs, source/descriptor/import counts, and expected member counts agree with the six-member VCS or two-member GIS record.

The first three corresponding schema counterexamples were observed as accepted before the tightening and rejected in the final check. No malformed path from a counterexample was opened.

## Data-Only Validation Receipt

At `2026-08-28T05:09:51.583Z`, existing Bun 1.3.14, Ajv 8.20.0 in strict mode, and jsonc-parser 3.2.0 completed the bounded check with exit 0 (1.09 seconds).

- JSON.parse and jsonc-parser agree on schema, records, and captured descriptor JSON; duplicate-property count is zero.
- Ajv accepts the complete record file and rejects 219 counterexamples: 201 unknown-property insertions covering every record object, plus 18 role/shape/roster/path/cardinality violations.
- All 23 roster triples match 75; all 21 manufactured files-plus-query subjects are distinct.
- UTF-8 round trips, unique per-case paths, all 18 captured file hashes/lengths, 8 textual import edges, and descriptor ownership/surface omissions agree. Guarded nofollow rereads matched the current canonical bytes at the receipt time.
- No TypeScript Program, TypeChecker, mutation harness, Nx task, harness child, native command, production write, or Compose traversal ran.

The first extended one-off validation expression stopped with `TypeError: undefined is not an object (evaluating 'const [name, mutate] of mutations')` because its counterexample list accidentally contained a sparse slot. That temporary expression was corrected before the complete receipt above; it changed no fixture or production input.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| Schema | 26981 | `22ae8e17bb0375c95fe80d9d42504b53a36c827aad170b50b08446bf630ab5e7` |
| Records | 56372 | `aca7b4396c3e0e0c66e95bbe329a1e04c0b0cbf86fac28dffff08e397a4baadc` |

Compiler parse diagnostics, alias resolution, semantic outcomes, and native behavior remain intentionally unverified. A future implementation must verify these assertions against the frozen input bytes; this data-validation receipt must not be used as a semantic pass.
