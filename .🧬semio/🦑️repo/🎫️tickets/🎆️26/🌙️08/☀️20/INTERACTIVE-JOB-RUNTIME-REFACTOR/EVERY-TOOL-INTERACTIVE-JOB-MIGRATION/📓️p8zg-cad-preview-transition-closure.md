# P8zg CAD Preview Transition Closure

## Verdict target

The final P8zf P0 is repaired at source level. Every production command path
that changes or clears the persisted CAD engagement checkpoint now crosses one
operation-context-aware transition authority. This is audit-ready static
evidence, not a compile or runtime-pass claim.

## Exhaustive transition inventory

The whole CAD editor was scanned for engagement_session, checkpoint
serialization, document-reset construction, snapshot_of,
cad_config_from_runtime, and raw CadConfigMutation::Snapshot construction.
The production paths that can change or clear the checkpoint are:

- 🎮️commands/🤝️engagement/🦀️component.rs: submit, possible selection,
  repeat-last, abort, pointer-down and pointer-move transitions.
- 🎮️commands/🧰️utility/🦀️component.rs: active-utility switching clears a
  live session.
- 🎮️commands/📥️io/🦀️component.rs: successful scene import clears a live
  session before replacing the document.
- 🎮️commands/🗺️model-definition/🦀️component.rs: the adjacent
  setActiveExample document/runtime reset also clears a live session.

The internal engagement helpers in
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs
mutate only the operation-local CadPlayRuntime; their command callers above
perform persistence through the same authority.

All four cohorts now emit changed checkpoints only through
preview_transition_snapshot_of. The exhaustive command-tree scan found no
direct cad_config_from_runtime, no raw CadConfigMutation::Snapshot, and no
former preview_snapshot_of call.

## Mandatory persistence authority

preview_transition_snapshot_of is the single changed-checkpoint authority. It:

- serializes the candidate checkpoint and compares it to the persisted base;
- requires the dispatch context's real CadPreviewOperationIdentity only when
  the checkpoint changed;
- stamps app instance, parent document, operation id, operation generation and
  canonical base revision;
- rejects a negative base and uses checked increment in the exact
  0..=2_147_483_647 persisted descriptor domain;
- increments exactly once for a changed checkpoint and does not increment for
  an equal checkpoint;
- returns a typed invalid/conflict fault before an emit can persist a changed
  checkpoint when context is missing or generation is exhausted.

cad_config_from_runtime is no longer a public command helper and always
preserves the base stamp while packing non-session fields.
snapshot_of is the non-session snapshot authority: it now returns
Result<_, Fault> and rejects any serialized engagement-checkpoint difference.
Camera, sun, node, reference, locale, dislocate-option and engagement-input
callers use that checked route, so ordinary config snapshots retain the stamp
without a spurious increment.

CadPlayRuntime now owns active_utility_id, locale and terminology during an
operation. Utility and locale commands therefore no longer patch a raw packed
config. The active-example reset preserves those shell fields while its
session reset uses the transition authority.

Production operation identity continues to originate in
ArtifactEditor::handle from the actual document operation and canonical base
revision. No process identity, hash, pseudo-generation or global sequence is
introduced.

## Source fixtures

The CAD testkit gained drive_with_operation, which dispatches the real
CadCommand with an explicit public operation identity or an intentionally
missing context.

The following production-dispatch fixtures are in the main CAD editor source:

- production_transition_authority_routes_engagement_utility_and_import_without_noop_increment
  proves ordinary engagement start, utility clear, import clear and active
  example clear each route through production dispatch; every real checkpoint
  transition advances exactly once, while a utility/config-only equal
  checkpoint and engagement input preserve the stamp.
- production_transition_authority_isolates_two_app_aba_sequences proves
  A→B→A payload equality cannot collide with freshness, and identical
  generations/payloads from two app identities cannot cross-compare as fresh.
- production_transition_exhaustion_and_missing_context_fail_before_checkpoint_persistence
  exercises utility clear, import clear, active-example clear and ordinary
  abort at maximum generation, plus utility/import with missing context. Every
  case returns an error and leaves the caller's checkpoint, generation and
  operation stamp unchanged. It also proves the ordinary snapshot authority
  rejects a direct changed-session bypass.

The existing max/+1 cross-surface round-trip fixture remains intact.

## Static gate results

    rustfmt --edition 2021 <CAD transition cohort>
    rustfmt --edition 2021 --check <same cohort>
    => exit 0

    bun JSON.parse over every changed JSON descriptor in the seven repaired cohorts
    => exit 0; 6 current changed descriptors parsed

    exact CAD Rust/Proto/GraphQL/TypeScript/JSON generation coherence scan
    => exit 0; integer/i32/int32/Int/number, minimum 0, maximum 2147483647

    exact CAD engagement-session/reset/snapshot routing scan
    => exit 0; exhaustive and clean

    exact Note/Layout stale lifecycle vocabulary scan
    exact Process/Sourcing permanent-string-leak scan
    exact seven-cohort mutable-global payload-authority scan
    => exit 0; clean

    bun ./📜️script.ts verify interactivity
    => exit 0; DENY mode clean

    bun ./📜️script.ts verify interactivity tool-jobs --format json
    => exit 1, expected repository-wide residual:
       34 process-global payload candidates
       12 framework-reserved routes
       875 live command registrations
       bounded rows 9; factories 6; registrations 1; dispatches 1; aliases 4;
       self-tests 16

    git diff --check -- <seven repaired cohorts plus P8zc/P8zg>
    => exit 0

    corrected rg --pcre2 added-line debug scan over the same scope
    => no matches (rg exit 1)

## Unrun gates

Per instruction, no Cargo command, compilation/type/borrow/Send check, build,
unit/integration test execution, native/release/Wasm runtime, actual worker
launch, generated-code discovery/regeneration, cache deletion, modifying git
operation, or ticket metadata operation ran.

The production-dispatch fixtures above require the independently authorized
native/release/Wasm execution gates. The repository-wide tool-job ledger remains
expected-red and is outside this corrective cohort.
