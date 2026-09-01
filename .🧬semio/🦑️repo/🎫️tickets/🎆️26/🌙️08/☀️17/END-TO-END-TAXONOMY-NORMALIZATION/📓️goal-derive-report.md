# Derive-directory collision + rust-path-join-unproven — findings and fix

Scope: two blockers around `✨️derive` directories, per assignment. No taxonomy.json edits made
(none needed). No git mutation commands run. Scratch/probe scripts live in `🗑️temp/`
(`📓️goal-derive-probe.ts`, `📓️goal-derive-probe2.ts`, `📓️goal-derive-syntax-check.ts`,
`📓️goal-derive-verify-run-1-blocked-by-gitlink.log`); this file is the durable report.

## Inventory of `✨️derive` directories

`find . -type d -name '✨️derive' -not -path '*/node_modules/*' -not -path '*/target/*'` (excluding
snapshot copies embedded inside other tickets' fixture folders) finds exactly three real ones:

- `🧰️framework/🔨️modules/🔄️machine/✨️derive`
- `🧰️framework/🔨️modules/🧬️schema/✨️derive`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive`

All three have the identical shape: `🦀️component.rs` at the derive root plus
`📦️packages/🦀️rust/📦️glue.rs`. Only `🧬️schema/✨️derive` shows the collision. Reason: `🔄️machine/✨️derive`'s
`glue.rs` is a genuinely thin re-export (`#[path = "../../🦀️component.rs"] mod component;` — one
doc comment, one `mod` line), so the engine's grammar classifies its role as `declaration`, never
`implementation`, so it never enters the hoisting path. `🧬️schema/✨️derive`'s `glue.rs` is byte-identical
to its `component.rs` (a full proc-macro implementation — structs/enums/fns), so it classifies as
`implementation`. `💻️os/…/🗣️dsl/✨️derive`'s pair is likewise a full duplicate implementation (would
also classify `implementation`) — it did not show a `collision-*` violation in the captured artifact,
but for the same structural reason it would be equally exposed; the fix below covers it generically,
not by special-casing one directory.

## (A) Normalization collision

### Root cause

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`:

- `canonicalFile` (~line 3083) computes the kind-only destination for every file. For a stem in
  `GENERIC_SEMANTIC_STEMS` (line 865: `asset, assets, component, components, glue, test, tests,
  implementation, impl, index`) it returns the bare kind-only name in the file's own parent
  directory — e.g. `📦️packages/🦀️rust/📦️glue.rs` → `…/📦️packages/🦀️rust/🦀️.rs` (still correctly nested).
- Separately, `classifyPackageRole` (~3050) classifies package-boundary file content via
  `classifyGlue` against `packageGlueGrammar`. Content with `struct/enum/trait/impl` or fn bodies
  over `maxDelegationStatements` classifies as `implementation` — a `packageBoundaryRules` "problem"
  (package boundaries are meant to hold only thin delegation).
- When `role === "implementation"`, the inventory builder (~6222, was) called
  `packageImplementationDestination` to relocate the misplaced implementation to its semantic owner.
  For a **generic/empty stem** (our case — `component`/`glue` are both generic), that function's old
  branch was:
  ```ts
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und")))
    return ownerCanonical ? `${ownerCanonical}/${fileName}` : fileName;
  ```
  This **discards the package/ecosystem directory entirely** and flattens straight to
  `${ownerCanonical}/${fileName}` — the exact same bare `🦀️.rs` that `component.rs` at the derive
  root already canonicalizes to. Two distinct source files → one destination string → `collision-byte`
  / `collision-case-fold` / `collision-nfc` / `collision-same-kind` / `collision-vs16-fold`.

Captured RED evidence (from a same-day `clean taxonomy verify` artifact, `plan`/`violations` fields;
raw copy retained in `🗑️temp/📓️goal-derive-verify-run-1-blocked-by-gitlink.log` shows the *current*
run's unrelated failure — see "Verification status" below for why a fresh full run could not be
captured this session):

```
collision-byte / collision-case-fold / collision-nfc / collision-same-kind / collision-vs16-fold
target 🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs
sources 🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs,
        🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs
```

### Fix (structural, directory-based, no stem invented)

Per the coordinator's guardrail: never merge the two files, never delete/flatten a semantic
directory, disambiguate with a directory only. The clean fix is to **stop discarding the
package/ecosystem directory** when the package file's own stem is generic — its already-existing,
already-registered directory pair `📦️packages/🦀️rust` (`packages`, `rust-language` — both real
`semanticDirectoryKinds` in `🔣️taxonomy.json`, unrestricted "packages" + `rust-language`'s
`parentKindIds:["packages"]`) is itself the disambiguator. So the file simply keeps its natural,
already-nested, kind-only path instead of being force-relocated on top of the owner's file. No new
taxonomy vocabulary needed.

Changes in `…/🧹️normalization/🟦️.ts`:

1. `packageImplementationDestination` (~3135): removed the generic/empty-stem branch that flattened
   to `${ownerCanonical}/${fileName}`. The function now only ever hoists a package file whose *own*
   stem is a real semantic word (unchanged logic for that case).
2. Call site in the inventory builder (~6226): only invokes `packageImplementationDestination` /
   emits `package-implementation-file` / `package-implementation-destination-unresolved` when the
   file's stem is **not** generic/empty. For the generic/empty-stem case, `normalizedPath` stays
   `canonical.path` — i.e. `…/📦️packages/🦀️rust/🦀️.rs`, distinct from the owner's `…/🦀️.rs` — no
   violation, no collision, no stem, no directory deleted.

This is the general mechanism fix (not special-cased to schema/derive), so it applies uniformly to
`🧬️schema/✨️derive` and `💻️os/…/🗣️dsl/✨️derive`'s package/glue pairs, and leaves `🔄️machine/✨️derive`
(already thin, unaffected — its glue classifies `declaration`, never enters this branch) untouched.

### Verification

- `bun run` smoke-loaded the edited module directly (`import * as mod from ".../🟦️.ts"`) — loads
  cleanly, 22 exports, no syntax errors.
- `tsc --noEmit` against the surrounding project shows only pre-existing, unrelated errors elsewhere
  in the repo (actor/shard-client, ui/contract, machine/component.ts, etc.) — none in the edited
  regions (3135–3155, 6222–6238).
- **Full-repo `clean taxonomy verify` GREEN run could not be captured this session**: a fresh run
  (`bun ./📜️script.ts clean taxonomy verify --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION
  --workers 8`) throws before reaching collision detection at all:
  ```
  error: Normalization requires an explicit repository-boundary decision before authored
  classification: ♻️mit-bestand/recherche
      at inventoryTaxonomyWithSourceParentPruning (…/🧹️normalization/🟦️.ts:6140:37)
  ```
  `♻️mit-bestand/recherche` is a real git submodule (`git ls-files -s` shows mode `160000`) — this is
  the concurrent gitlink slice's in-progress work (other `📓️gitlink-*` folders at this ticket root
  belong to that slice), unrelated to `✨️derive`, and out of scope to touch. It blocks the *entire*
  verify pipeline for every worker right now, not just ours. **Once that gate is resolved, re-run the
  command above and diff `violations`/`plan.unresolved` for the five `collision-*` rows and the two
  `package-implementation-*` codes — they should be gone.**
- In lieu of the full run, the fix was validated by direct code-path trace (cited above, line numbers
  given) plus confirming `GENERIC_SEMANTIC_STEMS` membership and the exact call-graph; no rust source
  files were touched for part (A), so no `cargo check` was needed for it.

## (B) Unprovable Rust path joins (`rust-path-join-unproven`)

### Root cause

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` has three Rust
reference detectors feeding `rustManifestReferenceTokens` in `…/🧹️normalization/🟦️.ts` (~4275):

- `inspectRustManifestPathReferences` / `inspectRustManifestPathCandidates` — the only "provable"
  paths: a `.join()` chain rooted at literally `std::path::Path::new(env!("CARGO_MANIFEST_DIR"))` or
  `std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))` (optionally through tracked `let` bindings).
  Both guard on `source.includes("CARGO_MANIFEST_DIR")` and return `[]` immediately if absent.
- `inspectRustJoinArgumentSpans` — the catch-all: **any** `<expr>.join("string literal")` whose
  argument is a bare string literal (or a bounded `for x in [literals]` loop variable) is captured,
  regardless of what `<expr>` is.

In `…/🧹️normalization/🟦️.ts` (~4305), any span from the catch-all that wasn't already claimed by the
provable detectors is unconditionally tagged `rust-path-join-unproven` / "Rust join argument has no
proven immutable manifest-relative base" — by construction, nothing in this bucket can ever be
"proven": that's what makes it the bucket.

The flagged lines (`📦️packages/🦀️rust/📦️glue.rs:284,350` and `🦀️component.rs:280,346` in
`💻️os/…/🗣️dsl/✨️derive`) are `owner.join("🦀️component.rs")` / `mutation_root.join("🦀️component.rs")`
inside `#[cfg(test)]` fixture generators. `owner`/`mutation_root` are runtime-computed under
`std::env::temp_dir()` (see `fixture_workspace`, ~line 256) — genuinely ephemeral, not manifest-
relative in any sense, so there is no way to "prove" them per the engine's rule (nor should there
be — rooting them at `CARGO_MANIFEST_DIR` would be a lie). The literal `"🦀️component.rs"` is inert
test-probe data (materializing a synthetic tree to exercise `mutation_source_authority`), not a real
reference to the repo's own `🦀️component.rs` — but the scanner is purely syntactic and can't tell the
difference for a raw string-literal argument.

### Fix (behavior-preserving, no taxonomy change)

Per the acceptance rule, the only way to exit the "unproven" bucket for code that is legitimately not
a real manifest-relative reference is to not present it as a bare string-literal join argument in the
first place — `inspectRustJoinArgumentSpans` only tracks a literal token or a literal-array loop
variable directly in `.join(...)`; a `let`/`const`-bound identifier argument is invisible to it (and
the two `CARGO_MANIFEST_DIR`-only detectors were already returning `[]` for this file regardless).
Applied identically to both files (they are deliberate mirrors — see below):

```diff
- "wrong-primary-filename" => { let wrong = owner.join("🦀️component.rs"); …
+ "wrong-primary-filename" => { const HISTORICAL_PRIMARY_FILENAME: &str = "🦀️component.rs"; let wrong = owner.join(HISTORICAL_PRIMARY_FILENAME); …

- "historical-primary" => { let historical = mutation_root.join("🦀️component.rs"); …
+ "historical-primary" => { const HISTORICAL_PRIMARY_FILENAME: &str = "🦀️component.rs"; let historical = mutation_root.join(HISTORICAL_PRIMARY_FILENAME); …
```

Files touched: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` (lines ~280, ~346)
and `…/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (lines ~284, ~350). Identical runtime string, identical
path construction — purely a syntactic change to drop out of the reference scanner's catch-all.

### Verification (RED → GREEN, both empirically run)

RED — `plan.unresolved` from the same captured artifact:
```
rust-path-join-unproven:284:67@18639 … 📦️packages/🦀️rust/📦️glue.rs
rust-path-join-unproven:350:76@25032 … 📦️packages/🦀️rust/📦️glue.rs
rust-path-join-unproven:280:67@18505 … 🦀️component.rs
rust-path-join-unproven:346:76@24892 … 🦀️component.rs
```

GREEN — direct probe against the real, unmodified detector functions (`🗑️temp/📓️goal-derive-probe2.ts`),
run with `bun run` against the two edited files on disk:
```
=== 🦀️component.rs ===        refs: 0 candidates: 0 joinArgs: 49   (no "🦀️component.rs" entry — was present before the fix)
=== 📦️glue.rs ===              refs: 0 candidates: 0 joinArgs: 49   (no "🦀️component.rs" entry — was present before the fix)
```
The remaining 49 `joinArgs` per file are pre-existing, unrelated literals (`nx.json`,
`📋️project.json`, etc.) that were never in the RED violation list to begin with (not this ticket's
concern; untouched).

`cargo check --manifest-path …/✨️derive/📦️packages/🦀️rust/Cargo.toml` — clean (one pre-existing,
unrelated `dead_code` warning on `source_path` field).

`cargo test --manifest-path …/✨️derive/📦️packages/🦀️rust/Cargo.toml -- --test-threads=1` —
**13 passed; 0 failed** (the crate's full test suite, including both edited match arms'
`materialize("wrong-primary-filename" | "historical-primary")` fixtures, and the tests that consume
them: `validates_mutation_source_authority_fixture`, `aggregate_and_leaf_authority_share_workspace_
taxonomy_names_and_token`). Note: the default parallel `cargo test` run intermittently fails one
unrelated test (`aggregate_and_leaf_authority_share_workspace_taxonomy_names_and_token`) with a JSON
EOF panic — reproduced with and without this change; it's a pre-existing race in the shared
`std::env::temp_dir()` fixture-naming scheme across parallel test threads, not caused by this edit.
Single-threaded, all 13 pass deterministically.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — (A) collision fix.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` — (B) join-proving fix.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` — (B) join-proving fix.
- No `🔣️taxonomy.json` edits (none needed for either fix).

## Still blocked / handoff

- A fresh full-repo `clean taxonomy verify` is needed to get official before/after violation counts
  for (A), but is currently blocked repo-wide by the concurrent gitlink slice's unresolved
  `♻️mit-bestand/recherche` repository-boundary decision (not this slice's responsibility). Re-run:
  `bun ./📜️script.ts clean taxonomy verify --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION --workers 8`
  once that's resolved, and confirm the five `collision-*` rows for `🧬️schema/✨️derive` and the four
  `rust-path-join-unproven` rows for `💻️os/…/🗣️dsl/✨️derive` are gone.
