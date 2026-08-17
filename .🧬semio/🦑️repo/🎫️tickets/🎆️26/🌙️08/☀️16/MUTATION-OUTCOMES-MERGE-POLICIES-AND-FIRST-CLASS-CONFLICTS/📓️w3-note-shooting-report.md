# W3 — note + shooting: MutationOutcome conversion (lane R5)

## Scope done

- Note (`✏️s/🔌️plugins/🗒️note/**`): all 33 `🔺️diff` leaves converted to
  `protocol::MutationOutcome<NoteDiff>` per the fan-out recipe, plus their 33 `🦠️mutation`
  siblings (return-type only), plus the facet's `apply_note_mutation` call site.
- Shooting (`✏️s/🔌️plugins/🎥️shooting/**`): all 31 `🔺️diff` leaves converted to
  `protocol::MutationOutcome<ShootingDiff>`, plus their 31 `🦠️mutation` siblings.
- Hand-written `impl Mutation<P>` (config/presence, 4 total, none had `fn validate`):
  `NoteConfigMutation`, `NotePresenceMutation`, `ShootingConfigMutation`,
  `ShootingPresenceMutation` — all now `fn diff(&self, base) -> MutationOutcome<P>`, wrapping
  their whole-snapshot success value in `MutationOutcome::new(..)`.
- No `fn validate` overrides existed in either lease (grep confirmed empty before and after).
- Call-site fixes: `🧬️mutations/🦀️component.rs` facet helpers/tests in both plugins
  (`apply_note_mutation`, `assert_mutation_diff_absorb_law` call site in shooting's facet test,
  `note_config_operation_backwards_...` / `shooting_config_operation_backwards_...` tests) updated
  to `.into_parts().0` / `.diff()` where a raw `Diff` was expected. Shooting's `ArtifactBuilder`
  (`🧬️schema/🦀️component.rs:434-436`) already expected the outcome-based signature — no change
  needed there.
- Verb-family messages implemented per kind (not bare wraps): create→Fatal `duplicate-id`
  (+ Fatal `invariant` on unknown/non-group container for `create-block`); delete(+plural)→Error
  absent / Warning `partial`; rename/change/edit/replace→Error/`no-op`; move/resize/drag/
  scale/rotate→Error/`no-op`/Fatal `invariant` (non-finite/non-positive, incl. bulk `partial`
  variants for `drag-assets`/`scale-assets`/`rotate-assets`); reorder→Error/`no-op`; insert
  (table row/col)→Error; remove (table row/col)→Error/`no-op` at the 1-row/1-col floor;
  duplicate(-block(s))→Error source/Fatal id-collision (+`partial` for plural); `set-active-*`→
  Error on an unknown addressed id / `no-op` when already active. The 7 root-scoped scalar
  setters in each plugin (note: grid visible/spacing/subdivisions/opacity, snap enabled/spacing,
  pencil width, eraser radius; shooting: sun enabled/azimuth/elevation/intensity, shadow enabled,
  ambient intensity, material roughness) return message-free `MutationOutcome::new(..)` under the
  contract's shrink-only allowlist for root `change-<artifact>-<field>` kinds.
- Facet `🧪️Tests`: no `assert_missing_target_is_error`/`assert_fatal_never_applies` pairs existed
  yet for either facet; not added this pass — see Known gap below.

- Added a `🔖️OutcomeLaws` test region to both facets' `🧪️Tests` (mirroring
  `✏️s/🔌️plugins/🕸️dag/…/🧬️mutations/🦀️component.rs`): note gets 16 tests
  (`assert_missing_target_is_error`/Fatal-never-applies pairs covering create/delete(s)/rename/
  change/move/resize/drag/duplicate/insert/remove/edit/replace); shooting gets 12 (covering
  create/delete/rename/change/reorder/drag/scale/rotate/replace/set). Also fixed 3 more raw-`Diff`
  call sites this surfaced (`change_grid_spacing(..).diff(&base)` in note's
  `root_scalar_inverse_and_absorb_laws` test, plus the config-mutation `.diff()` call sites in
  both plugins' `🎚️config/🦀️component.rs` tests) — all now `.into_parts().0`.

## Verify — real counts

`cargo check -p semio-s-plugin-note` and `cargo check -p semio-s-plugin-shooting` (crate names
from each plugin's `📦️packages/🦀️rust/Cargo.toml`): **zero errors in note's or shooting's own
files** in every run this session. Both crates currently fail to *link* only because their shared
dependency `semio-s-plugin-stdio` doesn't compile — confirmed unrelated to this ticket (no
`MutationOutcome`/`Mutation`/`Severity` symbols involved; `cannot find trait DiffCodec/OpBinary in
this scope` + ~20 `OpBinary`/`OpText` trait-bound-not-satisfied errors on legacy format enums:
`StlMutation`, `SemioAnimationMutation`, `LasDiff`, `PlyDiff`, `GifDiff` (87a/89a), `ObjDiff`, at
e.g. `✏️s/🔌️plugins/🗄️stdio/…/🟪️stl/…/✏️editor/🦀️component.rs:51`,
`✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/…/✳️animation/🧬️schema/🧬️mutations/🦀️component.rs:218`).

This is live, in-flight churn from another lane (matches
`FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`, whose ticket folder shows
concurrent edits this session): polling `cargo check -p semio-s-plugin-stdio` repeatedly during
this session showed the stdio-owned error count falling **197 → 176 → 170 → 23 → 20 → 8** with
zero action from me — someone else was actively fixing it live. It then held steady at exactly 8
for 4 consecutive polls, all 8 in one file with a missing trait import:
`✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/🦀️component.rs`
(`impl OpText for SemioAnimationMutation` / `impl OpBinary for SemioAnimationMutation` at lines
218/351, `cannot find trait OpText/OpBinary in this scope` — looks like a one-line
`use protocol::{OpText, OpBinary};` is missing after some glue/import reshuffle) plus 4 downstream
`✳️animation/✏️editor` and `👁️viewer` trait-bound errors from the same cause. Out of my lease
(`🗄️stdio`, not `🗒️note`/`🎥️shooting`) — not touched. Real final pasted tail (both plugins, same
blocker):

```
error: could not compile `semio-s-plugin-stdio` (lib) due to 8 previous errors; 230 warnings emitted
```

`cargo test -p semio-s-plugin-note --lib` and `cargo test -p semio-s-plugin-shooting --lib`:
same story — 0 note/shooting-owned errors, build blocked transitively by stdio (197 and 176
stdio errors respectively at the time each ran, before stdio's count kept dropping). **No test
binary has produced a pass/fail count yet** because the workspace can't link while stdio is red.
This is not a claim of passing tests — re-run both commands once stdio's lane finishes; my code
should build clean at that point based on every note/shooting-scoped check being error-free.

## Files touched

- 33 note `🔺️diff` + 33 `🦠️mutation` leaves under
  `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/`
- 31 shooting `🔺️diff` + 31 `🦠️mutation` leaves under
  `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (`apply_note_mutation`)
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (test call site only)
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs`

Logs: `🧪️w3-note-shooting-note-check.txt`, `🧪️w3-note-shooting-shooting-check.txt`,
`🧪️w3-note-shooting-note-test.txt`, `🧪️w3-note-shooting-shooting-test.txt` (all in this folder).
