# P1 — Catalog & registry hygiene (2026-09-18)

Slice P1 against `📓️audit-plugins-artifacts.md` Q2/Q5. Four tasks: the `🗺️catalog.json` `directoryName` finding, the `🗟️artifacts` stub, the `💡️reasoning` crate-name drift, and the `🔋️energy` `[DEBUG]` ignore. No cargo build/check/test was run (host at its concurrency limit); `cargo metadata --no-deps` only.

---

## Task 1 — `🗺️catalog.json` `directoryName`: the audit finding is a MISREADING, the catalog is correct

### What the audit claimed

`📓️audit-plugins-artifacts.md` §Q2.2 and §Q5 P0.2 claim all 26 nested `*-extension-*` / `*-module-*` rows in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json`
have `directoryName` values that "cannot resolve on disk", with "~13 outright wrong emoji", and that the field should be rederived as `<parent>/🧩️extensions/<emoji><name>`.

### What `directoryName` actually is

It is the **flat installation/staging basename** a built module is materialized under — never a source path. Five independent proofs:

1. **The schema forbids a path.** `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🧬️schema/🔣️.json:9` — `InstallationDirectoryV1` is
   `^(?![📁📂📄])(?:\p{Extended_Pictographic}️…)[a-z0-9]+(?:-[a-z0-9]+)*$`.
   One emoji-prefixed kebab segment; `/` is not admissible. The deployment schema `$ref`s it at `📦️deployment/🧬️schema/🔣️.json:22`. The proposed `<parent>/🧩️extensions/<emoji><name>` shape is structurally impossible.
2. **The parser's own docstring says so.** `📦️deployment/🟦️.ts:40` — "📦️Validates the **hand-authored** deployment authority **without deriving any name from an ID**." There is no generator: a repo-wide grep for a writer of `🗺️catalog.json` across `**/*.ts` finds only readers (`📦️deployment/🟦️.ts:1`, `🧪️tests/✅️catalog-complete/🟦️.ts:68`, `🧪️tests/🪪️plugin-identity/🟦️.ts:12`, `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:202,781`, and the Rust identity tests).
3. **Every consumer joins it onto a build-output root, never onto `✏️s/🔌️plugins`.** All 15 call sites of `moduleDirectoryName()` / `moduleIdForDirectoryName()`:
   - `🔌️plugin/🏗️build/📥️installation/🟦️.ts:61,72,76,96` → `join(extensionOutRoot, …)`, `${MODULE_EXTENSION_ROUTE}/…`
   - `🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts:103`, `🏗️build/📦️materialization/🟦️.ts:115,162` → `join(pluginOutRoot, …)`
   - `🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:50` → `join(pluginModulesRoot(profile), …)`
   - `📊️size/🟦️.ts:139,143`, `📦️materialization/🟦️.ts:115` → read basenames back out of those same output roots
   - `📇️registry/🎮️playground/🧭️session/🟦️.ts:58`, `📽️projection/🟦️.ts:87` → build `moduleUrl`s
   Not one call resolves a source directory.
4. **The installation roots are FLAT, so the names must be globally emoji-unique.** `📦️deployment/🟦️.ts:48-50` throws on a duplicate sibling emoji across all 60 rows. Using disk basenames would collide immediately: `📐️cad` vs `📐️brep` (flow) vs `📐️spatial-shape` (cad), `🧱️block` vs `🧱️concrete` (process) vs `🧱️slabs` (sourcing), `🪵️sourcing` vs `🪵️wood` (process) vs `🪵️beams` (sourcing). The "wrong emoji" the audit flagged are exactly the deliberate disambiguations.
5. **All 60 rows materialize verbatim on this machine.** Every one of the 26 nested rows exists as a real directory under `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules/` — including `🪓️process-extension-wood`, `🧇️sourcing-module-slabs`, `⚖️imperative-extension-logic`, `🔷️cad-extension-spatial-shape`.

### Verdict

**`🗺️catalog.json` was not changed** — rewriting it as the audit proposed would have failed its own schema, collided on emoji, and broken every module URL and staging path in the OS. **`📓️audit-plugins-artifacts.md` Q2.2 / Q5 P0.2 should be retracted.**

### Verification script

`🐍️p1-catalog-verify.ts` (this folder) checks every row against the real contract: `InstallationDirectoryV1` pattern, `pluginId` pattern, global emoji uniqueness, identical order against the generated `🤖️generated/🔌️plugins.json`, existence of each row's `cratePath` on disk, and materialization under the four staging roots.

```
$ bun ".🧬semio/…/🐍️p1-catalog-verify.ts"
60 catalog rows, 60 materialized in a staging root, 0 not built on this machine.

34 of 60 directoryName values coincide with a ✏️s/🔌️plugins source basename (the 34 top-level
plugins); the 26 nested extension/module rows deliberately do not — their sources live at
<parent>/🧩️extensions/<emoji><name> and cannot be flattened into one emoji-unique installation
root without colliding.

[p1-catalog-verify] clean — every row is schema-valid, emoji-unique, ordered against the generated
registry and backed by a real crate on disk.
```

### Registry validators run

| Command (cwd `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`) | Exit | Tail |
|---|---|---|
| `bun ./📜️script.ts generate` | 0 | `plugin registry catalog refreshed (60 plugin crates, 65 playgrounds, 54 framework packages)` / `.vscode/launch.json regenerated` |
| `bun ./📜️script.ts check-generated` | 0 | `plugin registry generated catalog and launch bytes are fresh.` |
| `bun ./📜️script.ts native-catalog-selection-check` | 0 | `native-catalog-selection-oracle cases=23 positive=4 denied=19 authority=planning-only published=0` |
| `bun ./📜️script.ts plugin-root-ownership-check` | 0 | `registry-plugin-root-ownership cases=7 ajv=1 sqlite=7` |
| `bun ./📜️script.ts rust-taxonomy-mounts-check` | 0 | `registry-rust-mounts-oracle cases=9 ajv=1 compiler=9` |
| `bun ./📜️script.ts check` | **1** | 2299 lines of **pre-existing** plugin-taxonomy-tree violations |
| `bun ./📜️script.ts catalog-complete` | 1 | refuses to run without `--build-root <absolute fresh build root>` — needs a full fresh plugin build, not run (cargo budget) |
| `npx vitest run` on `🧪️tests/✅️catalog-complete/🟦️.ts` + `🧪️tests/🪪️plugin-identity/🟦️.ts` | 0 | `Test Files 2 passed (2) / Tests 21 passed (21)` |
| `bun ./📜️script.ts verify taxonomy report --scope "✏️s/🔌️plugins"` (repo root) | 1 | aborts before any finding: `error: frozen-coordinate-evidence-invalid: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json: document digest does not match registered bytes` |

`check`'s failure is a repo-wide structural baseline, not a regression from this slice: **all 34** plugins are cited, the top kinds being `907× … is not reachable from Cargo manifest`, `429× window X has unexpected child`, `259× surface X is missing 👥️presence/🧬️schema/`, `256× … 🎚️config/🧬️schema/`. Nothing in the output names a file this slice touched (the only two near-misses are `🔋️energy: … example "🏛️bestest-900FF"/"🏛️bestest-600FF" is not a valid emoji+VS16+kebab slug`, which are example-directory slugs, unrelated to the deleted test).

### Three stale registry-test counts fixed (uncovered by the above)

`wfc` was extracted into its own plugin **today** (ticket ☀️18 EXTRACT-WFC-PLUGIN), taking the catalog from 59 to 60 rows, but three hard-coded counts were not moved with it, leaving both registry suites red:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts:75` — `toHaveLength(59)` → `60`
- `…/🧪️tests/✅️catalog-complete/🟦️.ts:469,473,474` — `"…the 59 real manifests…"` → `60`, `manifestCount` `59` → `60`, `audit.order` `59` → `60`
- `…/🧪️tests/🪪️plugin-identity/🟦️.ts:56,57` — `"…for all 59 crates"` → `60`, `toHaveLength(59)` → `60`

Fixing the first of those unmasked two further stalenesses that the early `59` assertion had been short-circuiting:

- `…/🧪️tests/✅️catalog-complete/🟦️.ts:480` — `demonstrator.dependsOn` was pinned to `["cad","gis","procedural","process","puzzle","sourcing"]`, but `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:33-48` has declared the 8 `flow-extension-*` actors since commit `0b460ed19f` (2026-09-17), with a comment explaining the choice. Expectation updated to the declared manifest (the manifest is the authority — see `project-registry-depends-on-is-declared-runtime-metadata`).
- `…/🧪️tests/✅️catalog-complete/🟦️.ts:469,483` — the "known **17** missing source pairs" list still named `trinity`; `trinity` now has its descriptor pair, so the list is **16**. A gap closed, not a regression.

After these six edits both suites are green (21/21).

### Consumers of `directoryName`

Audited and unchanged — every one already resolves the current values correctly (list under "What `directoryName` actually is", point 3). The one place `directoryName` is compared to a *hand-written* expectation is `✏️s/🔌️plugins/💡️reasoning/🧪️tests/🔬️identity/🦀️.rs:51` and `✏️s/🔌️plugins/🪐️space/🧪️tests/🔬️interactive-job-catalog/🦀️.rs:212`, both of which read it from a per-plugin identity fixture (`💡️reasoning` → `💡️reasoning`; correct).

---

## Task 2 — `✏️s/🔌️plugins/🗟️artifacts` removed (plus two siblings), and two stale references fixed

`🗟️` (U+1F5DF) is a wrong-emoji twin of the real taxonomy directory `🗿️artifacts` (U+1F5FF), which 34 plugins use. Three `🗟️artifacts` shells existed, all with **zero files** and untracked by git:

```
$ find "✏️s/🔌️plugins/🗟️artifacts" -type f | wc -l   →  0
$ git ls-files "✏️s/🔌️plugins/🗟️artifacts"           →  (empty)
```

| Removed | mtime | Contents |
|---|---|---|
| `✏️s/🔌️plugins/🗟️artifacts` | Sep 2 18:17 | 8 empty dirs, deepest `◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️session` |
| `✏️s/🔌️plugins/🎬️sequence/🗟️artifacts` | Sep 12 11:12 | empty mirror of `🗿️artifacts/🎬️sequence` |
| `✏️s/🔌️plugins/📏️layout/🗟️artifacts` | Sep 12 11:10 | empty mirror, *and* a second `🎅️standards` typo branch beside `🏅️standards` |

Removed with `rm -r`; `find . -type d -name "🗟️*"` now returns 0. None was touched today, so no concurrent lane was mid-creation.

Two live references treated `🗟️artifacts` as a real path and pointed at nothing. The real directory is `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/…`:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-inference-law-source/🟦️.ts:13` — `familyRel` constant
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json:364` — nx `inputs` glob for `test-root-inference-law-source`

Both corrected to `🗿️artifacts`. `familyRel` is only used to key an in-memory virtual filesystem, so this is a correctness/legibility fix rather than a behaviour change; the `📋️project.json` glob, however, was a genuinely dead cache input that never matched a file.

`bun ./📜️script.ts test root-inference-law-source` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`): **7 pass / 1 fail**. The one failure is pre-existing and unrelated — `🧱️root-inference-law-source/🟦️.ts:242` requires the launch name `🧬schema💡️inference🧪source-ownership` to appear once in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`; `git show HEAD:` confirms **0** occurrences in both files at HEAD, so the seed has never carried that entry.

Repo-wide `grep "🗟️"` over `*.ts`/`*.json`/`*.rs` outside ticket folders now returns nothing.

---

## Task 3 — `💡️reasoning` crate renamed to `semio-s-plugin-reasoning`

Sibling plugin crates are `semio-s-plugin-<plugin id>` (`semio-s-plugin-note`, `semio-s-plugin-forms`, `semio-s-plugin-raster`, `semio-s-plugin-remodel`, …). `💡️reasoning` was still `semio-s-plugin-reasoning-mindmap`, a leftover of a pre-2026-08 "mindmap" name; its id, directory, artifact, component package (`semio:reasoning`) and artifact-kind prefix (`s.reasoning.`) are all `reasoning`.

| File:line | Change |
|---|---|
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml:4` | `name = "semio-s-plugin-reasoning-mindmap"` → `"semio-s-plugin-reasoning"` |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📋️project.json:2` | nx project `@semio-tech/reasoning-mindmap-plugin` → `@semio-tech/reasoning-plugin` (matches `@semio-tech/note-plugin` et al.) |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📜️script.ts:2,10,19` | router docstring + `runCargoTestBudgeted` + `describePluginComponent` package arg |
| `✏️s/🔌️plugins/💡️reasoning/🧫️fixtures/🧫️plugin-identity/🔣️.json:5` | `packageName` |
| `✏️s/🔌️plugins/💡️reasoning/🧪️tests/🔬️identity/🦀️.rs:25-29` | docstring: the crate name now *follows* the convention instead of being a documented divergence; the playground-variant pinning note kept |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:19977,19988,19999` | `semio_s_plugin_reasoning_mindmap_component.{js,d.ts,core.wasm}` → `semio_s_plugin_reasoning_component.*` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧫️fixtures/🔣️.json:209-210` | `alias`/`package` → `reasoning` / `semio-s-plugin-reasoning` (see caveat below) |
| `Cargo.lock:9629` | package name (ordering preserved: still between `semio-s-plugin-raster` and `semio-s-plugin-remodel`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` | regenerated: `packageName` → `semio-s-plugin-reasoning`, `wasmOut` → `semio_s_plugin_reasoning.wasm` |

Not touched, and why:

- **Root `Cargo.toml`** — carries only *paths* for this plugin (`:81`, `:235` members, `:302` the separately-and-correctly-named `semio-s-artifact-reasoning-wires`). No crate-name reference existed, so no edit was needed.
- **`🔒️dependencies.json`, `🧅️layering.json`** — reference `💡️reasoning` only by path; no crate name.
- **`.vscode/launch.json`** — regenerated byte-identical by `plugin-registry:generate`; its reasoning entries are keyed by playground variant `reasoning-wires`, not the crate name.
- **`reasoning.mindmap.fixture`** — a **wire schema id**, not a crate name. It is a committed data contract shared by `🧩️puzzle`, `♾️infinite/🎲️board` and the wgpu engine surfaces fixture. Deliberately left alone.

Verification:

```
$ cargo metadata --no-deps --format-version 1 > /dev/null
cargo metadata OK

$ bun nx show projects --json | python3 -c "…[p for p in d if 'reasoning' in p.lower()]…"
['test-s-plugins-reasoning-artifacts-wires-standards-1-subsets-any-76deae-📡️mutate-wires-1',
 '@semio-tech/reasoning-wires-rs', '@semio-tech/reasoning-js', '@semio-tech/reasoning-plugin']
mindmap present: []
```

Residual live (non-ticket) hits for `reasoning-mindmap` / `reasoning_mindmap` after the rename: only `🔬️identity/🦀️.rs:29`, which quotes the historical 2026-09-05 regression (`semio:reasoning-mindmap`) on purpose.

The Rust oracle `✏️s/🔌️plugins/💡️reasoning/🧪️tests/🔬️identity/🦀️.rs::plugin_identity_is_the_same_in_every_authority` is the authority that ties the fixture's `packageName` to the Cargo `name =` line and to the generated registry row; all three were moved together, but it could **not be executed** (no cargo runs permitted this session). The equivalent TypeScript join, `🧪️tests/🪪️plugin-identity/🟦️.ts`, was run and passes.

---

## Task 4 — `🔋️energy` `#[ignore = "[DEBUG] W3-1b probe"]` deleted

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🧪️tests/🔬️unit/🦀️.rs:316-345`, `fn debug_w3_1b_probe`. Deleted as a leftover probe, not demoted to a running test, because it asserts nothing:

- Its only inputs come from `SEMIO_ENERGY_PROBE`, and it `expect()`s that variable — it cannot run unattended at all.
- It contains **zero assertions**: it prints a `[DEBUG]` line and `std::fs::write(parts[2], …)` — writing a report to an arbitrary caller-supplied path.
- Everything about it is tagged `[DEBUG]`: the docstring, the ignore reason, the name, every `expect`/`panic` message.
- "W3-1b" is a wave of the historical ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` ("dispatched W3-1b energy physics vs EnergyPlus"), long finished. `SEMIO_ENERGY_PROBE` has no other reference anywhere in the repo.

The file went 426 → 396 lines. No dead code left behind: the helpers it called (`simulation_config`, `report_json`) are `pub fn` in the production module `🏛️bestest/🦀️.rs:418,517` with other callers (`🦀️.rs:439,513`, `🧪️sim/🧪️tests/🔬️unit/🦀️.rs:1166`), and the file's only import is `use super::*`. The neighbouring `#[ignore]`-free real oracles (`committed_bestest_fixtures_match_the_builders`, `regenerate_bestest_fixtures`) are untouched.

Per the constraint, no cargo was run for this crate.

---

## Findings handed back (not fixed here)

1. **`📓️audit-plugins-artifacts.md` Q2.2 and Q5 P0.2 are wrong** and should be retracted — see Task 1. The catalog needs no regeneration pass.
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧫️fixtures/🔣️.json` is wholesale stale.** Its `dependencies` array has 29 entries of which **26 aliases no longer exist** in the manifest it mirrors (`🧹️fixture-sweep/📦️packages/🦀️rust/Cargo.toml`, 56 deps — the crate moved from per-plugin crates to per-artifact crates), and its `moduleSha256`/`retainedSha256` are correspondingly stale. `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts:148` `deepEqual`s them, so that law is red for reasons predating this slice. Only the one dangling crate name was corrected; regenerating the fixture belongs to the fixture-sweep owner.
3. **`bun ./📜️script.ts check` (plugin registry) is red repo-wide** on 2299 plugin-taxonomy-tree violations across all 34 plugins — 907 unreachable-from-Cargo-manifest paths, 429 unexpected window children, 515 missing `🎚️config`/`👥️presence` schema dirs. A large pre-existing backlog with no current owner; worth its own ticket.
4. **`🧱️root-inference-law-source`** is missing its launch entry in `.vscode/🧩️launch.seed.jsonc` (`🧬schema💡️inference🧪source-ownership`), red at HEAD.
5. **`🔋️energy` example slugs `🏛️bestest-900FF` / `🏛️bestest-600FF`** fail the emoji+VS16+kebab slug rule (uppercase `FF`) — 2 of the 2299 above.
6. **`verify taxonomy` is unrunnable at HEAD.** It throws inside `planMoveReferenceAuthority` → `frozenCoordinateEvidenceCoordinates` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:6999`) because the frozen-coordinate evidence digest registered for `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` no longer matches that file's bytes. The fixture is clean in the working tree (committed `9b605a4550`, 2026-09-09; on-disk mtime Sep 5), so the registered digest, not the file, has drifted. It aborts the whole planner before a single taxonomy finding is produced, at any scope. Owner: whoever registered that evidence contract.

## Constraints honoured

No sub-agents; everything foreground. No git-modifying command (no commit/stash/checkout/reset/clean), no worktree. No `🗑️generated` folder touched or swept. No cargo build/check/test — only `cargo metadata --no-deps --format-version 1 > /dev/null`. `cd /Users/ueli/Documents/semio` explicit in every bash call, emoji paths quoted. Ticket `🎫️ticket.json` not edited, ticket not closed. Concurrent peers' edits (the wfc extraction, the `📽️projection`/`🔎️discovery` work, the `.vscode` MCP seed changes, the new tests being appended to `✅️catalog-complete/🟦️.ts`) were left alone.
