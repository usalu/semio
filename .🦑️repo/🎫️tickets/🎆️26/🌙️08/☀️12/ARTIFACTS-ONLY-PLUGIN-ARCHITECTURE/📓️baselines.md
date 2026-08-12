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

## ⚠️ `semio-framework-plugin` is RED (observed 2026-08-12 ~17:50) — nothing plugin-side is verifiable

```
🔌️plugin/🦀️component.rs:5790:41  error[E0499]: cannot borrow `self.children` as mutable more than
                                  once at a time — borrowed in the previous loop iteration
🔌️plugin/🦀️component.rs:3152:38  error[E0560]: struct `TutorialBase` has no field named `document_dsl`
🔌️plugin/🦀️component.rs:3439:35  error[E0609]: no field `document_json` on `semio_framework::ExampleDefinition`
```

**Not APA's** — APA has modified no Rust framework file.

**Ownership: all three are UCAS's, and the file is live.** The renames landed at the definitions (`TutorialBase.document_dsl` → `artifact_dsl` at `🛂️manifest:1436`; `ExampleDefinition.document_json` → `artifact_json` at `🛂️manifest:2682`) and had not yet reached two `#[cfg(test)]` call sites in `🔌️plugin` (`:3152`, `:3439`). **Retry-and-wait is the protocol; do not patch.**

### A mistake worth keeping: "unowned" is a much stronger claim than "I can't tell who owns it"

APA initially diagnosed this pair as **orphaned debt from a closed ticket** (`26/08/10/RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE`) with no live owner, and broadcast that conclusion — plus an offer to patch it — to all four peer sessions. DKM disproved it in one step with the signal that actually settles ownership:

| file | mtime | meaning |
|---|---|---|
| `🛂️manifest/🦀️component.rs` | Aug 12 03:50 | rename landed at the definitions ~14h ago |
| `🔌️plugin/🦀️component.rs` | Aug 12 17:33 | **minutes ago — actively being edited** |

Both files are rows in UCAS's own hot-file table. This is one session's rename mid-propagation, and the `E0499` `self.children` borrow is that same session's composition round in the same file. The broadcast was retracted on all four channels before anyone acted on it.

The root error is instructive: APA reasoned from a *plausible origin story* rather than from evidence, having already told every session that `git status` is useless here because the repo auto-commits — and then not using the mtime check it had itself recommended. **Rule: check mtime before declaring anything unowned.** A patch applied on the strength of "nobody owns this" would have landed inside a live edit, which is precisely the failure the whole cross-session protocol exists to prevent.

Consequences, both material:
1. **Every plugin crate depends on this**, so no per-plugin `cargo check` proves anything until it is green. This is why W3 batches 3 and 4 run with cargo disabled and verify structurally instead — a red SDK makes per-agent cargo gating pure cost.
2. **The E0560/E0609 pair only surfaces under `--all-targets`** (plain `check` skips `#[cfg(test)]`). While it is red, no session can run the triad law harness — the correctness argument for both the mutations and composition tickets.

### `🧩️puzzle` is `blocked-churn`, not green and not broken

`cargo check -p semio-s-plugin-puzzle --all-targets` died on the dependency above before reaching puzzle. **Zero errors originate in any `🔌️plugins/🧩️puzzle` path** (grep-verified), but nothing is proven. Recorded here rather than reported as a pass, because a third session (INFERENCE-FAMILY) is waiting on this answer before building on puzzle, and an optimistic "probably fine" would have them build on an unverified base.

## APA's own cargo baseline

`CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p <crate>` — recorded per crate inside each `📓️w3-<crate>-report.md` at step 0, before any edit, so every W3 packet carries its own before/after pair rather than relying on a global snapshot that goes stale within minutes.
