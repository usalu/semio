# Resumed Draw Source Audit

Read root and s AGENTS, completion-audit.md, IO report, native and TypeScript project/router sources and repository artifact-package provider/admission. Scoped git diff --stat for Draw and composition is empty: the reviewed implementation is present in the checkout. No duplicate build was started.

## Current Routes

- repo run @semio-tech/draw-js:test --lang=en -- io-descriptors is valid: the single selector invokes neutral parity and strict IO typecheck. Full test invokes both too.
- repo run @semio-tech/draw-drawing-rs:test --lang=en -- --lib io_descriptors_match_neutral_fixture is the current declared focused native route. The provider strips a recognized test level, validates remaining native arguments and forwards them through repository admission. This selector is not an artifact-owned special command.
- The native test router selects eleven Drawing family packages including the root artifact. A focused filter limits executed tests but does not guarantee only the root crate compiles. Require actual output showing io_descriptors_match_neutral_fixture ran; admission or zero tests is insufficient.
- repo run @semio-tech/draw-drawing-rs:test --lang=en remains required for the full eleven-package suite. repo run @semio-tech/draw-drawing-rs:check --lang=en uses the current manifest and repository preparation/admission. Both depend on graph-generate.

## Source Findings

The native root test reads the retained neutral fixture relative to its root module, compares actual io().entries, and rejects unsupported pairs. The manifest has test-only serde_json. Current Dialect fields, fidelity Debug values and optional sniff registration agree with the implementation. No grounded API or target-name defect was found in this focused review. Historical capacity-blocked reports describe the prior checkpoint, not current host state or new results. Resumed owners must supply executable proofs.
