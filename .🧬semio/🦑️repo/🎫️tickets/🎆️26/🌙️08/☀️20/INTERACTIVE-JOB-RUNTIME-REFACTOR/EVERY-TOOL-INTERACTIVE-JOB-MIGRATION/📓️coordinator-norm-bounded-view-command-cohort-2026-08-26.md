# Norm Bounded View Command Cohort

## Scope

The fifteen Norm editor apps now route evaluate and setSelectedCheckIndex through concrete app-owned retained command factories. setSnapshot remains fail closed because DSL parsing and semantic mutation decomposition are not bounded first-step work.

## Contract

- Exact factory owner: each concrete EditorApp play app.
- Exact controller: s.norm.variant@1/*#editor.
- Exact payload schema: document-schema.tool-command.v1.
- Raw wire: 256 bytes.
- Decoded items: 8.
- Work per step: 1.
- Output: 1,024 bytes.
- Step ceiling: 7,500 microseconds.
- Checkpoints are rejected because both admitted reducers finish in one bounded work step.
- evaluate emits no mutation; setSelectedCheckIndex emits one config mutation and cannot mutate the artifact.

## Evidence

- Language-neutral fixture: ✏️s/🔌️plugins/📕️norm/🧪️fixtures/🎯️bounded-view-command-limits.json.
- Third-party Ajv result: 🧪️coordinator-norm-retained-fixture-ajv-2026-08-26.txt.
- Rust cohort fixture law is hosted by the EN 1990 editor and checks the shared limits, empty/single/max/max+1/checkpoint cases, exact payload schema, and exact two-key factory catalog.
- Rustfmt completed for all fifteen editor components.
- Scoped git diff --check completed without diagnostics.
- Official static verifier admitted all 30 Norm view-command rows. The workspace ledger moved from 217 admitted / 719 remaining to 254 admitted / 682 remaining while Mathematical's concurrent seven-row cohort was also present.

## Runtime Status

The Norm crate runtime test is queued behind the serialized Lowpoly and Puzzle compiler leases. No runtime or timing pass is claimed in this checkpoint.
