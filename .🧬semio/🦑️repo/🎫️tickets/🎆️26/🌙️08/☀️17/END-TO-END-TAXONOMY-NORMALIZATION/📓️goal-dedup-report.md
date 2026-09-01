# Proc-Macro Glue Deduplication — Schema Derive & DSL Derive

## 1. Duplication confirmed (real `diff` output)

**`🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs` (288 lines) vs
`🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (283 lines)** — `diff` showed
exactly one hunk, a rustfmt line-wrap of a single `match` arm (`other => { return Err(...) }` vs
`other => return Err(...),`). Everything else byte-identical. Confirms the finding: same source,
formatting-only drift.

**`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` (2121 lines) vs
`…/📦️packages/🦀️rust/📦️glue.rs` (2235 lines)** — `diff` showed the two were **not** merely
rustfmt-apart. `git log` proved why: commit `bb06c41f7` (2026-08-28 11:09:46, after `component.rs`'s
last touching commit `f7b265d58` at 09:20:58) touched **only** `glue.rs`, applying an in-progress
async-convention pass (loop-based `Vec`/`Map` (de)serialization instead of `.iter().map().collect()`,
plus `// 🚫️async: …` provenance comments and one extra `#[cfg(test)]` module
`mandatory_mutation_descriptor_tests`) that had not yet been mirrored into `component.rs`. `glue.rs`
was the strictly-more-current copy; `component.rs` was stale. Every `component.rs`-only difference
was either a relative `include_str!`/`#[path]` prefix (`glue.rs` sits one directory deeper) or logic
`glue.rs` had already superseded (e.g. `component.rs`'s `let source = …local_file();` — assigns an
`Option` with no error path — vs `glue.rs`'s already-fixed `.ok_or_else(...)` handling). No content
existed in `component.rs` that was missing from `glue.rs`.

**Resolution:** used `glue.rs`'s content (the current one) as the merge source for the new owner
`component.rs`, not the reverse — preserves the in-flight async-convention work instead of discarding it.

## 2. De-duplication performed

Both crates now follow the `🔄️machine`/`⏳️async` exemplar shape exactly: owner `🦀️component.rs` holds
all implementation (helpers, structs, and one `pub fn expand_<name>(...)` per macro); `📦️glue.rs`
(crate root, `path = "📦️glue.rs"` unchanged in `Cargo.toml`) holds **only** the
`#[proc_macro_derive(...)]`/`#[proc_macro]`-tagged entry points, each a one-line delegation into
`component::expand_<name>(input)`. Every macro name, attribute list (`attributes(...)`), and doc
comment was carried over verbatim onto the glue-side wrapper; internal function bodies were moved
unmodified (no behavior changes) — only the wrapping/entry-point layer changed.

- **Schema derive**: 1 macro (`ArtifactSchema`). `component.rs` 288→273 lines, `glue.rs` 283→21 lines.
- **DSL derive**: 9 macros (`MutationLeaf`, `DslRecord`, `DslArtifact`, `DslDiff`, `DslScalar`,
  `DslOps`, `DslEnum`, `Mutations`, `CompositeMutation`). `component.rs` 2121→2195 lines (net +74:
  absorbed `glue.rs`'s already-more-current logic, expected), `glue.rs` 2235→92 lines.

**Lines removed (net, both crates combined): 4927 → 2581 = 2346 lines.**
One source of truth remains per crate: `component.rs`.

### Companion test fix (in-scope, not a taxonomy.json edit)

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🦀️component.rs`
(`facade_exports_match_registered_macros`) asserted `registered(component.rs) ==
registered(glue.rs) == expected("registeredDerives")` — an assertion that only held **because** of
the duplication bug (both files carried identical `#[proc_macro_derive]` tags). Updated it to assert
the corrected invariant: `registered(component.rs)` is now empty (implementation only, no crate-root
tags) and `registered(glue.rs) == expected("registeredDerives")`. This is the DSL-derive crate's own
test suite, in scope for this dedup; no taxonomy.json touched.

## 3. Suppression restored (undone)

Found, in the **uncommitted working tree** (not in any of the last 5 commits — `git log -S` confirmed
`genericPackageStem` never appears in tracked history), a bundled diff against HEAD that mixed two
unrelated changes in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`:

1. A genuine, good fix: `TICKET_GENERATED_OUTPUT_DIRECTORY = "🗑️temp"`, excluded from
   `explicitTicketRows`, and the default `planArtifactPath` nested under it — parks generated
   `📊️taxonomy-plan/verify` JSON so it can't re-enter its own reference closure. **Left this in place.**
2. The suppression to undo: a `genericPackageStem` guard added to
   `inventoryTaxonomyWithSourceParentPruning` (skipping the `packageImplementationDestination` hoist
   for generic/empty-stem implementation files) plus deletion of the matching generic-stem branch
   inside `packageImplementationDestination` itself.

Reverted only (2): removed the `genericPackageStem` const/guard and its comment, changed
`if (role === "implementation" && !genericPackageStem)` back to `if (role === "implementation")`, and
restored `if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und"))) return ownerCanonical
? \`${ownerCanonical}/${fileName}\` : fileName;` at the top of `packageImplementationDestination`.
Verified via `git diff` afterward: only the `🗑️temp` hunks remain against HEAD; the
`genericPackageStem`/`packageImplementationDestination` region is now byte-identical to HEAD.

**Suppression restored: YES (undone).**

## 4. Measurement — blocked by an unrelated, already-committed bug (not mine, not fixable in scope)

Per the coordinator's note, `--plan` was pointed at
`$T/🗑️temp/goal-dedup-plan.json` (inside the repo, inside the already-excluded `🗑️temp` dir — the
tool hard-rejects any `--plan`/`--cancel-file`/`--resume` path outside the repository, so the
coordinator's literal `/tmp/...` suggestion does not work against this build; the in-repo `🗑️temp`
convention is what actually satisfies "never re-enters the reference closure").

```
B=$(git rev-parse HEAD)   # bb06c41f73f0122fbed315b7487428b976f99921
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/🧬️schema" --baseline "$B" \
  --plan ".../🗑️temp/goal-dedup-plan.json" --workers 8
```

Ran twice (reproducible both times): crashes during the repo-wide incoming-reference-candidate scan
(this phase is **not** scope-limited — `--scope` only restricts which sources get *moved*, not what's
scanned as a possible reference) with:

```
error: frozen-coordinate-evidence-invalid: .🧬semio/…/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w2-schema-api.md:
coordinate is not one non-opaque repository-relative source path
  at frozenMarkdownCoordinateEvidenceCoordinates (…normalization/🟦️.ts:6756)
```

Traced this to `🔣️taxonomy.json`'s `frozenCoordinateEvidenceContracts` entry
`frozen-ticket-clean-architecture-layering-enforcement-w2-schema-api` — a different ticket's file,
registered by a different, still-in-progress feature slice (see this ticket's own
`📓️w-frozen-coordinate-evidence-authority.md`, which self-reports "regression packet is still in
progress"). The whole-document `sha256` still matches the live file exactly (confirmed with
`shasum -a 256`), so the file was **not** edited out from under anyone; this is a standing bug in the
newly-committed (`f7b265d58`, same day, before my baseline) coordinate-span validator itself. It blocks
`clean taxonomy plan` for **any** scope right now, not just mine, and fixing it would mean editing
`🔣️taxonomy.json` (forbidden — owned by another worker) or someone else's in-flight validator. Retried
once; identical crash both times (not a transient race — no uncommitted changes to the offending file).

**Before/after `moves=`/`unresolved=`/collision rows: could not be captured live** because of the
above. In place of the live run, verified the fix structurally by extracting and running the exact
classifier the tool uses (`classifyGlue`'s `analyzer === "rust"` branch, from
`🧹️normalization/🟦️.ts`) directly against both new `📦️glue.rs` files:

```
🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs => thin-delegation
🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs => thin-delegation
```

Both were previously `"implementation"` (matched `/\b(?:struct|enum|trait|union|impl)\b/` — the full
duplicated body). Since `packageImplementationDestination`'s owner-root hoist only fires for
`role === "implementation"`, and `glue.rs` no longer qualifies, it no longer computes the same
kind-only owner-root destination as `component.rs` (whose own stem, `component`, is also in
`GENERIC_SEMANTIC_STEMS`, so it independently canonicalizes to the same owner-root kind-only path
either way). **This removes the collision at its root cause — two sources no longer map to one
destination — independent of the frozen-coordinate-evidence crash**, which is a pre-existing,
unrelated environmental blocker on the measurement command itself, not on the fix.

Reference numbers from the task brief (BEFORE dedup: `moves=3 unresolved=124`, WITH suppression in
place: `moves=5 unresolved=201`) could not be reproduced against a live run for the reason above; no
numbers are fabricated here.

## 5. Build/test proof

```
cargo check -p semio-framework-schema-derive          # Finished, 0 errors
cargo test  -p semio-framework-schema-derive           # 0 tests (crate has none), doctests 0 — ok
cargo check -p semio-framework-os-kernel-dsl-derive    # Finished, 1 pre-existing warning (dead_code
                                                        # on MutationAggregateSourceAuthority.source_path,
                                                        # present in HEAD before this change too)
cargo test  -p semio-framework-os-kernel-dsl-derive    # 13 passed; 0 failed (incl. the updated
                                                        # facade_exports_match_registered_macros)
cargo check -p semio-framework-plugin                  # downstream `#[derive(ArtifactSchema)]`
                                                        # consumer (git grep -l 'derive(ArtifactSchema)')
                                                        # — Finished, 219 pre-existing warnings, 0 errors
cargo test  -p semio-framework-plugin --no-run          # test binary compiles cleanly, 0 errors
```

`semio-s-plugin-lowpoly` (also an `ArtifactSchema` consumer) was tried first but its dependency graph
transitively hits `semio-s-plugin-stdio`, which fails to compile for an unrelated reason (a missing
PDF-schema-mutation file, `insert_page/🦀️component.rs`, belonging to a different in-flight ticket's
plugin work) — switched to `semio-framework-plugin` directly, which compiles standalone.

## Summary

- Duplication: confirmed real (schema: rustfmt-only; dsl: `glue.rs` was ahead, `component.rs` stale).
- Dedup: done, exemplar shape, 4927→2581 lines, one source of truth per crate.
- Suppression: restored (undone) by hand; `🗑️temp` parking fix (unrelated, good) left in place.
- Measurement: blocked by a pre-existing, unrelated `taxonomy.json`-registered frozen-evidence bug
  (not caused by, and not fixable within, this slice); collision removal proven structurally instead
  via the exact classifier regex.
- Builds/tests: green for both derive crates and a real downstream `ArtifactSchema` consumer.
