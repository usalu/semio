# Playbook Geometry Ownership

The concrete procedural Playbook extension now builds one instance-owned S geometry session and a shared operator registry containing the real Math and BREP implementations. Generic Flow receives these capabilities explicitly. The existing VCS instance-owner handle supplies both retained commands and rendering; artifact mutations remain SetPayload operations with inverses.

BREP registration and registry construction contain no asynchronous work. Their canonical APIs are synchronous, and actual native, wasm guest and composition-law callers have been updated without a ready-future wrapper. Mesh registration receives the same session.

Imported geometry persists the original format, interchange data and tolerance. Preview and export reconstruct session-local handles from that source. The old prefixed-handle predicate has been replaced with the engine's actual 64-character lowercase hexadecimal identity. The schema was extended before implementation; neutral vectors cover durable import, two parametric previews and hostile identities, with first-party JSON output compared to Serde JSON.

The initial focused native command failed compilation during the authored schema/test-before-owner stage, before any law assertion ran. It encountered the missing concrete dependencies and a fixture include path; both are corrected. This is not a passing runtime claim. Actual native execution and genuinely budgeted Session retirement are pending. Session close uses the first-party retained retirement step, supplied by the execution agent; RegistryRetirement is driven before that Session retirement.

## Owned Files

- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema/🔣️.json
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧫️fixtures/🌐️geometry-lifetime/🔣️.json
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📜️script.ts
- Cargo.lock (only this extension's dependency row)

## Synchronous Registration Caller Updates

- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️evaluate-budget/🦀️.rs
- ✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🔌️port-types/🦀️.rs
- ✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🌿️catalogue/🦀️.rs
- ✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🧩️generation3d-example-geometry/🦀️.rs
- ✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🧊️generation3d-app-laws/🧪️tests/🔬️flow-operators/🦀️.rs
- ✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🔁️generation3d-incremental-eval/🦀️.rs

## Review Corrections and Scope

The imported source cache now reuses geometry within an instance, hands obsolete source/handle strings to first-party ValueRetirement, clears changed/invalid/removed sources, and publishes the exact current preview/import claim union before host retirement and after export source resolution. The independent replay oracle compares exported geometry, not transient handles; the engine mints handles using its persistent-label counter.

Retained command routing now uses the real instance owner, but the media codecs remain the existing synchronous BREP implementations. This increment does not claim cancellable, sliced import/export computation: no retained codec API exists in the inspected engine. Generic Flow evaluation and native tessellation already have retained stepping APIs; fully retained media encoding/decoding is a separate codec implementation task. Session and registry destruction in this increment must still meet the real item/byte grants.

The new geometry-contract and canonical-architecture owner targets execute the neutral AJV oracle and six exact native laws. The package test script now awaits the runner, so it cannot claim success while work is still pending.

Additional owned paths:

- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️geometry-lifetime/🟦️.ts
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📋️project.json

Latest geometry-contract run: portable AJV oracle passed one test and 10 assertions in 71 ms. Native compilation reached the concrete extension and found only the two still-pending Session retirement methods; no native assertion ran. The paired implementation agent owns those methods.

The first native lifecycle law ran and correctly rejected fallback geometry. Temporary `[DEBUG]` runtime output exposed the real cause: blanket scalar parameter injection replaced typed number-channel dictionaries, so Polygon reported `missing input: radius` and downstream Extrude reported `missing input: wire.handle`. Parameter application now sets named sliders and only applies a neuron's own object keyed by its widget identity. No prefixed-handle compatibility or weaker preview assertion was added. The next run must produce evaluated geometry for both neutral fixture parameter rows.

The owner preserves phase-boundary receipts without synthesizing an extra item after an underlying cursor has spent its grant. Full native verification is pending the shared extension lifetime compile-ready handoff.

Fresh native runtime produced real Polygon/Extrude handles and successful tessellation for height3/radius0.5/sides6 and height6/radius0.75/sides8. The independent coordinate extent assertions passed, together with durable-source replay and instance isolation. Temporary `[DEBUG]` geometry logs are removed. The full law still fails at final retirement with `neural.operator-retirement-not-implemented`; concrete Math/BREP capture contracts are being repaired against the strict current operator interface. No passing lifecycle claim is made.

After strict BREP SessionCapture retirement was implemented by the paired execution owner, the final geometry-contract Nx target passed1m33s: one independent AJV test/10assertions and all six exact native laws, including real coordinates, durable import replay, instance isolation, export undo and complete instance registry/session retirement. `[DEBUG]` instrumentation was absent in this run. The broader22-law native library is now running because registration, parameter routing and retained command ownership changed.

The first broader library command was rejected before native assertions: Nextest does not accept the supplied libtest-only test-threads argument. The corrected command uses the existing owner test quick target with its native runner controls. No pass is claimed for the rejected command.

The corrected full native library target passed all22/22tests, zero skipped (Nextest assertions0.402s, Nx7m9s including build/queue). This broad proof covers the final instance lifecycle, typed parameter application, retained command routing, mutation inverses and original extension helpers in one actual native library snapshot.

### Final Allocator Shell Admission

The new schema-first instance shell law failed before the SessionCapture correction: native payload drain incorrectly emptied the owner. After the correction, all seven selected native geometry laws and the independent Ajv contract (one test, fifteen assertions) passed in the actual uncached Nx run (3m54s). Runtime debug evidence measured one final release item, 944 actual shell allocation bytes, and terminal=true. The temporary debug line was removed after recording this evidence; clean and full twenty-three-law runs remain pending.
