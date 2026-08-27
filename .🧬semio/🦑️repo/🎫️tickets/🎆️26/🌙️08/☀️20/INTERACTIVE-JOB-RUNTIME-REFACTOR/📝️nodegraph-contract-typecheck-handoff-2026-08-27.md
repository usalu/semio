# NodeGraph Contract and Typecheck Handoff

## Scope and Outcome

Repaired the NodeGraph-specific TypeScript diagnostics without weakening the compiler configuration or using casts to conceal schema mismatches. The coordinator's complete canonical renderer typecheck r2 still fails with 385 TypeScript diagnostics, zero in NodeGraph, versus 415 previously. The local captured stream contained only 112 diagnostics because it was truncated; that was not a valid whole-run total. The coordinator's complete retained output is authoritative.

## Changes

- Matched workflow node generic types, scene status metadata, optional graph selection-domain capability, and exact menu reference arguments to their codebase-owned contracts.
- Forwarded the existing diagram connectability/reconnectability properties through its React facade.
- Gave the spotlight input a stable scoped ID and its existing localized accessible name.
- Repaired both hover consumers to decode the actual native handle pick identity (`node@port`), instead of reading a nonexistent `portId` property. Splitting at the first separator preserves Unicode and additional separators in port identifiers.
- Updated four test fixtures to the actual typed graph scene shape rather than obsolete JSON-string fields.
- Added a strict language-neutral pick-target schema and eight cases, checked with Ajv and an independent regular-expression oracle.

## Executed Evidence

- Canonical Nx renderer focused DOM command: 13 passed, 0 failed, 342 skipped, 355 total; 45.96 seconds. This includes graph slider accessibility, exact small parameter dispatch, and pick identity checks. Log: `🧪️nodegraph-contract-dom-r1-2026-08-27.txt`.
- Canonical Nx renderer typecheck: exit 1. Local truncated capture: `🧪️nodegraph-contract-typecheck-r1-2026-08-27.txt`. Complete coordinator r2: 385 remaining diagnostics; zero NodeGraph diagnostics, with 26 NodeGraph and four graph fixture diagnostics removed. Authoritative log: `🧪️coordinator-renderer-react-typecheck-r2-2026-08-27.txt`.
- Coordinator full renderer rerun: 470 passed, 0 failed, four files, 22.35 seconds. Log: `🧪️coordinator-renderer-react-full-r2-2026-08-27.txt`.
- Targeted `git diff --check` passed for the changed NodeGraph, mesh facade, diagram facade, renderer facade, and test paths.

## Limits and Next Work

These are source and actual DOM results, not a fresh-WASM or mounted backend claim. The shared slider UI emits `setGraphParameter`; all three consumers still require their completed retained backend registration and runtime latest-wins integration before end-to-end credit. Flow is owned by the peer executor; Procedural3d and Procedural2d remain this executor's next backend packet. The parent retains the Rust compiler lease and shared Flow source hold for the combined native gate.
