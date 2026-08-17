# W4-D report — attribute, then fix only what is ours

Lane 4-D of `HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS`. Task: attribute the 5
`semio-framework-plugin --lib` failures and the 2 `semio-framework-os-renderer-wgpu --lib`
failures with `git log --date=iso` / `git log -S`, fix only what traces to this ticket's lanes
(3-F, 3-G, 2-0), leave everything else alone.

## Verdict: nothing is ours. No code changed.

All 7 failures pre-date this ticket's lanes or were introduced by concurrently-live peer tickets
touching files this ticket's lease explicitly marks **forbidden** (`🏪️store/**`) or that are
**out-of-lease foreign territory that predates 2026-08-16 by days** (the wgpu `Shell`
component). None of lanes 3-F (`TableWindowKit`/`render_rows`), 3-G (`history_command` +
`🏪️store` edit-id fix), or 2-0 (`document_app` split) touch any of the failing regions —
confirmed both by `git log -S` on distinctive strings and by reading the "Changed files" section
of `📓️w3-f-report.md` / `📓️w3-g-report.md` / `📓️w2-0-report.md`.

## Attribution table

| # | Test | Owning ticket | Evidence (`git log --date=iso` / `-S`) | Ours? | Action |
|---|---|---|---|---|---|
| 1 | `component::app::artifact_definition_contract_tests::registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically` | FULL-STDIO (still open) | Test body added by commit `3140b01d2c` (2026-08-16 02:17:58, "🗃️Full Stdio Artifact Standards, Codecs, Inferences, and Mutations"). The strict grammar it now fails against — `validate_capability_identity` in `🔌️plugin/🦀️component.rs:2624-2653` — was added ~75 min later by commit `dbcc4fa462` (2026-08-16 03:32:28, same FULL-STDIO Wave-0 remediation work per its message: "stdio registry contract audit"). Owner identity `s.stdio.ifc` has only 3 segments; the new `"resource"` rule demands `segments.len() >= 5 && segments[3] == "resource"`, which `owner.child("first-0")` (4 segments) can never satisfy — a self-inconsistency **within FULL-STDIO's own two commits**, not touched since. | No | Left alone |
| 2 | `component::app::artifact_definition_contract_tests::identities_and_locales_are_explicit_and_conflicts_do_not_overwrite` | FULL-STDIO (still open) | Same as #1: test added `3140b01d2c`, broken by the same `validate_capability_identity` grammar (`"localization"` rule: `segments.len()==5`) added in `dbcc4fa462`. `git log -S"identities_and_locales_are_explicit_and_conflicts_do_not_overwrite"` shows exactly one hit, `3140b01d2c`. | No | Left alone |
| 3 | `component::app::artifact_definition_contract_tests::plural_definition_carries_every_artifact_capability_without_a_dispatch_edit` | FULL-STDIO (still open) | Same pair of commits as #1/#2 (`git log -S` on the test name → `3140b01d2c` only; the validator → `dbcc4fa462` only). `assert!(definition.validate().is_ok())` fails for the same canonical-grammar mismatch. | No | Left alone |
| 4 | `component::plugin_runtime::plugin_builder_contract_tests::a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` (order-dependent: sometimes `a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them` fails instead — see below) | FULL-STDIO (still open) | Root cause is `register_child_store_factory` in `🏪️store/🦀️component.rs:557-567` (forbidden lease path), added fresh by commit `3140b01d2c` (2026-08-16 02:17:58, same FULL-STDIO commit as #1-3). It uses `Arc::ptr_eq(existing, &factory)` to decide idempotency, but every call to `register_typed_child_store_factory` (line 648) allocates a **brand-new** `Arc::new(...)`, so `Arc::ptr_eq` can never be true across two independent call sites even though the docstring at line 640 explicitly promises "Idempotent, same call-once-at-init contract." The plugin test helper `register_test_child_factory()` (`🔌️plugin/🦀️component.rs:16400-16404`) is pre-existing (last content touch `63686457bd`, 2026-08-16 02:50:31, "Plugin Dependencies…" ticket, and originally written well before that — Aug 8-12 per line-history) and is called from **two** test functions sharing the one process-global registry. Whichever of the two runs first in the shared test binary wins; the other always hits `Conflict{kind:"s.test.child"}`. Reproduced deterministically: ran the two tests together 3× (2 parallel, 1 single-threaded) — the loser flipped between the two tests every time, confirming a genuine race/non-idempotency bug in `register_child_store_factory`, not a flake in either test. | No | Left alone — root cause lives in `🏪️store/**`, off-limits per the ownership table ("consume as-is") |
| 5 | `component::plugin_runtime::plugin_builder_contract_tests::merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` | Peer store work (successor to closed MUTATION-OUTCOMES; conflict vocabulary) | The check that now rejects the test — `conflict.messages.is_empty()` inside `validate_persisted_conflicts` (`🏪️store/🦀️component.rs:2629-2709`) — was added **today at 2026-08-16 20:26:15** by commit `c8a29e41c5` (the newest commit on the branch, "⚙️Refactor OS store schema mutations and SPR command resolution with change merge policy"), entirely new code (`@@ -2440,0 +2629,81 @@`, i.e. inserted, not modified). The failing test's `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` body was last content-touched 2026-08-16 02:50:31 by `63686457bd` ("Plugin Dependencies, Artifact Contributions and Composite Mutations" ticket) — hours before the store validation tightened. The test seeds a `Conflict` with `messages: Vec::new()` and a hand-picked id `"test-degraded-conflict"` (not content-addressed), which the brand-new `validate_persisted_conflicts` now rejects on the `messages.is_empty()` arm before it would even reach the content-addressed-identity check. This is squarely `🏪️store/**` — forbidden per the ownership table — and postdates every HUB-SPACES lane's work on this file. | No | Left alone |
| 6 | `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments` | Pre-existing, unrelated (predates this ticket by 10 days) | `git log -S"window_silhouette_border_emits_notched_outline_segments" -- Shell/🧊️component.rs` → single hit, commit `23d0db68338`, 2026-08-06 05:42:37. Confirms lane 3-I's `📓️w3-i-report.md` attribution (§ "The other 3 failures"). Not touched since; no lane in this ticket edited this region. | No | Left alone — re-confirms 3-I's finding |
| 7 | `shell::shell_input_tests::standalone_multi_app_variants_resolve_their_declared_app` | Pre-existing, unrelated; tied to the still-open `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` surface-id migration | `git log -S"standalone_multi_app_variants_resolve_their_declared_app"` and `-S"resolve_playground_app_id"` both → single hit, commit `23d0db68338`, 2026-08-06 05:42:37. The fixture (`"puzzle2d-play"`) predates the surface-id convention (`"s.puzzle2d@1/*#editor"`) that `resolve_playground_app_id` (build.rs-generated `ProgramBridge`) now emits. Confirms lane 3-I's attribution. | No | Left alone — re-confirms 3-I's finding |

Note on #6/#7: lane 3-I's run (`📓️w3-i-report.md`) saw **3** pre-existing wgpu failures (313/3); this
run sees only **2** (314/2) — `shell::ui_prefs_themes_i18n_tests::load_ui_prefs_once_prefers_a_lock_over_storage`
(the env-var-lock one 3-I flagged as "plausibly flaky under cargo test's default multi-threaded
runner") now passes. Consistent with 3-I's own flakiness note; not investigated further since it
is not failing now and was never in this lane's failing set per the brief.

## Why #4 and #5 are especially worth spelling out

The brief singled out #4 as smelling like a global-registry collision our own lanes might have
triggered. It is a real collision, but the second registrant is the **other pre-existing test in
the same file** (`a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them`), not
anything HUB-SPACES added — no lane in this ticket registers `"s.test.child"` or calls
`register_child_store_factory`/`register_typed_child_store_factory` anywhere. The actual defect
(non-idempotent `Arc::ptr_eq` check on freshly-allocated `Arc`s) is a `🏪️store/**` bug that will
keep firing for **any** two tests in this binary that legitimately re-register the same child
kind — a real gap worth flagging to whoever owns `🏪️store/**` next, but it is out of this lane's
lease to fix.

#5's error message ("history repeats or omits conflict identity") does use MUTATION-OUTCOMES's
vocabulary as the brief anticipated, but the specific validation that fires was authored **after**
MUTATION-OUTCOMES closed, by the newest commit on the branch — so it's a live peer session
continuing store/conflict work, not a residual gap MUTATION-OUTCOMES's own closing summary already
disclosed. Either way it's `🏪️store/**`, forbidden regardless of which ticket is doing it.

## Commands run + result counts (real tails pasted into ticket-folder logs)

- `cargo test -p semio-framework-plugin --lib` → **217 passed; 5 failed** (stable across 2 runs;
  which 2 of {`a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`,
  `a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them`} shows up as the 5th name
  flips between runs — confirmed order-dependent, see #4 above).
  Log: `🧪️4-d-plugin-lib-test.txt`.
- `cargo test -p semio-framework-os-renderer-wgpu --lib` → **314 passed; 2 failed** (stable).
  Log: `🧪️4-d-wgpu-lib-test.txt`.
- `cargo test -p semio-s-plugin-space --lib` → **204 passed; 0 failed** (regression guard, green).
  Log: `🧪️4-d-space-lib-test.txt`.
- `cargo test -p semio-hub --lib` → **11 passed; 0 failed** (regression guard, green).
  Log: `🧪️4-d-hub-lib-test.txt`.

## Changed files

None. No edit was made anywhere in the repo — every failure traced to peer/forbidden territory
(`🏪️store/**`, FULL-STDIO's `🔌️plugin/🦀️component.rs` grammar addition, or a pre-existing wgpu
`Shell` fixture gap), so per the brief's "fix what is ours" instruction there was nothing in this
lane's lease (`🔌️plugin/**`, `📺️renderer/🧑️‍🎨️engine/**`) to change.

## sharedFileRequests

None filed. All 7 failures are entirely outside this lane's lease (`🏪️store/**` is explicitly
forbidden by the ownership table; the wgpu `Shell` fixtures are foreign and already attributed by
3-I). Nothing here is "trivially additive to a peer region that's idle" — `🏪️store/**` had a
commit land as recently as today 20:26, i.e. actively live.

## What is NOT done

- The `🏪️store/**` `register_child_store_factory` idempotency bug (non-functional `Arc::ptr_eq`
  check) is real and will keep breaking any test file with two independent registrations of the
  same child kind. Not fixed — out of lease, and the fix belongs with whoever owns `🏪️store/**`
  (FULL-STDIO or its successor).
- The `🏪️store/**` `validate_persisted_conflicts` requiring non-empty `messages` and a
  content-addressed conflict id breaks `merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads`'s
  fixture. Not fixed — out of lease; whoever owns the live `🏪️store/**` work today should either
  relax the check or the plugin-side test fixture needs updating in lockstep with it (which would
  require touching `🏪️store/**`, forbidden here).
- The FULL-STDIO `validate_capability_identity` canonical-grammar mismatch (#1-3) — FULL-STDIO's
  own ticket to reconcile between its own two commits.
- wgpu `window_silhouette_border_emits_notched_outline_segments` and
  `standalone_multi_app_variants_resolve_their_declared_app` (#6-7) — pre-existing, already
  attributed by 3-I, re-confirmed here, not fixed (out of lease/not ours).

Never claimed a test passes that wasn't actually run. All four commands above were executed in
this session; tails are pasted into the linked `.txt` logs.
