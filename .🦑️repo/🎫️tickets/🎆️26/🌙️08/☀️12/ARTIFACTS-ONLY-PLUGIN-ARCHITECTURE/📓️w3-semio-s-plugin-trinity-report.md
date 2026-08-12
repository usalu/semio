# W3 — `semio-s-plugin-trinity` (🔱️trinity) migration report

## Clearance

SMO's live predicate `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`
lists `🔱️trinity` under **"RELEASED — lane finished, compiles in the workspace check"**
(`🔌️jack`, `♻️rewrite` facets; `jack`: 10 triads, `SetFixture` gone; `rewrite`: `SetState` deleted,
7 field-level mutations). Not HELD, not another session's. Proceeded.

## Answers to the two prominently-requested questions

**(a) `recompute_derived` / `DerivedPropertyReadonly` — did any moved file contain them, and where are they now?**

Yes — exactly **one** moved file: `🧮️executor/🦀️component.rs` (old plugin-root path) called
`g.recompute_derived();` at its old line 353. Its **new path** is:
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🧮️executor/🦀️component.rs`
(same relative line, file moved verbatim, zero edits to its body).

Neither symbol was preserved or defended — the file was moved as-is per instructions, for the
INFERENCE-FAMILY session to delete/rewrite next.

All **other** occurrences in the plugin were already inside files I did **not** move (pre-existing
locations, unchanged by this wave) — listed here so the INFERENCE-FAMILY session has the complete
in-plugin call-site map, all at their **original, still-current** paths:

| File | Lines | Symbol |
|---|---|---|
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs` | 66 | `DerivedPropertyReadonly` (enum variant definition) |
| same file | 289 | `pub fn recompute_derived(&mut self)` (method definition) |
| same file | 513, 630, 810, 864 | `recompute_derived()` call sites (incl. one in `#[cfg(test)]`) |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` | 153 | `TrinityRamError::DerivedPropertyReadonly { .. }` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | 246 | `recompute_derived()` call |
| `✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/📌️panels/🔍️inspection/🦀️component.rs` | 22 | `recompute_derived()` call |
| `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/📌️panels/🔍️inspection/🦀️component.rs` | 23 | `recompute_derived()` call |
| `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs` | 585, 972 | `recompute_derived()` call |
| `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` | 42, 127 | `recompute_derived()` call — **this file was NOT moved this wave**, see §"🔨️modules exception" below; still at this path |

**(b) Did the move split across both artifacts (`🔌️jack`, `♻️rewrite`)? Which went where?**

**No split — all five plugin-root dirs are jack-owned; zero moved into `♻️rewrite`.**

Justification: every one of `🌳️ast`, `🔤️lexer`, `🧮️executor`, `🗣️language-service`'s own file-header
docstrings self-identify as "Trinity **jack** query AST / lexer / executor / language service" and
internally `use crate::artifacts::jack::{...}` — never `crate::artifacts::rewrite::`. `rewrite`
*consumes* this kernel (its own engine facade, `🗿️artifacts/♻️rewrite/…/⚙️engine/🦀️component.rs`,
carries this exact pre-existing doc comment confirming the design: *"The jack query-language compute
itself... lives in the plugin's `🫀️core` cross-artifact kernel — used by both the `jack` app's UI and
the `rewrite` app's `apply_rule` — not here."*) via plain same-crate module calls
(`crate::ast::…`, `crate::executor::execute`, `crate::language_service::parse`,
`crate::lexer::tokenize`) — confirmed by grep, 3 rewrite-side files depend on it
(`🗿️artifacts/♻️rewrite/…/⚙️engine/🦀️component.rs`, `🎛️apps/♻️rewrite/🌍️world/🦀️component.rs`,
`🎛️apps/♻️rewrite/🪟️windows/🔎️jack/🦀️component.rs`). Since the DSL is literally named after `jack`
and `jack` is the artifact that declares its identity, physical ownership went to jack's artifact
engine. Because the crate-root module names (`ast`, `lexer`, `executor`, `language_service`, alias
`core`) were kept stable and only their `#[path]` targets were repointed, **zero call sites anywhere
in the crate — jack or rewrite — needed source edits.** `pub use language_service as core;` is
carried over unchanged; grep confirms nothing actually uses `crate::core::` (dead alias, pre-existing,
not touched).

## What changed

### Step 1 — dead facet directories

All three (`🛂️manifest/`, `🎟️capabilities/`, `🔧️setup/`) confirmed **unmounted**:
`grep -n "manifest\|capabilities\|setup" 📦️glue.rs` → zero hits before my edits.

- `🎟️capabilities/🦀️component.rs` — 1-line doc-only stub. Deleted, dir removed.
- `🔧️setup/🦀️component.rs` — 1-line doc-only stub. Deleted, dir removed. (Not to be confused with
  `.setup(crate::register_trinity_exports)` on the `Plugin::builder` in root `🦀️component.rs` — that
  function is defined directly in `📦️glue.rs`'s `//#region 🔖️Bundle`, unrelated to this facet dir.)
- `🛂️manifest/🦀️component.rs` — 1-line doc-only stub, **but the directory also held three real data
  fixtures**, not doc-only:
  - `🛂️manifest.jsonnakagin.manifest.json` (28,964 bytes — Nakagin Capsule Tower node-graph manifest:
    `nodeKinds`/`edgeKinds`/`portKinds`/`wireKinds`)
  - `🛂️manifest.jsonrewrite-lhs.manifest.json` (1,520 bytes)
  - `🛂️manifest.jsonrewrite-rhs.manifest.json` (2,055 bytes)

  These are discovered **by filename pattern alone, repo-wide, regardless of directory** — confirmed
  by reading `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts:44-66`
  (`findManifestFiles`), whose own doc comment states the invariant explicitly: *"A manifest source is
  tagged by its filename, not by living in a directory named 'manifest'... sits directly under the
  component's own artifact folder with no 'manifest'-named parent directory at all... e.g.
  `🛂️manifest.jsonnakagin.manifest.json`"* (this is literally trinity's own filename, cited as the
  worked example in the framework script). The already-closed `🧩️puzzle` plugin (per `w0-census`)
  already uses exactly this pattern (`🗿️artifacts/<kind>/🛂️manifest.json<descriptor>.manifest.json`,
  confirmed on disk) — I followed the same precedent:
  - `nakagin` → `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json`
    (content is node/edge/port graph data — jack's domain; confirmed by `manifest_id: "nakagin"` used
    pervasively across `🗿️artifacts/🔌️jack/🦀️component.rs`, `🗣️language-service`,
    `🧮️executor`, and the shell binary)
  - `rewrite-lhs` → `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🛂️manifest.jsonrewrite-lhs.manifest.json`
  - `rewrite-rhs` → `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🛂️manifest.jsonrewrite-rhs.manifest.json`

  No Rust/TS file anywhere references these three files by their literal filename (`grep -rn` for the
  exact names, repo-wide, returns only one hit — the doc-comment *example* in math's script.ts, not a
  real reference) — discovery is filename-pattern-driven, so relocating them is transparent to the
  codegen. Then `🛂️manifest/🦀️component.rs` + now-empty dir deleted.

No `.DS_Store`/`node_modules` junk at plugin root (confirmed absent before and after).

### Step 2 — plugin-root compute dirs relocated into `🔌️jack`'s artifact engine

Moved, per-file grain, no inlining, no merging:

| Old path | New path |
|---|---|
| `✏️s/🔌️plugins/🔱️trinity/🌳️ast/🦀️component.rs` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🌳️ast/🦀️component.rs` |
| `✏️s/🔌️plugins/🔱️trinity/🔤️lexer/🦀️component.rs` | `.../⚙️engine/🔤️lexer/🦀️component.rs` |
| `✏️s/🔌️plugins/🔱️trinity/🧮️executor/🦀️component.rs` | `.../⚙️engine/🧮️executor/🦀️component.rs` |
| `✏️s/🔌️plugins/🔱️trinity/🗣️language-service/🦀️component.rs` | `.../⚙️engine/🗣️language-service/🦀️component.rs` |

Destination directory chosen to mirror the **existing** engine mount location jack already uses
(`🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/`) rather than inventing a different layout — this exactly
matches the destination the root policy census (`📜️script.ts` report-mode table, lines 4883-4887)
already recommended for these four dirs (`Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/<name>/`).

`📦️packages/🦀️rust/📦️glue.rs`, `//#region 🔤️Jack kernel` (was lines 23-33): the four `#[path]`
attributes repointed from the plugin-root paths to the new artifact-engine paths above. **Module
names at crate root kept unchanged** (`pub mod ast;`, `pub mod lexer;`, `pub mod executor;`,
`pub mod language_service;`, `pub use language_service as core;`) — added a doc comment explaining
why (cross-artifact kernel, see §b above). Zero call sites elsewhere in the crate needed editing as a
result (verified: `crate::ast`, `crate::lexer`, `crate::executor`, `crate::language_service` all
still resolve to the same module path, only the file backing them moved).

None of the four files had any internal `#[path]` attributes or relative-path assumptions (`grep`
confirmed — all internal `use` statements are `crate::`-absolute or external-crate), so no
in-body edits were needed.

### `🔨️modules/` — NOT moved this wave; concrete, verified reason (not a hand-wave)

`🔨️modules/` holds two subtrees, both under `🔌️jack/`:
- `🐚️shell/📦️packages/🦀️rust/` — `semio-s-plugin-trinity-jack-shell` (`role = "tool"`), a `[[bin]]`
  crate (`📦️bin.rs`, 141 lines) — a real Jack-query REPL.
- `🧠️lsp/📦️packages/{🦀️rust,🟦️typescript}/` — `semio-s-plugin-trinity-jack-lsp` (`role = "tool"`), a
  `cdylib`/`rlib` crate built via `wasm-pack` with a checked-in `pkg/` build output
  (`trinity_jack_lsp_bg.wasm` + `.d.ts` + `.js`) and a companion TypeScript package.

Both are **separate Cargo crates with their own `Cargo.toml`**, structurally the same crate-boundary
class as the `🧩️extensions` exception in the dispatch packet (own package, own manifest, path
dependency back to the parent crate) even though their `role` metadata says `"tool"` rather than
`"extension"`. Verified this is not a hypothetical risk but a concrete one: **both crates are
hardcoded, non-glob members of the repo-root `Cargo.toml` workspace `members` array**
(`Cargo.toml:58-59`) **and** the `🧠️lsp` TypeScript package is a hardcoded, non-glob member of the
repo-root `package.json` `workspaces` array (`package.json:18-19`):

```
Cargo.toml:58:    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust",
Cargo.toml:59:    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust",
package.json:18:    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust",
package.json:19:    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript",
```

Moving these two crates correctly requires editing **both** of those literal string arrays in
**repo-root files**, which are explicitly outside my boundary (single plugin directory; "no
repo-root `📜️script.ts`" is named explicitly and these two files are the same class of shared,
contested resource). A wrong or missed edit here silently drops the crates from the workspace or
breaks every other agent's `cargo`/`bun` invocation — exactly the failure mode I have zero ability to
verify (cargo banned this wave). I inventoried them (below) and did **not** move them. Filed under
`## sharedFileRequests`.

`.vscode/launch.json:2429` runs `cargo run -p trinity_jack_shell -- ...` — by package name, not
path, so this specific entry is unaffected either way (noted for completeness, not a blocker).

This is the **one item preventing full plugin-root closure** this wave — see `apa-status` below.

### Step 2 — plugin root, current state

```
$ ls -a "✏️s/🔌️plugins/🔱️trinity/"
.
..
AGENTS.md
README.md
🎛️apps
📦️packages
🔨️modules   ← flagged exception, see above
🗿️artifacts
🦀️component.rs
```

Six of seven entries now match the target shape exactly (`🦀️component.rs`, `AGENTS.md`, `README.md`,
`🎛️apps`, `🗿️artifacts`, `📦️packages`); `🔨️modules` is the one documented, justified holdout.

### Step 3 — escape-hatch call sites

None. `grep -rn "register_mesh_\|register_solid_\|register_dwg_\|register_2d_export_handlers\|register_app_io\|register_os_media_" "✏️s/🔌️plugins/🔱️trinity"` → zero hits, whole plugin tree, before and
after my edits. `📓️w0-a-escape-hatch.md`'s full call-site census also lists no trinity entries. No-op,
confirmed both ways.

### Step 4 — dependency purge

`grep -rn "semio_framework_os::" "✏️s/🔌️plugins/🔱️trinity"` (excluding `_os_kernel`) → **zero hits**,
whole plugin tree. `semio-framework-os = { workspace = true }` removed from
`📦️packages/🦀️rust/Cargo.toml` (was line 38, between `infinite_canvas` and
`semio-framework-plugin`). `semio-framework-os-kernel` (a different crate, used extensively via the
`dsl`/`store`/`protocol`/`vcs` extern-crate aliases in `📦️glue.rs`) was **not** touched — still
required, still present.

### Step 5 — inventory only, nothing changed

- **`thread_local!`**: zero hits anywhere in the plugin.
- **`std::fs`/`std::env`/`std::process`/`Command::new`**, outside `#[cfg(test)]`: exactly one file,
  `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` (the flagged shell crate, not moved this
  wave) — `use std::env;` / `use std::fs;` (line 5-6), `std::process::exit(1)` (line 32). This is the
  Jack-query REPL's genuine CLI entry point (reads a fixture-path arg, reads/writes files, exits
  non-zero on error) — real, intentional IO for a standalone dev tool, not app/draft-lane state.
  Nothing else in the plugin touches the filesystem, environment, or a subprocess.
- **Network** (`reqwest`/`TcpStream`/`hyper::`/`std::net::`): zero hits.
- **`fn seed(`**: zero hits.
- **Draft-lane / interior mutability**: no `thread_local!` anywhere, so no per-app draft scratch state
  to inventory for either the `jack` or `rewrite` app. No `OnceLock`/`Mutex`-backed statics found in
  the plugin tree either (unlike energy's read-only registration caches) — none to report.

## Files touched

**Moved:**
- `✏️s/🔌️plugins/🔱️trinity/🌳️ast/🦀️component.rs` → `.../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🌳️ast/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🔤️lexer/🦀️component.rs` → `.../⚙️engine/🔤️lexer/🦀️component.rs` (same tree)
- `✏️s/🔌️plugins/🔱️trinity/🧮️executor/🦀️component.rs` → `.../⚙️engine/🧮️executor/🦀️component.rs` (same tree)
- `✏️s/🔌️plugins/🔱️trinity/🗣️language-service/🦀️component.rs` → `.../⚙️engine/🗣️language-service/🦀️component.rs` (same tree)
- `✏️s/🔌️plugins/🔱️trinity/🛂️manifest/🛂️manifest.jsonnakagin.manifest.json` → `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json`
- `✏️s/🔌️plugins/🔱️trinity/🛂️manifest/🛂️manifest.jsonrewrite-lhs.manifest.json` → `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🛂️manifest.jsonrewrite-lhs.manifest.json`
- `✏️s/🔌️plugins/🔱️trinity/🛂️manifest/🛂️manifest.jsonrewrite-rhs.manifest.json` → `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🛂️manifest.jsonrewrite-rhs.manifest.json`

**Removed:**
- `✏️s/🔌️plugins/🔱️trinity/🛂️manifest/` (dir + doc-only `🦀️component.rs`, after data files above extracted)
- `✏️s/🔌️plugins/🔱️trinity/🎟️capabilities/` (dir + doc-only `🦀️component.rs`)
- `✏️s/🔌️plugins/🔱️trinity/🔧️setup/` (dir + doc-only `🦀️component.rs`)
- `✏️s/🔌️plugins/🔱️trinity/🌳️ast/`, `🔤️lexer/`, `🧮️executor/`, `🗣️language-service/` (now-empty dirs, `rmdir`'d after their one file each moved out)

**Updated:**
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` — repointed 4 `#[path]` mounts in
  `//#region 🔤️Jack kernel`, added an explanatory doc comment; no other region touched.
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml` — removed unused
  `semio-framework-os = { workspace = true }` dependency line.

**Not touched (flagged, see above):** `✏️s/🔌️plugins/🔱️trinity/🔨️modules/` (whole subtree, incl. the
real `bin.rs` — inventoried, not deleted, not moved).

## Step 6 — structural verification (no cargo, as instructed)

1. Plugin root shape:
   ```
   $ ls -a "✏️s/🔌️plugins/🔱️trinity/"
   . .. AGENTS.md README.md 🎛️apps 📦️packages 🔨️modules 🗿️artifacts 🦀️component.rs
   ```
   6/7 match target; `🔨️modules` is the documented exception.

2. **Every `#[path = "..."]` in `📦️glue.rs` resolves to a real file** — exhaustive, not sampled.
   Extracted all attribute lines with a Python pass over the file (regex-anchored to
   `^\s*#\[path\s*=\s*"..."\]` so doc-comment prose mentioning `#[path]` isn't miscounted), resolved
   each relative to `📦️packages/🦀️rust/` (the file's own directory), and stat'd every target:
   ```
   total path attrs: 307
   missing count: 0
   ```
   (`.` self-mounts, used by the grouping `pub mod X { #[path="."] ... }` pattern, are excluded from
   the file-existence check by design — they're not leaf mounts.)

3. Dangling-reference sweep for everything I moved or removed, repo-wide (excluding build/target/node_modules dirs):
   ```
   $ grep -rln "🔱️trinity/🌳️ast\|🔱️trinity/🔤️lexer\|🔱️trinity/🧮️executor\|🔱️trinity/🗣️language-service\|🔱️trinity/🛂️manifest\|🔱️trinity/🎟️capabilities\|🔱️trinity/🔧️setup" . | grep -v "🎯️target\|node_modules\|.git/"
   ```
   Hits, all inspected:
   - `📜️script.ts` (repo root) lines 4883-4887 — the **report-mode-only** W0/W2 policy advisory table
     (`important.md` confirms APA's policy regions "census, they never gate — until APA W5"). Its
     entries for the four moved dirs are now stale keys (the old paths no longer exist, so the table
     entry simply never matches again — harmless, not a compile-time or runtime dependency). Flagged
     under `sharedFileRequests` for whoever does the W5 root-script pass, not touched by me (root
     `📜️script.ts` is explicitly out of my boundary).
   - A dozen+ hits in unrelated historical ticket folders (`.🦑️repo/🎫️tickets/…`) — scratch/report
     files from *other, older, already-closed* tickets that happen to mention these old paths in
     prose. Not live code, not touched, not in scope.
   - No hits in any `.rs`, `.ts`, `Cargo.toml`, `project.json`, or `package.json` outside the two
     files above.

4. Every moved file confirmed to exist as its own standalone file at its new path (not pasted into
   `📦️glue.rs` or any parent `mod`) — this is directly evidenced by item 2 above: each of the 307
   `#[path]` targets, including the four relocated ones, is a real file on disk, and `📦️glue.rs`
   itself contains no inlined bodies for `ast`/`lexer`/`executor`/`language_service` (still pure
   `#[path]` + `pub mod` declarations, zero code added to the module bodies themselves).

## sharedFileRequests

1. **File**: repo-root `Cargo.toml`, `[workspace] members` array, lines 58-59.
   **Reason**: if/when `🔨️modules/🔌️jack/🐚️shell` and `🔨️modules/🔌️jack/🧠️lsp` are relocated under
   `🗿️artifacts/🔌️jack/…/⚙️engine/{shell,lsp}/📦️packages/🦀️rust`, these two literal path strings need
   updating in lockstep, plus each crate's own `Cargo.toml` `path = "../../../../../../../../…"`
   relative dependency back to `semio-s-plugin-trinity` and to
   `semio-framework-os-kernel`/`dsl_lsp` needs its `../` depth recomputed for the new nesting depth.
   Out of my single-plugin boundary; I did not move the crates or touch this file.
   **Patch**: not written — needs a decision on the exact destination depth first (proposed above:
   `🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/{shell,lsp}/📦️packages/🦀️rust`, mirroring
   the four single-file dirs' destination, but this is a proposal, not a ruling).

2. **File**: repo-root `package.json`, `workspaces` array, lines 18-19.
   **Reason**: same move, TS side — `🧠️lsp`'s Rust and TypeScript package paths are both hardcoded
   there. Same dependency as request 1.

3. **File**: `📜️script.ts` (repo root), lines 4883-4887 (report-mode policy advisory table).
   **Reason**: four stale keys now point at paths that no longer exist (harmless — report-mode only,
   never gates a build) — cosmetic cleanup for whoever does the W5 pass over that file. Not urgent.
   **Patch**: not written (four one-line key deletions/updates, trivial for the owning session).

4. **File**: `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/📦️index.ts`.
   **Reason**: pre-existing (untouched by me, predates this session) — every export path in this file
   targets a **flat** `🗿️artifacts/<kind>/🧬️schema/…` shape that no longer exists on disk (the real
   tree is nested under `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/…`, same drift the energy-plugin
   W3 report flagged for its own TS facade). Not this wave's scope (Step 2 here only covers the five
   flagged plugin-root compute dirs), and fixing it correctly needs the intended post-restructure WASM
   facade shape from whoever owns the artifact schema layout — flagging, not guessing.

## Concurrent-churn observations

- `git log --oneline -5 -- "✏️s/🔌️plugins/🔱️trinity"` and `stat -f '%Sm'` on the plugin root showed the
  tree stable (last batch-touch dated `Aug 12 10:50`, consistent with the repo-wide batch mentioned in
  `w0-b-plugin-shape.md`) at the start of this session — no other agent's edit landed on this plugin's
  files mid-session as far as `git log --oneline -3` at report time shows.
- Per the ticket's rule, I never ran a bare or scoped `cargo check`/`build`/`test` for this plugin —
  intentionally deferred, as instructed. No compile evidence exists yet for this wave's edits.
- Did not touch `🧬️mutations/**` anywhere in the tree (out of scope, confirmed no edits inside any
  `🧬️mutations` dir — the only files I moved under `🗿️artifacts/🔌️jack/…` were the four engine kernel
  files and the manifest JSON, neither inside a `mutations` dir).
- Did not author or touch any `thread_local!`/draft-lane facet (none exist in this plugin to touch).
- Did not rename or re-declare any artifact kind id (`jack`, `rewrite` untouched, no new
  `ArtifactKindSpec`/`id:` string added or changed anywhere).

## apa-status: partial

Everything in scope is done and structurally verified **except** the single flagged item:
`🔨️modules/🔌️jack/{🐚️shell,🧠️lsp}` remains at the plugin root because moving it correctly requires
editing two repo-root files (`Cargo.toml`, `package.json`) outside my plugin-directory boundary — see
`sharedFileRequests` #1-2. Both crates were inventoried (role, real IO usage, dependency shape), not
deleted, not silently left unexamined. Every other step (dead-facet deletion incl. real fixture-data
relocation, the four-dir engine relocation with full justification for the no-split decision,
escape-hatch census, dependency purge, Step 5 inventory, Step 6 structural verification) is complete
and evidenced above. Cargo verification was intentionally deferred per the ticket's ban on running
cargo this wave — the consolidated build should check, in priority order: (1) the four repointed
`#[path]` mounts in `📦️glue.rs` actually compile (structurally verified to resolve to real files, not
compiler-verified), (2) the removed `semio-framework-os` dependency doesn't turn out to be needed by
a macro-expanded or feature-gated path my grep couldn't see, (3) the manifest-fixture relocation is
picked up correctly by `math`'s `findManifestFiles` codegen at its new artifact-root locations.
