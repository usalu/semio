# Companion Compiler Checkpoint

The normal canonical parent check (handle1749/PID53184, typing-net-parent-check-reboot11220615) settled status1/signalnull at 2026-10-30T00:39:23.424Z. The durable observed outcome is in 🗑️generated/typing-net-parent-check-reboot11220615.outcome.json.

The real owner recipe runs production native, production wasm32-wasip2, then the exact semio-framework-tool-machine-tests companion --tests. Output contains two completed production check phases (18.95s and38.78s) before the companion fails. The selected failed harness is the actual companion Cargo package, not a substitute production-only scope. No assertions ran in this check.

Six concrete errors are in existing neutral-law callers:
- Gesture laws: ActorId(text(actor).to_string()) at163 and167 supplies String instead of SharedUtf8.
- Typing laws: same constructor at161.
- Scrub laws: same constructor at138.
- Scrub exhaustive ledger exploration at202 supplies ScrubInput instead of the current &mut Option<ScrubInput>.
- Scrub persisted foreign-chart fixture at235 supplies &str.into() to ActorId rather than the explicit ActorId constructor.

Exact test source root: 🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️unit/. Only 🌊️gesture-tool-laws/🦀️.rs, ⌨️typing-laws/🦀️.rs and 🫧️scrub-laws/🦀️.rs were corrected: direct text.into() for SharedUtf8, explicit ActorId("a".into()), and &mut Some(input.clone()) at the one outdated ledger send. No production type, compatibility constructor, ownership fence, grant, fixture expectation or schema was changed.

This is a genuine compiler red against the existing language-agnostic typing/scrub/gesture corpora. The same corpora already pass independent XState and Ajv validation in the complete TypeScript suite (37 tests, 25,773 assertions, status0). The correction adapts test callers to current owned APIs; it does not change expected semantic outputs.

One canonical normal check rerun is justified by these six exact causal fixes. Other three recovery owners remain untouched. The full parent runtime recipe still selects both production PACKAGE and COMPANION, and must enumerate the required production reader/transfer/growth tests before runtime acceptance is claimed.

## Actual Expanded Canonical Check Acceptance — 2026-10-30 01:26 UTC

Corrected normal check handle74598/PID101518 settled observed status0/signalnull at2026-10-30T01:26:00.614Z. Its real owner recipe now completes four compiler phases: native production PACKAGE (0.48s), wasm32-wasip2 production PACKAGE (0.69s), native PACKAGE --tests (3.25s, explicit main package Checking line), and actual COMPANION --tests (1.24s, explicit semio-framework-tool-machine-tests Checking line). Nx reports0/1cachehits. Durable outcome/log typing-net-parent-check-companion-fixed-reboot11220615.

This confirms the six corrected companion fixture callers compile and adds genuine native production-unit type-check evidence while preserving the existing native/wasm/companion steps. It does not execute assertions, link test binaries, or prove the hidden full-runtime Cargo101 cause. The runtime recipe selects both packages in one invocation; these check phases select them separately, so no exact combined feature-unification/runtime acceptance is inferred. Root-owned diagnostic-driver acceptance remains required before the held full runtime rerun. OS30547 remains live in normal graph preparation/custody; no new native assertion result.
