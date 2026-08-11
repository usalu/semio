# Fan-out brief — Artifact Standard Subsets

Read `🧪subset-roster.json` in this ticket folder first — it is the completeness oracle and per-subset spec brief (checks, hard/soft severity, composer duty, schema gaps). This doc is the *how*, the roster is the *what*.

## Ownership rules (hard constraints)

- You may ONLY create/edit files under `🗿️artifacts/<your-artifact>/` (including that artifact's standard-level `⚙️engine`/`🎹️composer`/`🧬️migrations` files when you own the whole standard).
- **FORBIDDEN to touch**: `📦️glue.rs` (stdio), root `📜️script.ts`, `🔣️taxonomy.json`, `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, any `Cargo.toml`. These are orchestrator-only. Deliver what they'd need instead (see "Handoff" below).
- Never run git commands (commit/stash/checkout/worktree) — live concurrent devs + an auto-commit daemon share this tree.
- All scratch/logs/reports go inside this ticket folder, `.txt`/`.md` only (never `.log` — gitignored and dropped by ticket_close).
- Never call `ticket_close` or `ticket_reopen` — only the orchestrator does, with the explicit ticket path `26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES`.
- If `cargo check` fails on a file you didn't touch, check `baseline-cargo-w0.txt` in this folder first — some breakage is pre-existing/from a concurrent session (noted there). Don't fix files outside your artifact; note it and proceed.

## The pattern (study before writing anything)

Canonical pilot: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/` (post-W2-restructure; was `✳️a-2b`). Also read `📋️subset-pattern.md` in this folder if it exists (authored by the W2 pilot agent) for the up-to-date canonical shape under the new mechanism.

Every real (non-`✳️any`) subset dir gets, under `🏅️standards/🔖️<std>/🪆️subsets/✳️<id>/`:
- `🧬️schema/` (rs+ts): pure `pub use …::subsets::any::schema::*` — a subset is a validation stamp, not a new snapshot type.
- `🧐️analyzer/` (rs+ts): a `DIALECT` const, a pure `check_<subset>_conformance(&Snapshot) -> Vec<Diagnostic>` fn with per-check `CODE_*` consts (hard finding = `Severity::Error`, soft = `Severity::Warning`, codes namespaced `stdio.<artifact>.<subset>.<check>`), delegates parsing to `✳️any`'s analyzer then appends conformance diagnostics, inline `#[cfg(test)]` tests (conforming + each violation case).
- `🎹️composer/` (rs **and** ts — the ts leaf is required, unlike the old a-2b pilot which was missing it): `impl ArtifactComposer` (writes SELF; reads at least `[ANY-of-this-standard, SELF]` plus the artifact's catalog DAG deps), hard-gates on the conformance fn before serializing, `impl SubsetValidator` + a `register()` fn calling `register_subset_validator`, inline tests.
- `🏗️builder/` (rs+ts): `impl ArtifactBuilder`, `build()` re-runs the conformance check as a hard gate.
- `🚪️io/` (rs+ts): doc-leaf only, referencing the owning (`✳️any`) subset's import/export tree — do NOT duplicate the full import/export leaf tree here.
- `🔣️component.json` manifest at `🏅️standards/🔖️<std>/🪆️subsets/🔣️component.json` (one per standard, shared across all its subsets — if it already exists from another subset in the same standard, ADD your entry, don't overwrite): `{"artifact": "s.stdio.<artifact>", "standard": "<std>", "subsets": {"*": {"name": "..."}, "<your-id>": {"name": "<spec name + citation>", "levels": [...optional]}}}`.

Registration: your standard's `🎹️composer/🦀️component.rs` `entries()` needs a `composer_entry_of::<YourComposer>()` added, and its `⚙️engine::register()` needs your subset composer's `register()` called. If the standard-level files aren't yours to edit (rare — only when another unit in the same wave owns that standard), note it in your report instead of editing.

## Handoff protocol (glue.rs)

Write your glue module block to `<ticket>/🧩glue/<artifact>-<std>-<subset>-glue.rs.txt`:
- The exact `pub mod <rust_ident> { #[path = "..."] pub mod schema; ... }` block (rust_ident = slug with `-`→`_`, and a leading digit prefixed `_`).
- The exact anchor: the full `#[path = "…"]` line of the `✳️any` subset in the same standard's `pub mod subsets { ... }` block in `📦️glue.rs`, so the orchestrator knows where to insert.

If you need a new Cargo dependency, write `<ticket>/🧩glue/<artifact>-cargo-deps.txt` instead of editing any `Cargo.toml`.

## Report

When done, write `<ticket>/w<N>-<artifact>-report.json`:
```json
{"unit": "<artifact>", "standards": [{"std": "<std>", "subsets": [{"id": "<id>", "files_created": [...], "tests": [...], "validator_registered": true}]}],
 "glue_snippet_paths": [...], "allowlist_removals": [...], "cargo_dep_requests": [...], "schema_gaps": [...],
 "preexisting_breakage_noted": [...], "status": "done|partial|blocked", "blockers": [...]}
```

## Verification (self-check before reporting done)

`cargo check -p semio-s-plugin-stdio 2>&1 | grep -A5 "your artifact's path"` — should show no NEW errors beyond `baseline-cargo-w0.txt`'s noted pre-existing ones. Do not run the full workspace build or `verify gate` — those are orchestrator-only end-of-wave gates.
