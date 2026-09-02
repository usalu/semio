# Clear Filename Collisions Violating Emoji-Extension Statute

## A. Rust module files beside a canonical `🦀️.rs`

Determined per-file by reading the `#[path]`/`mod` declarations that reach each file, and by
checking `semanticDirectoryKinds`/`semanticPackageProjectionContracts` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.

### Fixed (3)

All three use a `#[path = "…"]` declaration in a sibling `🦀️.rs`, are NOT part of a
same-emoji slug collection, and are NOT referenced by the wgpu-renderer nested-cargo-package
projection ledger (verified by grep against the three authority/purity/projection fixtures under
`🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-*`).

- `🔌️plugin/🖥️host/⏳️imports.rs` → `🔌️plugin/🖥️host/⏳️imports/🦀️.rs`
- `🔌️plugin/🖥️host/⏳️runtime.rs` → `🔌️plugin/🖥️host/⏳️runtime/🦀️.rs`
- `🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` → `🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs`
  (emoji `🧵️` chosen to match the existing, taxonomy-registered precedent at
  `🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs`, a real "executor" directory already using this emoji.)

Repointed every live `#[path]` declaration and every doc-comment/script.ts backtick reference to
these three files (excluding one deliberate verbatim quotation inside
`⏳️runtime/🦀️.rs` that reproduces another file's exact prior wording in quotation marks — changing
it would falsify the quote) and excluding all historical ticket-workspace reports under
`.🧬semio/🦑️repo/🎫️tickets/…` (frozen per taxonomy's own
`referenceClosure.historicalDocumentEvidence` policy). Files touched:

- `🔌️plugin/🖥️host/🦀️.rs` (3 `#[path]` decls + 2 doc mentions)
- `🔌️plugin/🖥️host/🧵️shard/🦀️.rs` (1 `#[path]` decl + 2 doc mentions)
- `🔌️plugin/🖥️host/⚡️effects/🦀️.rs` (1 doc mention)
- `🔌️plugin/🖥️host/🌐host/🦀️.rs` (1 doc mention — a real, distinct sibling "network host" module)
- `🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` (2 doc mentions)
- `🛎️services/🦀️component.rs` (2 doc mentions)
- `🎭️actor/🦀️.rs` (1 doc mention)
- `📜️script.ts` (2 literal path constants for the executor audit)

Verified with `cargo check -p semio-framework-plugin-host --offline` (foreground): compiles with
the same 3 pre-existing `FromValue`/`ToValue` trait-bound errors in the crate's own `🦀️.rs`
(traced to `🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs`, confirmed via `git status` to be
concurrently modified by another session right now) both before and after my rename — my change
introduces zero new errors, and the compiler resolved all three new paths cleanly.

### Blocked (2) — leave alone

- `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎠️runtime.rs`
- `💻️os/🖥️host/🎠️activation.rs`

Both use the emoji `🎠️` only as a shared cross-file **prose "packet marker"**
("🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME") the two files' author chose to visually pair them —
not a claim on a taxonomy file-kind. Superficially this looks identical to the fixed trio, but
`taxonomy.json`'s `semanticDirectoryKinds.members-of-wgpu-target.memberNames` already registers
`"🎠️runtime"` as the wgpu package's *planned* canonical member-directory name, and both files are
listed by literal path/token in **three live, same-day-edited authority catalogs** for the
`nested-cargo-packages-v1` package-projection contract:

- `🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json`
- `…/🧪️nested-cargo-package-authority/🔣️.json` (byte-budget-pinned destination-path mappings)
- `…/🧪️nested-cargo-package-purity/🔣️.json`

These catalogs pin exact `sourcePath`/`relativePath`/`destinationPathBytes` values and
doc-comment-token counts for the *entire* wgpu-renderer package's move to a new package root
(`destinationManifestPath` swaps `📦️packages/🦀️rust` and `🎯️targets` — a separate, much larger,
already-tracked migration). Renaming either file now — without moving the whole package and
recomputing every byte-budget/token-census entry across all three catalogs — would desync a live
migration ledger last edited today (Sep 2) by, in all likelihood, a concurrent session. This is
exactly the "never repoint across module boundaries" risk called out in the ticket brief.
Left both files untouched.

## B. `🔌️nx-plugin.mjs` — blocked, live concurrent edit in progress

`nx.json` is `MM` (staged + unstaged changes) right now. The unstaged diff, observed unchanged
across two checks several minutes apart, is renaming the exact plugin entries this ticket item
targets:

```
- "plugin": "…/📚️library/🔌️nx-plugin.mjs"
+ "plugin": "…/📚️library/🟨️.mjs"
- "plugin": "…/🧪️test/🔌️nx-plugin.mjs"
+ "plugin": "…/🧪️test/🟨️.mjs"
```

`🧪️test/🔌️nx-plugin.mjs` is already staged Deleted with a new `🧪️test/🟨️.mjs` in place.
`📚️library/🟨️.mjs` is staged Added — but it currently implements a *different* plugin
(`@repo/emoji-project-json`) than `📚️library/🔌️nx-plugin.mjs`'s `@repo/policy-scripts-file`
(the `breach-*` folder-policy-lint project generator, confirmed live and depended on by
`📜️script.ts`, `🧬️schema/🔣️.json`, the vscode extension, and `taxonomy.json` itself — this is not
dead code). `📚️library/🔌️nx-plugin.mjs` itself is still present on disk, untouched, with no git
status entry. This is unambiguously someone else's in-progress rename of the very same file this
ticket flagged, mid-transition. Touching it now would race a live edit and very likely corrupt
the merge (the `@repo/policy-scripts-file` logic isn't in the new `🟨️.mjs` yet). Left entirely
untouched; re-checked once and the diff was identical, so it isn't actively re-writing every
second, but its target end-state (does `policy-scripts-file` get merged into `🟨️.mjs`, or does
`nx-plugin.mjs` become a second file needing a directory remedy of its own?) isn't mine to guess.

## C. `📋️mimes.csv` — fixed

`🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes.csv` → `🖼️assets/📃️list/📋️mimes/📊️.csv`.

`📊️.csv` (SPDX license list) is a genuinely different dataset already occupying the canonical
name in that directory; `📋️mimes.csv` is a second, distinct CSV (MIME registry). No code
references the file by path (only a descriptive, non-literal "`mimes.csv`-shaped" phrase in
`🔨️modules/🚪️io/🦀️.rs`'s doc comment, describing a generic row/header *shape*, not this file —
left as prose). No taxonomy/script.ts/nx.json references existed to repoint. Mechanical move,
zero downstream references, no build to verify against.

## D. Fixture `⚛️file*.tsx` variants — left alone (structural, not a breach)

`🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/📁️some/📁️folder/⚛️file{,_fixable,
_fixable_expected,_fixed,_invalid}.tsx`.

Read the consuming test,
`🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/…` → actually consumed by
`🔨️modules/💻️client/⌨️cli/🧪️component_test.go` (plus `🧩️vscode/📦️packages/🟦️typescript/🧪️extension.test.ts`
for the `.tsx`/`_invalid` pair). The Go test is the policy/breach/autofix engine's own test suite:
it reads `⚛️file_fixable.tsx`, runs `applyAutofixes`, and asserts the result equals
`⚛️file_fixable_expected.tsx`; separately it asserts `⚛️file_invalid.tsx` reports breaches and
`⚛️file_fixed.tsx` (plus the matching `🐍️/🔷️/🐹️` siblings) report zero. The suffixes ARE the test's
subject matter — they are synthetic, deliberately-violating example inputs used to test the very
tool that finds violations like the ones in this ticket. Renaming them to be "compliant" would
delete the fixture's reason to exist. Left untouched, same as icon/font collections.

## Verification

- `cargo check -p semio-framework-plugin-host --offline` (foreground): 3 pre-existing errors only
  (`FromValue`/`ToValue` trait bounds, traced to a concurrently-modified
  `🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs`), unchanged before/after my edit. No error
  names any of my touched files.
- `bun 📜️script.ts verify taxonomy` (no args): still fails before reaching the
  "expected report or enforce" gate — currently `SyntaxError: Export named 'createBoundedMailbox'
  not found in module '…/🎭️actor/📦️packages/🟦️typescript/🟦️.ts'`. This error's exact text changed
  between two checks made ~15 minutes apart (was `Cannot find module './📬️mailbox.ts'` the first
  time), and `git status` on that package shows `📬️mailbox.ts`/`🧵️shard-client.ts`/
  `🧵️shard-runtime.ts`/`🧵️turn-scheduler.ts` deleted and a new untracked `🟦️.ts` — this is a live,
  currently-in-progress refactor by another session, unrelated to anything this ticket touched
  (I made zero edits under `🎭️actor/`). Confirmed the same package-scoped test entry point
  (`bun ./📜️script.ts test quick` inside `📚️library/📦️packages/🟦️typescript`) hits the identical
  blocker, so there is currently no working entry point to get a live taxonomy report at all.
- Repo-wide grep for every old filename (`⏳️imports.rs`, `⏳️runtime.rs`, `🏃️executor.rs`,
  `📋️mimes.csv`) after excluding `node_modules/target/dist/storybook-static/.nx/.git/
  ♻️mit-bestand/.cursor/.🧬semio`: zero live hits remain outside `.🧬semio/🦑️repo/🎫️tickets/…`
  (frozen historical reports) and the one deliberate verbatim quote noted above.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️imports.rs` → `.../⏳️imports/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs` → `.../⏳️runtime/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` → `.../🧵️shard/🧵️executor/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` (repoint)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs` (repoint)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs` (repoint)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs` (repoint)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` (repoint)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs` (repoint)
- `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` (repoint)
- `📜️script.ts` (repoint 2 literal path constants)
- `🧰️framework/🔨️modules/🖼️assets/📃️list/📋️mimes.csv` → `.../📃️list/📋️mimes/📊️.csv`

Blocked, untouched, documented above: `🎠️runtime.rs` (wgpu), `🎠️activation.rs` (os/host),
`🔌️nx-plugin.mjs` (both locations), and the 5 `⚛️file*.tsx` fixtures.
