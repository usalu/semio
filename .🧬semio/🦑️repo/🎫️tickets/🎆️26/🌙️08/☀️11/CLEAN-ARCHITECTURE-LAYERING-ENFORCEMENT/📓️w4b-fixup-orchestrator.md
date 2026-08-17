# Wave 4b fix-up (orchestrator)

## Fixed: 3 scoping gaps the verify agent found
1. `💻️os/🦀️component.rs:2914` and `🖥️host/🦀️component.rs:3038` — both
   `OS_SPACE_SCHEMA` consts had their identifier already renamed by an
   earlier pass but the VALUE still said `"s.space"`. Fixed both to
   `"os.space"`.
2. `💻️os/🟦️component.ts:2235,2245,2258,2276,2284` — 5× `schema: "s.workflow"`
   → `"os.workflow"`.

## Corrected: a real scoping ERROR in my own wave-4b dispatch
The EngineCanvas agent renamed `"s.play.workflow"`/`"s-play"` →
`"os.play.workflow"`/`"os-play"` in its test fixtures. Investigation showed
this was WRONG: `S_PLAY_CONTROLLER_ID = "s-play"` and `S_PLAY_BODY_WORKFLOW
= "s.play.workflow"` are defined and owned by the `🪐️space` PLUGIN itself
(`✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs`) — the same
"s"-plugin naming convention as `"cad-play"`/`"process3d-play"` elsewhere,
not an instance of the generic os-schema-id violation this wave targets.
Per the plan's own design decision ("plugin-owned s.* app ids stay"), these
should never have been renamed. Reverted `EngineCanvas/🧊️component.rs`
back to the original plugin-owned values — confirmed this also resolves the
verify agent's flagged inconsistency (the renderer react test file and the
plugin consts were correct all along; only my EngineCanvas rename was the
outlier).

## Confirmed correct, no change needed
`✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs:121,127,283` —
`kind_id = "s.space"` is a plugin-owned artifact-kind identifier, a
DIFFERENT namespace from the `S_SPACE_SCHEMA` os-level wire schema (now
`"os.space"`) passed as a separate argument to the same `create_draft()`
call. Confirmed by reading `create_draft`'s call shape
(`create_draft(kind_id, schema_id, ...)`) — correctly left untouched.

## Verification
- `cargo check -p semio-framework-os` — clean.
- `cargo check -p semio-framework-os-renderer-wgpu` — blocked only by
  unrelated stdio codec `OpText`/`OpBinary` trait-bound churn (confirmed
  zero errors mention EngineCanvas/play.workflow/s-play).
- Repo-wide grep for `"s.space"`/`"s.workflow"` (excluding the confirmed
  legitimate `kind_id` sites) — zero survivors.

## Wave 4b status: COMPLETE
All s.*→os.* schema-id renames landed and verified; the one incorrect
rename (EngineCanvas) was caught and reverted before it could propagate
further. Proceeding to Wave 5.
