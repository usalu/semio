# Shared Generation and Parameter Intent Checkpoint

## Ownership and Integration

The former Procedural3d-local immutable generation implementation now lives in `framework/os/playbook/🧬️generation`, exported as `flow::playbook::{GenerationPlayRoot, GenerationRootRetirement}`. The old implementation files were moved, not retained as a compatibility adapter. Both Procedural3d and Procedural2d snapshot, artifact, and diff models use the shared root. Their text/binary cold builders explicitly request unique-root mutation; shared roots cannot be mutated or silently cloned. Procedural2d snapshot retirement now delegates to the same typed Flow and generation retirement frontiers as Procedural3d.

The shared root keeps transparent serde wire semantics, O(1) Arc sharing, private ManuallyDrop ownership, guarded final-owner Drop, and native-iterator/bytewise JSON retirement. Four generation native laws moved with the implementation. The cold generation-authoring paths still clone state explicitly and are not credited as retained interactive edits.

The exact parameter payload moved from the Flow app into `framework/os/flow/🎚️parameter/📨️intent`, exported as `flow::graph_parameter::{SetGraphParameter, SetGraphParameterRetirement}`. The Flow command module reexports it. Widget IDs remain uncapped; values must be finite; optional surface IDs must be nonempty. Unknown whole-fixture fields are rejected. The retained disposer moves the two strings into byte owners and retires them with exact grants, including one-byte Unicode progress. Its terminal guard cannot recursively destroy retained strings during unwinding.

The payload also implements typed canonical JSON traversal for semantic parity. This is **not** its command-wire witness: `app_commands!` uses format byte + variant ordinal + pack record. The dedicated paged command worker must verify that actual binary format; generic whole-input reserve/decode wrappers remain unsuitable for the full identifier domain.

## Evidence

- Canonical Nx interactivity self-tests passed 785, with 33 owners / 254 custom rows / 25 generic rows. This is a whole-source checkpoint including concurrent peer work, not 785 new tests from this packet.
- Canonical Flow `test-source` passed after the payload move and again after the retirement oracle extension: four parameter cases, ten hostile input rejections, two Node Buffer byte-accounting oracles, three hostile fixture rejections, and the existing independent fast-json-stable-stringify oracle. The strict fixture schema links to the payload schema.
- Targeted `git diff --check` passed for shared generation, parameter intent, Procedural integration, Flow fixture script, and root verifier changes.
- Native execution remains coordinator-owned and pending for these additions. Exact Flow-core nextest filters: `test(generation_root_)` (four tests) and `test(graph_parameter_intent_)` (three tests). Existing `test(graph_parameter_preserves_)` belongs to the held earlier numeric-helper gate.

## Remaining Boundaries

No new Procedural3d or Procedural2d parameter factory is registered yet. Complete candidate/inverse construction, global allocation admission, actual pack-wire verification, exact Store publication, runtime latest-wins cancellation, and mounted undo/preview still require implementation and execution. The shared copier's local allocation counters are not a global memory reservation or a maximum-envelope timing certification. Existing cold decoder/replay and unrelated generation edits are not reclassified as fully interactive by this model move.
