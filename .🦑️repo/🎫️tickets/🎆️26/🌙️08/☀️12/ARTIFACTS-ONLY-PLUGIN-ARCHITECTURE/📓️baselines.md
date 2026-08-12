# Durable baselines — distinguish "new breakage" from "the tree was already like this"

Three sessions edit this tree concurrently and all three have lost time to that ambiguity. Record a baseline before changing anything, cite it afterwards. Provenance is established with `git log --oneline -- <path>` against the auto-commit flag counter (`🐙️ueli…🚩️<n>`), **never** with `git status` — the repo auto-commits, so recent work reads as clean.

## Flag counter reference

| session | started at flag |
|---|---|
| SEMANTIC-MUTATIONS-OVERHAUL (SMO) | before 485 |
| UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (UCAS) | 491 |
| ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE (APA) | 492 |

A file whose last commit predates a session's start flag cannot have been changed by that session.

## `🦑️repo/…/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

| when | result |
|---|---|
| before APA touched it (2026-08-12 ~15:30) | **132 pass / 22 fail** / 838 expect() |
| after APA's two edits | **134 pass / 20 fail** / 840 expect() |

APA made exactly two changes here, both strictly reducing failures:
1. `pluginChildDirs` literal → `["🎛️apps"]`, paired with the taxonomy flip.
2. `artifactComponentDirs` literal → `["🧬️schema","⚙️engine","🚪️io"]`, fixing a **stale expectation left by a closed ticket**. Provenance: `🔣️taxonomy.json`'s last commit is flag **490**, predating both UCAS (491) and APA (492), so the three-entry value was already in the tree; the likely origin is `26/08/12/DERIVE-ARTIFACT-ANALYZERS-COMPOSERS-AND-BUILDERS`, closed today, which collapsed the artifact lifecycle dirs and updated taxonomy discovery but left this test expecting the old eight-entry list. Confirmed with UCAS before taking it.

**The remaining 20 failures are pre-existing and are NOT APA's.** They span: `dependency-boundary`, `ui scrollbar styling`, `micro-commit`, `playground static sites` (×2), `package boundary guards`, `commit`, `command budgets` (×2), `resolveCargoPackageName` (×2), `loadTaxonomy` (×2 remaining), `validateTaxonomy`, `discoverPackages` (×4), `computeWorkspaces`. Anyone reading this suite red should diff against 20, not against 0.

## Peer-reported baselines (their evidence, recorded here so APA does not re-derive it)

- **stdio** (UCAS): `2021 passed / 5 failed / 3 skipped`. The five failing facets' last commit predates their ticket.
- **workspace** (SMO, ~15:50): `cargo check --workspace` → **0 errors** across framework and all 33 plugins. This is the first point in this session where plugin-side verification became meaningful; before it, stdio was mid-rename and every plugin was transitively red.
- **raster** (SMO): `66 passed / 0 failed`.

## APA's own cargo baseline

`CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p <crate>` — recorded per crate inside each `📓️w3-<crate>-report.md` at step 0, before any edit, so every W3 packet carries its own before/after pair rather than relying on a global snapshot that goes stale within minutes.
