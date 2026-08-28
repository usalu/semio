# Source Admission Projection 51

## Proposed Single Projection

The shared admission API should project one sorted sequence of candidate records. Each record keeps the exact relative spelling supplied by its source, its physical kind, and a set of independent admission origins: `tracked`, `nonignored-untracked`, `ignored-generator`, and `explicit-ticket`. `tracked` retains stage-zero index identity rather than claiming all Git index stages are admitted. The projection must not normalize Unicode, collapse generator provenance into Git membership, read file content, classify mutations, or introduce another roots/skip-list traversal.

The proposed language-neutral schema and vectors are:

- [schema](./🧪️source-admission-51/🧬️schema/🔣️.json)
- [vectors](./🧪️source-admission-51/🔣️vectors.json)

The output order is raw UTF-8 byte order of the preserved relative spellings. `path` in the neutral matrix is the exact physical repository-relative `sourcePath`, not an NFC-normalized comparison key. A candidate enters only through one of the four declared origins. Duplicate paths merge origins into one output record; they never overwrite provenance. Stage-zero index evidence and matching generator-contract identities remain independent fields. Missing, opaque, unsafe, nonzero-stage, and cancellation conditions are explicit reports. A cancelled invocation returns no partial admitted output.

## Required Admission Cases

The matrix covers stage-zero Git only; nonignored untracked; declared ignored generator output; explicit ticket input; root-level and hidden authored files; canonical byte order without Unicode normalization; duplicate Git/generator provenance; undeclared ignored output; absent indexed paths; nonzero Git index stages; a legitimate source-to-destination move while the old index row remains; symlink/unsafe ancestry; case-folded opaque paths; and cancellation.

## Existing Candidate-Union Test Coverage Observed

The current full-inventory TypeScript tests exercise the phase sequence, scoped untracked enumeration, literal/decomposed Unicode scope comparison, symlink leaf handling, scoped symlink-ancestor fallback, ignored-generator admission, explicit-ticket admission, and cancellation at phase boundaries. They do this through `inventoryTaxonomy`, not through a separately exposed candidate-union contract. The current tests do not expose per-row origin accumulation, raw physical spelling as a public result, missing tracked diagnostics at admission, conflicted/nonzero stage handling, or cancellation inside a generator tree walk. The new matrix is therefore complementary, not a duplicate roots/skip-list authority.

## Observed Candidate-Union Quirks Requiring Agreement

1. `gitRows` filters stage `0`, but the current inventory silently omits a stage-zero path absent from the worktree rather than retaining its index identity as an explicit observed absence.
2. The current `Map<string, CandidatePath>` preserves only one row per path. It cannot expose that a generated ignored output is also Git-tracked, so generator provenance is lost.
3. `scopedGitPathspec` broadens an unsafe or indexed ancestor scope to the whole repository. The new projection needs an explicit `unsafe-scope` report or a separately frozen fallback contract; silently widening a lightweight caller is not equivalent to an admitted scoped result.
4. Tracked row order is inherited from Git while untracked and generator walks apply byte ordering. A shared projection must sort the final union once by raw UTF-8 bytes.
5. `ignoredGeneratorRows` has no cancellation parameter and walks output roots after an initial phase cancellation check. Cancellation needs to be observed inside every potentially large generator traversal.
6. Generator and ticket traversal use `lstat` on encountered nodes, but the existing full inventory later performs content reads. The projection must stop at physical candidate metadata and retain no-follow ancestry as its own authority.
7. The reviewed shared proposal requires absent tracked paths and nonzero index stages to be diagnostics. An absence is not automatically a structural error: the shared Git index can legitimately retain an old path after a physical move because this workflow does not update it. The matrix requires the old exact index identity to remain observable, the new nonignored physical path to be admitted, and conflicted stages to make the result ambiguous without selecting arbitrary stage content.
8. Current `generatorContractsForOutputPath` first calls `normalizeRelative(path)`. The public projection must retain the original `sourcePath` while deciding whether generator-root matching may use a distinct normalized comparison key; the v1 pure matrix deliberately keeps the physical spelling in both input and output and needs that matching-key detail frozen with the normalizer schema.

## Boundary

This is a ticket-only proposal. It does not claim that current normalizer behavior meets these cases, does not inspect a real `compose` segment, and does not change N, D, the package index, schemas, or callers.

## Preserved Initial Proposal

The initial eight-case draft remains unchanged at [initial schema](./🧪️source-admission-51/🧬️schema/🔣️.json) and [initial vectors](./🧪️source-admission-51/🔣️vectors.json). It is historical preparation evidence only: it does not meet the now-accepted closed-output shape.

## Closed V1 Projection Proposal

The accepted normalizer footprint now has a ticket-only closed result proposal at [v1 schema](./🧪️source-admission-51/🧬️schema/🔣source-admission-v1.json), [v1 vectors](./🧪️source-admission-51/🔣source-admission-v1.json), and [reference validator](./🧪️source-admission-51/📜️script.ts). Every result has `schemaVersion: 1`, `scope`, `status`, byte-sorted observations, and byte-sorted diagnostics. Each observation preserves exact physical `sourcePath`, physical kind, worktree mode or absence, explicit-directory status, ordered admission origins, all available Git index entries, and generator-output identities.

The pure projector accepts only observed candidate facts plus configured opaque prefixes and schema-declared generator output roots. It derives ignored-generator provenance from matching declared output roots rather than accepting it from a candidate label. The future I/O wrapper remains separately responsible for Git enumeration, lexical-before-filesystem exclusion, no-follow ancestry, cancellation polling, and constructing those candidate facts.

The twelve v1 cases cover hidden/root/build-named authored candidates; byte order; overlapping tracked/generated/ticket provenance; explicit-ticket directories; symlink leaf versus unsafe ancestor; nested case-folded `compose` rejection before observation; all conflicted index stages; NFC-only scope comparison without physical spelling rewrite; stale stage-zero old-path absence plus a new nonignored worktree path; cancellation inside a generator walk; untrusted generator claims; and missing stage-zero identity.

## Corrected V2 Projection Proposal

The active ticket controller now dispatches `reference` and `subject` modes to the corrected v2 [schema](./🧪️source-admission-51/🧪️v2/🧬️schema/🔣.json), [vectors](./🧪️source-admission-51/🧪️v2/🔣.json), and [reference controller](./🧪️source-admission-51/🧪️v2/📜️script.ts). V1 files remain historical evidence and are not rewritten.

V2 merges all rows having the exact same physical `sourcePath`; it unions unique Git tuples and non-generator origins, derives generator outputs by NFC comparison against declared roots while retaining raw candidate and root spelling, and rejects contradictory physical facts instead of selecting a row. Generator `ignored-generator` provenance is valid only when a matching declared ignored output root exists. The projector treats configured opaque paths only as repository-root prefixes; the standalone `compose` prefix additionally matches any case-folded segment. It validates safe slash-relative paths before policy, retains normalized ancestor-or-descendant scope membership without rewriting source spelling, and rejects nonregular, unobserved-without-cause, unsafe, opaque, conflicted, no-provenance, and incoherent index facts.

The v2 output remains the closed result shape requested for the normalizer: `schemaVersion`, `scope`, `complete|rejected` status, UTF-8-byte-sorted observations, and UTF-8-byte-sorted diagnostics. Each observation contains exact `sourcePath`, observed kind, mode or absence, explicit-directory status, canonical ordered origins, complete index tuples, and complete generator root identities. Its fourteen projection cases cover duplicate union, duplicate contradictions, valid and invalid ignored-generator claims, repository-root versus nested opaque behavior, case-folded nested compose, Unicode ancestor/descendant scope, invalid traversal/absolute/backslash/control paths, stale-index membership deltas, nonzero conflict tuples, pure cancellation, NFC generator root matching, index-without-origin, inconsistent physical facts, unsupported gitlink mode, and unobserved candidates. Three malformed candidate records are independently rejected by the same Ajv schema.

The current `subject` mode guards the canonical normalizer path with ancestor no-follow checks and a pre-import source hash, then dynamically imports it and requires an exported `projectTaxonomySourceAdmission`. On 2026-08-27, the guarded actual-N run reached the module at SHA-256 `f4e19f2977808e5241089cc27c2982a9d6beec2000a97f3dd22481afb4186ec4` and failed exactly because that export is absent; this is the required source/subject red, not a passing API check. It does not claim any filesystem or Git admission proof.

## Executed Neutral Validation

The first v1 validator run was intentionally red at setup because the installed Ajv default instance does not load draft 2020-12. The ticket validator now imports Ajv's draft-2020 entry point. The scoped Bun/Nx command `bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️source-admission-51/📜️script.ts reference` is green for fourteen v2 projections and three schema rejections. This is Ajv document validation plus a ticket-owned reference projection only; it is not third-party algorithm parity, actual normalizer validation, Git enumeration, filesystem candidate walking, or a mutation-source census. The corresponding `subject` command is currently red at the missing actual-normalizer export described above.
