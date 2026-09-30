# Flow Port Retirement Final Review

Read-only follow-up; no builds or tests executed. The implementation owner is currently authoring the neutral seam.

## Existing Concrete Boundaries

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2678` immediately destroys the geometry-port Box during FlowHostRetirement construction after calling sealing-only close. `:4073` does the same during FlowEvaluationSession.begin_close. The port supplied by Session.port owns its own allocated authority SessionState in addition to its Box. Both allocations must remain owned until byte admission. Zero grants must preserve the handles; sealing must not free these shells.

## Interface Constraint Sent To Owner

`🌊️flow/🌐️geometry/🦀️.rs:11–18` currently exposes only `close(&self)` with no retained terminal query, byte demand, or shell close step. Add the retained seam to this neutral first-party interface and retain the port inside both host/session retirement state until terminal. Charge the actual concrete Box Layout after port-owned allocations are terminal; do not charge inline Option storage.

The port is a separately admitted authority, whereas registry captures share the producing root authority. Unmodified full SessionCapture close waits for the native family cursor to become terminal after final authority-state extraction. A host generally retires its port before retiring the root registry/Session; requiring family terminal for port terminal would therefore create a close-order cycle. The port path must release its sealed authority's claims/jobs/state shell while explicitly handing family-reader ownership back to a still retained family owner. The producing root capture remains responsible for actual final native family and shell retirement. A fixture must prove the root remains open and usable after the port reaches its own allocation-empty terminal state, including insufficient state-shell and Box grants.

This constraint and concrete boundaries were sent directly to the Cargo implementation owner. Final settled source review will be appended after handoff.

## Owner's Proposed Reader Handoff

The Cargo owner proposes a SessionPort with a child authority and a retained producing-root SessionCapture anchor. Sealing transfers only child claims/jobs. Granted child release admits its actual authority-state allocation while the anchor guarantees family allocations remain retained. Anchor release then either delegates to a surviving producing reader with zero actual bytes or inherits genuine final family payload/shell retirement if the producer is gone. A neutral GeometryPortRetirement retains the concrete Box until port allocation-empty terminal, then admits its actual Layout. This architecture addresses the observed close-order cycle; it is a proposal awaiting source verification.

Fixture adaptation locations sent to the owner include the existing explicit sibling/unused close calls in session-lifetime tests, RecordingPort's old close-only stub, and the denied terminal Session.port temporary near line 235. A denied port still requires explicit shell retirement even though tessellation refuses immediately.

## Source Review Of Implemented Seam

The current `🌊️flow/🌐️geometry/🦀️.rs:23–55` wrapper retains its Box in ManuallyDrop, preserves ownership on zero grants, validates nested reported budget and terminal acknowledgements, and separately admits the concrete Box Layout before release. Its Drop refuses unfinished ownership. Host retirement and FlowEvaluationSession now store retained wrapper fields and include their absence in terminal predicates; begin-close seals the port without dropping it.

`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:1464–1501` implements the proposed separate authority plus producing-root capture. The anchor retains all family handles while the admitted child SessionState is consumed. Its actual shell receipt is installed only on the immediately consumed Arc allocator. Zero grants, insufficient shell bytes, and shared family pause preserve the owners. The next phase drives the anchor's actual reader/final-family cursor. SessionPort Drop requires both authority absence and capture emptiness. No new concrete defect was found in those paths.

## Remaining Cold-Driver Check Sent To Owner

The neutral wrapper itself exposes next byte demand and uses it for its explicit cold method. FlowHostRetirement and FlowEvaluationSession cold drivers currently use fixed pages; a valid supplied GeometryPort Box or own shell larger than the page can remain blocked forever. The nested port demand must reach those cold drivers without inflating an externally supplied interactive grant. This was sent to the Cargo owner for its settled handoff. No runtime test or complete acceptance is asserted here.

## Playbook Capture Adaptation

Root's latest `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:365` stores SessionCapture. Construction at line 384 retains the actual Session used to register Math/BREP operators. Closing first transfers import source/handle storage into ValueRetirement, releases the shared registry handle, drains RegistryRetirement, and then drives capture retirement. Terminal checks at line 447 now require actual capture emptiness, so native family terminal alone no longer acknowledges owner completion. On normal terminal destruction the capture, registry, imported payload, and both retirement frontiers are empty. No further concrete normal-close defect was found in this adaptation. Parent's red/green test execution remains its own evidence.

## Cold Driver And Fixture Follow-Up

Latest FlowHost cold retirement at host source line 2962 adapts its grant to the retained port's actual next byte demand, resolving the large-port fixed-page concern. FlowEvaluationSession.retire_cold already uses usize::MAX grants at line 4105, so the earlier statement that its cold path uses fixed pages was incorrect and is withdrawn. Interactive methods still pass the caller grant unchanged.

The updated native session-lifetime source explicitly retires existing ports, preserves a root-live sibling law, retains an orphan port after producing-root drop, and exercises an 8192-byte concrete port Box with zero grants and half/exact allocation grants. Its expected boundaries come from the portable portBoundary fixture. These fixtures were inspected, not executed here. No remaining concrete source defect was found in the latest port/capture paths. The Cargo owner subsequently confirmed this reviewed source handoff, including denied sealed-parent ports and producing-root-gone ownership. Source review is complete; runtime validation belongs to the execution owner.
