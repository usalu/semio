# vcs — Wave 4 report

## Task recap

Design doc §4: `vcs→R:any` — vcs was expected to reference stdio's `any` tagged-union subset via
`store::ArtifactLink` for some ad-hoc reference to another artifact's content.

## Finding: no ArtifactLink migration needed

Read `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️component.rs` (artifact root) and
`.../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` (the `VcsSnapshot` struct)
first, then grepped the whole plugin (`artifact_id|artifact_ref|ArtifactLink|target_id|linked_artifact|
referenced_artifact|snapshot_id|blob|content_id|pointer|ArtifactChild`, plus `checkpoint|commit_of|
history_entry|other_artifact|foreign|external_id`) across every `.rs` file.

`VcsSnapshot` is `{ schema, title: String, counter: i64, notes: String, status: String, tags:
Vec<String> }` — six plain scalar/collection fields, no id/blob/pointer field anywhere. The plugin's
"checkpoint"/"alternative" vocabulary (`envelope.vcs.checkpoints`, `ArtifactCommand::CommitCheckpoint`/
`CheckoutCheckpoint`/`CreateAlternative`/`SwitchAlternative`) is exclusively the **framework's own
built-in version-history mechanism** applied to VCS's own document — undo/redo/branch/checkpoint
metadata, not a reference to another artifact's content. Every `stdio` mention in the plugin is
generic import/export codec boilerplate (JSON/CSV/XLSX/ZIP/TXT serializers), unrelated to composing or
referencing the `any` subset.

**Conclusion**: `vcs` is a self-contained demo/exerciser for the framework's checkpoint/alternative
history UI (swimlane graph, undo/redo). It genuinely does not reference or compose another artifact's
content anywhere in its current implementation — there is no ad-hoc reference to replace. This matches
the task brief's sanctioned "no change needed" outcome (parallel to `draw` in wave 3).

## Unrelated pre-existing breakage found and fixed (in-boundary, own-file cascade)

Baseline `cargo check -p semio-s-plugin-vcs --all-targets` (run before any edit) was **RED**, entirely
unrelated to ArtifactLink: 34 errors, all inside `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/**` (app layer), zero
inside `🧬️mutations/**` (untouched, per rules).

Root cause, traced via `git log -1 --date=iso`:
- `.../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (defines the
  *current*, conforming `VcsDemoMutation` enum: `RenameVcs`/`ChangeCounter`/`ChangeNotes`/
  `ChangeStatus`/`AddTag`/`RemoveTag`) and `.../🎛️apps/🌿️vcs/🦀️component.rs` were both last touched at
  commit `62152fabcc`, **2026-08-13 00:29:42 +0200** (inside this ticket's window, opened
  2026-08-12 15:02:49) — this is the SEMANTIC-MUTATIONS-OVERHAUL fan-out's mutation-vocabulary rename
  for vcs landing.
- `.../🎛️apps/🌿️vcs/🎮️commands/📈️counter/🦀️component.rs` and `.../🎮️commands/🩹️patch/🦀️component.rs` were
  last touched at commit `c31024cc6c`, **2026-08-10 23:04:11 +0200** — *before* both this ticket and the
  SMO rename — still calling the old struct-variant names (`VcsDemoMutation::SetCounter{counter}`,
  `SetTitle{title}`, `SetStatus{status}`, `SetNotes{notes}`, and `AddTag{tag}`/`RemoveTag{tag}` as
  struct-variant literals instead of tuple variants wrapping the named payload struct).
- Separately, `impl ArtifactApp for VcsPlayApp` had `fn seed(store: &mut ArtifactStore<...>)` —
  `ArtifactApp::seed` was removed from the trait entirely by ticket
  `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` M4 (confirmed: the trait now has `fn genesis() ->
  Vec<Self::Mutation>` instead, with a doc comment naming exactly this app's use case — "only apps whose
  fixture is itself a rich history... need this" — but `genesis` can only emit flat mutations, it has no
  way to express `CommitCheckpoint`/`CreateAlternative`/`SwitchAlternative`, so it cannot reconstruct
  branching checkpoint history at construction).

The SMO rename landed cleanly on the mutation definitions but never propagated to vcs's own app-layer
call sites — an in-boundary, cross-cutting break inside the subtree this fan-out exclusively owns
("nobody else, ever"), blocking `cargo check`/`cargo nextest` outright (lib AND test targets). Since
verifying "no ArtifactLink change needed" requires being able to actually build/test the crate, and the
fix was mechanical (rename call sites to match the already-renamed enum; no `🧬️mutations/**` edits), I
fixed it:

1. `🎮️commands/📈️counter/🦀️component.rs`, `🎮️commands/🩹️patch/🦀️component.rs`: replaced all
   `VcsDemoMutation::SetXxx{...}`/malformed `AddTag{tag}`/`RemoveTag{tag}` struct-literal constructions
   with the mutations facet's own exported builder functions (`rename_vcs`, `change_counter`,
   `change_notes`, `change_status`, `add_tag`, `remove_tag` from `crate::artifacts::vcs::mutations`).
2. `🎛️apps/🌿️vcs/🦀️component.rs`:
   - Removed the invalid `fn seed(...)` trait-impl block (no longer a trait member).
   - Removed the dead `demo_authors()`/`vcs_kernel::Author` helper (its only caller was `seed`; the
     app-level `handle_action("commitCheckpoint", ...)` path has no wire field for authors anyway — see
     gap below).
   - Rewrote `seed_vcs_demo_history` (moved into the `#[cfg(test)] pub(crate) mod testkit` region) to
     drive the SAME narrative (14 checkpoints, 6+ alternatives, forks off `c3`/`c4`/`c8`/`c2`/`c1`)
     through `VcsApp`'s public surface instead of a raw `ArtifactStore`: field edits go through
     `VcsCommand::TextEdit` (whole-projection diff, reusing `patch::text_edit`'s existing diff logic so
     several field changes still land as one undo-log entry, matching the original grouping);
     checkpoint/alternative operations go through `handle_action("commitCheckpoint"/"checkoutCheckpoint"/
     "createAlternative"/"switchAlternative", ...)`.
   - Wired `testkit::app()`/`app_with_registry()` to call `seed_vcs_demo_history` after construction, so
     every test-constructed instance starts pre-seeded exactly as before (all seeding call sites and
     their check-count invariants are unchanged; only *how* it's dispatched changed).

### Honest gap (documented in-code, not silently dropped)

- **Per-checkpoint authorship is lost.** `handle_action`'s `"commitCheckpoint"` arm (framework-owned,
  `🔌️plugin/🦀️component.rs:6418`) hardcodes `authors: Vec::new()` — there is no JSON-args path for real
  authors. The original raw-store version attributed each seeded checkpoint to alice/bob/carol; the
  rewritten version cannot. No existing test asserts on authorship, so this is a silent fidelity loss
  now made explicit in the new function's doc comment, not a functional regression.
- **Production auto-seeding at construction is no longer wired.** The old `ArtifactApp::seed` hook ran
  unconditionally in `VcsArtifactApp::new`/`with_registry` for every instance (test AND production). Its
  replacement, `ArtifactApp::genesis() -> Vec<Self::Mutation>`, structurally cannot express
  checkpoint/branch construction. I restored seeding for the test harness (`testkit::app()`/
  `app_with_registry()`), which is sufficient to keep all existing tests green, but a real user opening a
  fresh `vcs-play` document in production no longer gets the pre-populated history the old hook gave it.
  Fixing that for real needs a framework-level hook `genesis` doesn't provide — out of this plugin's
  boundary (`🔌️plugin/🦀️component.rs` is W1-owned, read-only here). Documented in-code
  (`🔖️DocumentHelpers` region) and flagged here for whoever owns
  `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` follow-up.

## Verification

- Baseline (before any edit): `CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-vcs --all-targets` → **RED**, 34 errors, all inside `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/**` (confirmed via `grep "🔌️plugins/🌿️vcs"` on the raw output — zero errors outside the plugin's own app layer at that point; an earlier run had also surfaced a transient `graph_core` unresolved-crate error in the `stdio` dependency, which cleared on retry — DKM's known math-dissolution churn, not attributed to vcs).
- After the fix: `cargo check -p semio-s-plugin-vcs --all-targets` → **0 errors**, 13 lib + 15 test warnings, all pre-existing/unrelated (unused imports in stdio-adjacent io-registry glue, one `unused extern crate` in `📦️glue.rs` now genuinely unused after removing `demo_authors`, an `unnecessary qualification`, a `testkit` ambiguous-glob warning inside the SMO-owned `🧬️mutations/component.rs` test module — none introduced by this change beyond the glue.rs one noted below).
- `cargo nextest run -p semio-s-plugin-vcs --no-fail-fast` → **53 tests run: 53 passed, 0 skipped**. Reproduced twice (not flaky).

## Files touched

- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs` (seed rewrite, dead-code removal, trait-impl fix)
- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🎮️commands/📈️counter/🦀️component.rs` (mutation constructor fix)
- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🎮️commands/🩹️patch/🦀️component.rs` (mutation constructor fixes)

No files created or deleted. `🧬️mutations/**`, `stdio`, and all framework files were read-only throughout.

## sharedFileRequests

- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs:11` — `extern crate semio_framework_os_kernel as
  vcs_kernel;` is now unused (its only real call site, `demo_authors()`, was removed as dead code once
  `handle_action`'s commitCheckpoint path made per-author attribution unreachable; the one remaining
  `vcs_kernel` mention in the plugin is inside a doc comment, not code). Cosmetic-only (a warning, not an
  error) — safe to leave for W5's normal glue cleanup pass rather than urgent.

## Concurrent-churn observations

- On dispatch, `git status --porcelain` showed 8 pre-existing **staged** (not mine) changes under
  `✏️s/🔌️plugins/🌿️vcs/**`: a `type Transient = semio_framework_plugin::NoTransient;` /
  `TransientMutation` pair added to `VcsPlayApp`'s `ArtifactApp` impl, plus doc-comment rewording
  ("persistent fields only" → "artifact-lane fields only" / "across persistent, shared-ui and local-ui
  classes" → "across the artifact, presence and config lanes") in six schema files. These are a repo-wide
  mechanical pass (adding the new `Transient`/`TransientMutation` associated types + relabeling doc
  comments for the ephemeral-lanes vocabulary) unrelated to this ticket's ArtifactLink work and unrelated
  to the SMO mutation rename. I left them untouched (already staged, not conflicting with anything I
  edited) and they remain staged as found.
- One `cargo check` run mid-investigation surfaced a transient `error[E0433]: cannot find module or
  crate 'graph_core'` inside the `semio-s-plugin-stdio` dependency (not vcs's own files) — matches
  `📌️important.md`'s documented DKM math-dissolution churn (`semio-framework-math` being split into
  `graph_core`/`geometry` crates repo-wide). Retried once, cleared on its own; not attributed to vcs.

## Status

`ucas-status: complete` — vcs needed no `ArtifactLink` migration (verified: no ad-hoc reference to
another artifact's content exists anywhere in the plugin). The plugin's pre-existing, ticket-window
compile breakage (an incomplete SMO mutation-vocabulary rename plus an unrelated framework `seed`→
`genesis` hook removal, both entirely inside this plugin's own exclusively-owned subtree) was fixed as a
prerequisite for verifying that finding; 53/53 tests pass, reproduced twice.
