# Taxonomy Violations Audit — Procedural (2026-09-10)

Read-only audit. Reproduced `bun nx run @semio-tech/plugin-registry:check`'s taxonomy phase
directly (`bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts check`) — the
cheapest way, since the `check` target has no `dependsOn` and the taxonomy phase
(`validateTaxonomyTree`, `📜️script.ts:1212`) is a pure filesystem walk that never shells out to
`rustc` (that only happens in the unrelated `rust-taxonomy-mounts-check` fixture-test subcommand).
No cargo build, no dev server. Raw run: 19,448 lines, exit 1. Procedural-only lines saved at
`🗑️generated/taxonomy-procedural.txt` (640 lines, to be deleted at ticket close per the generated-
folder rule).

## 0. Numbers vs `📓️rebuild-2026-09-09.md`

Live run today: **640** procedural violations out of **19,447** repo-wide across the same **33**
plugins — one day's drift from the prior report's `639`/`19,175` (some file changed between runs;
not investigated, out of scope). Full per-plugin breakdown, largest to smallest
(`grep -oE '^  - [^:]+:' 🗑️generated/… | sort | uniq -c`):

| plugin | count | plugin | count | plugin | count |
|---|---|---|---|---|---|
| 🗄️stdio | 6272 | 🔱️trinity | 299 | 📏️layout | 181 |
| 📕️norm | 2646 | 🗒️note | 298 | 🖍️draw | 179 |
| 🔋️energy | 1592 | 🌍️gis | 284 | 📋️forms | 179 |
| 🏛️architect | 1193 | 🎥️shooting | 267 | 🕸️dag | 172 |
| 🧩️puzzle | 856 | **🌊️flow** | **240** | ✒️writer | 165 |
| 🧱️block | 708 | 🪐️space | 209 | 🖨️raster | 159 |
| **🌀️procedural** | **640** | 💠️lowpoly | 209 | 🎬️sequence | 148 |
| 🏗️fem | 581 | 📐️cad | 202 | 📜️imperative | 146 |
| 📸️remodel | 445 | 🎞️animate | 192 | 💡️reasoning | 146 |
| | | 🏭️process | 186 | 🪵️sourcing | 143 |
| | | ➗️mathematical | 184 | 📖️playbook | 133 |
| | | | | 🌿️vcs | 126 |
| | | | | 🎪️demonstrator | 67 |

Sum = 19,447. Procedural is 3.3% of the repo-wide total, 7th of 33 by count. `stdio` (6,272, 32.3%
of the repo total) and `flow` (240, this end-to-end packet's other plugin) are the two requested
comparison points — flow's mix is the same two dominant classes as procedural (see §3), just fewer
artifacts.

## 1. Why it's a hard error, not a warning

`CheckScript.run` (`…📇️registry/📜️script.ts:3403-3411`) severity-splits on
`🔣️taxonomy.json`'s declared maturity for `pluginAreas` (`["✏️s/🔌️plugins"]`):
`areas["✏️s/🔌️plugins"] = "clean"` (confirmed live in the file). The comment at `:3399-3402` says the
finalization flip to hard-failure is deliberately "a one-word edit in `🔣️taxonomy.json`" — that flip
has already happened, so every one of the 19,447 findings across all 33 plugins now `process.exit(1)`
instead of `console.warn`. Reverting that one field to `"legacy"`/`"mixed"` would turn the whole gate
green again (policy-level, not procedural-specific) — noted as the fastest unblock, not recommended
here since it re-hides the debt the flip was meant to surface.

## 2. Procedural's 640, classified

Procedural has exactly 3 artifacts under `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/`:
`🧊️generation3d`, `🌀️generation2d`, `🧩️assembly` — plus one plugin-root-level Rust file
(`🫀️core/🖼️semantic-ui/🦀️.rs`) and one plugin-root required dir (`🎮️commands/`).

| # | class | count | example path | policy rule (file:line) | proposed fix | mechanical? |
|---|---|---|---|---|---|---|
| 1 | leaf not reachable from Cargo manifest | 573 | `🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs is not reachable from Cargo manifest 📦️packages/🦀️rust/Cargo.toml` | `validateRustTaxonomyMounts`, `…📇️registry/📜️script.ts:1175`: every taxonomy leaf `.rs` under the plugin must be reachable from the crate's `Cargo.toml` root via the real Rust module graph (`inspectRustModuleGraph`, `:1170`) | Add the missing `mod`/`#[path]` declarations up the parent-module chain to `📦️packages/🦀️rust/…` so every `component.rs` is actually compiled — one edit per broken chain link, not per leaf (573 leaves likely collapse to a handful of missing `mod` statements at the standard/subset/mode roots) | **hand-authored** — no generator inserts `mod` statements; `rust-taxonomy-mounts-check` only tests the *detector*, not a fixer |
| 2 | artifact missing `🧬️schema/` | 3 | `artifact "🧩️assembly" is missing 🧬️schema/` | `validateTaxonomyTree`, `:1239-1241` (component walk over `TAXONOMY_ARTIFACT_COMPONENTS`) | Create `🗿️artifacts/<artifact>/🧬️schema/` with the 5 `TAXONOMY_SCHEMA_FILENAMES` formats | **hand-authored** — `new artifact` (`📜️script.ts:17377` `newScaffoldArtifactTree`) only writes the artifact-root `component.rs`/TS leaf, not `🧬️schema/`/`🚪️io/`/`⚙️engine/`/`📚️examples/` |
| 3 | artifact missing `🚪️io/` | 3 | `artifact "🧊️generation3d" is missing 🚪️io/` | `:1231-1233` | Create `🗿️artifacts/<artifact>/🚪️io/` + leaf | hand-authored (same generator gap) |
| 4 | artifact missing `⚙️engine/` | 3 | `artifact "🌀️generation2d" is missing ⚙️engine/` | `:1338-1340` | Create `🗿️artifacts/<artifact>/⚙️engine/` | hand-authored |
| 5 | artifact missing `📚️examples/` | 3 | `artifact "🧩️assembly" is missing 📚️examples/` | `:1343-1345` | Create `🗿️artifacts/<artifact>/📚️examples/<emoji-slug>/` with `component.rs`+TS+assets+tests | hand-authored |
| 6 | window has unexpected child `🧪️tests` | 14 | `window "✳️any/👁️viewer/👁️view/👁️preview" has unexpected child "🧪️tests" (expected one of 🍱️panes, 🪀️widgets, 🪛️utilities, 🎬️actions, ☑️options, 🎚️config, 👥️presence, 🫧️transient)` | `TAXONOMY_WINDOW_CHILDREN` check, `:1431` | Relocate window-local tests under the owning facet's own `🧪️tests/`, per the fixed `TAXONOMY.windowChildDirs` set | hand-authored (a rename/move per window, 8 distinct windows) |
| 7 | window has unexpected child `🎚️options` | 14 | (same 8 windows) | `:1431` | Rename `🎚️options` → `☑️options` (the taxonomy's declared name) per window | hand-authored |
| 8 | window has unexpected child `⚙️config` | 14 | (same 8 windows) | `:1431` | Rename `⚙️config` → `🎚️config` per window | hand-authored |
| 9 | surface missing `🎚️config/🧬️schema/` (+`/📜️.wit`) | 6 | `surface "✳️any/✏️editor" is missing 🎚️config/🧬️schema/📜️.wit` | `assertAppSchemaOwner`, `:1499-1513` | Add the 5 schema-format leaves under each surface's `🎚️config/🧬️schema/` | hand-authored |
| 10 | surface missing `👥️presence/🧬️schema/` (+`/📜️.wit`) | 6 | `surface "✳️any/👁️viewer" is missing 👥️presence/🧬️schema/` | `:1515-1522` (same function, presence branch) | Add the 5 schema-format leaves under each surface's `👥️presence/🧬️schema/` | hand-authored |
| 11 | plugin root missing `🎮️commands/🦀️.rs` | 1 | `plugin root is missing 🎮️commands/🦀️.rs` | `validatePluginContractRoot`, `:1200-1203` (`TAXONOMY.pluginRequiredChildDirs`) | Create `🎮️commands/🦀️.rs` at the plugin root | hand-authored |

Total: 573+3+3+3+3+14+14+14+6+6+1 = **640** (verified against the raw file).

## 3. Split by owner, and what's shared with other plugins

`surfaceDirsForPlugin` (`…📇️registry/📜️script.ts:1128-1145`) builds each surface's finding label as
`${subset}/${role}` — **it never includes the artifact/kind segment** (only `pluginId` prefixes the
line). So the raw check output cannot by itself tell you *which* of the 3 artifacts a `surface "…"`
or `window "…"` finding belongs to when two artifacts have the same subset/role/mode/window names
(they do: all three ship `✳️any/👁️viewer` + `✳️any/✏️editor`). Attribution below was reconstructed by
`find`-ing the actual window/surface dirs and checking which ones physically hold the offending
child — a real diagnostic gap worth fixing at the policy level (append the artifact `kind` to the
label) independent of the content fixes.

| owner | reachable (Cargo mount) | missing schema/io/engine/examples | window unexpected-child | surface config/presence schema | total |
|---|---|---|---|---|---|
| 🧊️generation3d | 259 | 4 | 18 (6 windows × 3) | 4 (`…/📜️.wit` only — schema dir exists) | **285** |
| 🌀️generation2d | 172 | 4 | 18 (6 windows × 3) | 4 (2 dir-missing on `👁️viewer` + 2 `📜️.wit`-missing on `✏️editor`) | **198** |
| 🧩️assembly | 141 | 4 | 6 (2 windows × 3) | 4 (both surfaces have only `📌️.empty.md` — `🎚️config`/`👥️presence` exist but are empty, no `🧬️schema/` at all) | **155** |
| plugin-root (`🫀️core/🖼️semantic-ui`, `🎮️commands/`) | 1 | — | — | — | **2** |
| **total** | **573** | **12** | **42** | **12** | **640** |

Shared-with-other-plugins, at the **class** level (not the specific paths — those are procedural's
own tree):
- Classes 1 (`not reachable from Cargo manifest`) and 2-5 (`missing 🧬️schema/|🚪️io/|⚙️engine/|📚️examples/`)
  are the two dominant classes for **every** plugin in the repo-wide 19,447 — `📓️rebuild-2026-09-09.md`
  already names them "the same two generic classes everyone else has." `✒️writer`'s sample in the raw
  log (`🗄️registry-check-full.txt` head) shows the identical `window "…" has unexpected child "🧪️tests"`
  and `… is not reachable from Cargo manifest …` lines procedural has — same policy, same message
  shape, different paths. A **policy-level** fix (e.g. relaxing `TAXONOMY_ARTIFACT_COMPONENTS`
  completeness, or a real `mod`-mounting generator) would shrink every plugin's count in the same
  stroke; there is no such generator today (§2) — the fix is per-plugin, per-leaf hand-authoring
  even though the *rule* is shared.
- `🌊️flow` (this packet's other plugin, 240 findings, verified by grepping the same raw run) is a
  useful scale comparison: 227 are `not reachable from Cargo manifest` and 4 are the single artifact
  `"🌊️flow"` missing `🧬️schema/|🚪️io/|⚙️engine/|📚️examples/` — 231 of 240 (96%) is the identical
  two-class shape as procedural, the remaining 9 are its own window/surface findings. Flow has 1
  artifact to procedural's 3, which is why its total is proportionally smaller.
- `🗄️stdio` (6,272, the largest single plugin) is 32% of the entire repo-wide total by itself —
  useful context for how small procedural's 640 (3.3%) actually is relative to the worst offender.

## 4. Commands for a fix lane, and expected residual

```
bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts check
```
is the whole gate (no `nx` needed to iterate; `bun nx run @semio-tech/plugin-registry:check` is the
CI-registered form). It fails fast on catalog staleness first — run
`bun nx run @semio-tech/plugin-registry:generate` beforehand if `🤖️generated/` drifts. There is no
`--plugin`/`--scope` flag on `check` (`CheckScript.run(_segments)` ignores its args, `:3367`); this
audit's `🐍️viewer-policy-check.ts` pattern (call the exported pure policy functions and
`.filter(...includes(scope))`) is the only way to scope narrower, and only for the 5 functions it
already imports — `validateTaxonomyTree`/`validateRustTaxonomyMounts` are **not exported** from
`…📇️registry/📜️script.ts`, so an equivalent procedural-only runner for *this* phase would need one
extra `export` on `validateTaxonomyTree` (a one-line, zero-behavior-change diff) or to keep eating
the full ~19k-line run.

Expected residual after a procedural-only fix lane clears all 640: repo-wide total drops to
**18,807** across the other 32 plugins — `plugin-registry:check` **stays red** (norm, architect,
stdio, etc. are untouched and are themselves multiple orders of magnitude bigger), so this alone
does not turn the gate green; it only removes procedural's slice and lets `📓️rebuild-2026-09-09.md`
§5's "outside this lane's remit" note be closed out.

## 5. The nested-gitlink abort — corrected, and its in-repo remedy

Reproduced `bun ./📜️script.ts verify taxonomy report` directly. It is **not** blocked by the
2026-08-17 ticket's gitlinks the way `🐍️viewer-policy-check.ts`'s header comment describes — those
15 paths under `…/📓️rust-join-provenance/🧪️runs/…` are `git rm`'d from the index already (`git status`
shows them `D` in the index column; `git ls-files -s` returns nothing for them) even though they're
still gitlinks in `HEAD`'s tree (`git ls-tree HEAD` shows mode `160000`) — since taxonomy inventory
reads the **live index**, not `HEAD`, they are moot and never reach source admission. The verify
command aborts on a **different, live** gitlink instead:

```
error: Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/🔎️recherche
    at inventoryTaxonomyWithSourceParentPruning (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:6528:37)
```
Confirmed: `git ls-files -s -- "♻️mit-bestand/🔎️recherche"` → `160000 92036c7c… 0`, clean (no
pending status) — a live, tracked, top-level nested-repo checkout unrelated to any taxonomy ticket.

Quoting the throw site (`🧹️normalization/🟦️.ts:6519-6528`):
```ts
if (scope && isExcluded(scope, taxonomy)) throw new Error(`Inventory scope is opaque: ${scope}`);
...
const blockingAdmission = sourceAdmission.diagnostics.filter((row) => row.code !== "tracked-path-absent");
if (blockingAdmission.length > 0) throw new Error(`Source admission rejected: ...`);
const repositoryBoundary = sourceAdmission.observations.find((row) => row.repositoryBoundary === "gitlink");
if (repositoryBoundary) throw new Error(`Normalization requires an explicit repository-boundary decision before authored classification: ${repositoryBoundary.sourcePath}`);
```
**Yes, the policy already supports an ignore rule** — `collectTaxonomySourceAdmission`'s `add()`
(`:2919-2921`) drops any candidate matching `taxonomy.exclusions` (`🔣️taxonomy.json`'s
`pathExclusions`) *before* it ever becomes a source-admission candidate:
```ts
const opaquePrefixes = ["compose", ...taxonomy.exclusions.map((entry) => entry.path)];
...
if (sourceAdmissionOpaque(path, opaquePrefixes) || !inScope(path, scope)) return;
```
so it never reaches the `repositoryBoundary` check or the `opaque-path` diagnostic (that diagnostic
only fires for candidates that *do* enter `projectTaxonomySourceAdmission`, which excluded paths
never do). `🔣️taxonomy.json` already has exactly this shape for two other opaque subtrees:
```json
"compose": { "path": "compose/", "mode": "opaque", "reason": "Explicit user-owned opaque subtree; filter before every filesystem access" },
"temp-compose": { "path": "temp/compose/", "mode": "opaque", "reason": "Explicit recovered opaque subtree; filter lexically before every filesystem access" }
```
A third entry — `"path": "♻️mit-bestand/", "mode": "opaque", "reason": "…nested git checkout, not
taxonomy-owned"` — would let `verify taxonomy report` proceed past this abort with **no code
change**, only a `🔣️taxonomy.json` data edit. Left un-made here: it is outside this ticket's
procedural scope and this audit is read-only. Note that even after that fix, `verify taxonomy` would
still need the same treatment for `♻️mit-bestand` before it can run to completion at all — it is a
single top-level blocker, not one-per-gitlink, since the 08/17 ticket's 15 gitlinks are already
moot.

## Files

- `🗑️generated/taxonomy-procedural.txt` — raw 640-line procedural slice (delete at ticket close).
- This report.
