# Tutorial Document Field Native Join

## Actual Failure And Canonical Source

The full common Kernel run executed265 tests:263passed,2failed,0skipped. The tutorial failure is `tutorial_document_track_language_neutral_serde_parity`: deserializing the shared fixture fails because Rust still requires `artifactDsl`. The other action-bus failure belongs to the publication lane and is not part of this field correction.

The shared manifest `🧪️fixtures/🔣️tutorial-document-track.json` contains two Load entries with exact `documentDsl` strings (`根 "a"` and `Änderung\nβ`) and previousDsl, plus forward/reverse boundary vectors. Its schema requires documentDsl. The authored TS `TutorialArtifactEventKind::Load` and TutorialBase use documentDsl. The UI owner confirmed that canonical field explicitly; no schema/fixture change or compatibility alias is intended.

Rust still has TutorialBase.artifact_dsl at manifest2014 and Load.artifact_dsl at2325. Exact authored consumers are manifest's minimal tutorial fixture, Plugin's tutorial declaration fixture, and WGPU Shell's Load match/base read/four TutorialBase literals. The stdio SVG test name containing artifact_dsl is unrelated and stays untouched. This is a wire-field rename to document_dsl, not a change to ArtifactEnvelope identity or a fabricated document payload.

## Scope

Root approved the exact Rust definition/consumer rename. It is queued behind the coordinated Plugin source capture; no source change or fresh parity result is claimed yet. Existing serde behavior for optional TutorialBase document input remains otherwise unchanged; no new default or old-name acceptance is introduced. Full native localInteraction three-map capture/restore remains separate work.

## Coherent Source Release

After the previous shared snapshot completed and the compiler owner confirmed no Rust process/const RED had started, the exact rename was mounted in all three authored files. A scoped rg found no old artifact_dsl field references in them and `git diff --check` passed. No TS/schema/fixture payload was changed. The compiler owner received this coherent boundary before the next capture; native parity and full265 remain queued, not claimed passed.

## Actual Native Result

The subsequent full common Kernel R2 executed266 tests (the publication lane added one action-bus conservation law), all266passed with0failures/0skips in4.898s, Nx exit0. The actual output includes the tutorial language-neutral serde parity PASS. The full retained output is `📓️common-kernel-full-r2-native-2026-08-28.md`, whose footer was read directly. This supersedes the earlier263/2 result for this source snapshot without exclusions. WGPU field consumers are source-coherent only; no WGPU build/runtime or full native tutorial capture/restore credit follows from the common Kernel gate.
