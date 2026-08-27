# Flow Session Ownership Review

## Executed Evidence

The coordinator independently ran `semio-framework-os-flow-core:test-source` through Bun/Nx. Exit 0: one strict language-neutral fixture, three hostile rejections, 42,405 actual semantic bytes, grants 1/64/4096, and the existing fast-json-stable-stringify oracle. The fixture includes 16 KiB text with 64 KiB reserved capacity. Log: `🧪️coordinator-flow-session-source-2026-08-27.txt`.

Three authored native session laws cover exact byte totals across a worker transfer, empty reserved capacity, and strict live-owner Drop. They have not run yet. Full neural native 42/42 is separate evidence, not a Flow integration pass.

## Source Review

The coordinator read FlowHostRetirement, FlowEvalSession, and all three new native laws. Session retirement moves exact collection owners, drains ordered collection edges, delegates nested Dictionary/cache ownership to verified neural cursors, and charges actual string bytes rather than unused capacity. Its strict ManuallyDrop state prevents an unfinished drop from recursively destroying the retained payload.

The lifecycle is not yet bounded end to end. `begin_close` still calls the process-global geometry retention function, which clones/unions all sessions' handles and replaces the global retained set. The new fixture does not populate other live sessions, so it cannot establish that boundary. Session tick/status creation, baseline cloning, whole preview JSON parsing, and collection retain loops are also outside the new close-cursor proof.

FlowHostRetirement still has a sparse HashMap extraction for kind metadata and direct destruction of opaque evaluation/ghost/projection owners. These are explicitly uncompleted boundaries, not acceptable one-item credit. The executor owns their replacement. The shared Flow copier's allocation admission and native checks precede concrete parameter-feature registration in Flow/Procedural2d/Procedural3d.

String-key BTreeMap pop_first avoids a sparse-capacity scan during retirement; insertion/lookup with arbitrarily long keys still needs bytewise retained work where interactive.
