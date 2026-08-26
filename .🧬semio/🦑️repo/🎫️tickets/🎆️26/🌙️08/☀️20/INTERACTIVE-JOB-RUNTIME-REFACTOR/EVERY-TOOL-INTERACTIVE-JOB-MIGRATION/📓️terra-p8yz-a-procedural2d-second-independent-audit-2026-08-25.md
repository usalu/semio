# P8yz-a Procedural2d Second Independent Source/Static Audit

Date: 2026-08-25  
Auditor: Terra (independent re-audit)  
Scope: current retained mounted Procedural2d source plus the explicitly authorized canonical-pack prerequisite. No production source was changed. No Cargo, Nx, Wasm, browser, or build command was run.

## Verdict: GREEN at the requested source/static boundary

No source/static blocking defect was found. The first audit's non-empty-snapshot defect is remediated in the current source: the retained fixture creates `retained-synapse` from `retained-neuron` port `out` to `retained-preview`, then proves census, exact row, field-delimited digest, full typed-snapshot equality, and terminal-empty close.

This is not a native/Wasm/browser/timing acceptance claim; those matrices were deliberately not executed under the packet constraint.

## Independent Evidence

- Canonical ingress is `P2D2` at `snapshot/.../binary/component.rs:89`; `admit_byte` compares each prefix byte before `allocate_after_discriminator` can create source/anchor/segment/catalog/value/typed owners (`1080-1095`, `1055-1076`). The hostile law passes `P3D3`'s second byte and asserts no semantic owner (`1358-1366`).
- The mounted session owns all five retained layers—source, anchor, segment, catalog, and value (`1009-1015`)—and drives them through one-byte/one-grant handback (`1131-1187`). It exposes progress (`1199-1201`), cancellation (`1213-1218`), bounded incremental close (`1220-1271`), and a terminal-empty Drop invariant (`1274-1282`).
- Shared `RetainedValueCursor` explicitly handles all tags `0x00..0x17`, including TableSoA, DSL value, packed forms, Wire, and Expr (`os pack/value/component.rs:983-1022`). Its executable law builds all 24 tags and proves terminal empty (`2700-2752`).
- The two Procedural2d ledgers parsed with exact assertions: shared value tags `24`, mounted lifecycle entries `12`, mutation variants `14`, and nested owner laws `11`. They include `clear-widget-layout`, TableSoA, recursive neural values, cluster tree/flow, and nested generation JSON.
- The typed snapshot owner materializes synapse rows without a generic record value (`snapshot binary:739-746`); the corrected law appends the semantically valid retained synapse (`1319-1325`) and verifies exact census/row/digest/full snapshot (`1351-1355`).
- The mutation owner lists fourteen variants including the 2D-only `ClearWidgetLayout` (`mutations binary:506-532`), and the all-14 retained structural-grant law is present at `3422-3450`.
- Store initialization is an explicit retained state machine: copy initial, seed history, replay applied forwards, hash inverses, restore checkpoint, and rebuild redo (`2827-2847`, `3074-3239`). Direct per-field mutation replay is at `583-673`; no generic whole-state apply/diff/clone path is used by that retained initializer.
- The real non-empty `VcsArtifactApp` law drives maintenance, verifies all-field/all-14 result and digest, explicitly ACKs, and covers Missing/WrongOperation/WrongGeneration/WrongBase/WrongParent with last-valid visibility (`editor/component.rs:689-730`). The Wasm production bridge preflights before producer construction/copy, exposes begin/admit/seal/poll/take/resume/retry/ACK/cancel/close, and checks publication authority immediately before maintenance (`wasm/component.rs:414-554`).
- The production-region reachability law splits before `MountedLaws` and rejects whole-buffer edges only in the production bridge (`wasm/component.rs:711-756`). The only Procedural2d batch `decode_pack` helper is `#[cfg(test)]` (`snapshot binary:18-23`); the mounted production regions have no `decode_pack`, `decode_document`, or `RecordValue` edge.
- The exact Rust-only global raw caller census is `10`: one shared fail-closed definition and nine remaining peer callers. The count intentionally scopes to `*.rs`, excluding root-script test strings.

## Commands Run

All passed with exit status 0:

```text
rustfmt --edition 2021 --check \
  framework/os/pack/value/component.rs \
  framework/pack/format/component.rs \
  framework/os/store/component.rs \
  procedural2d/snapshot/binary/component.rs \
  procedural2d/mutations/binary/component.rs \
  procedural2d/editor/wasm/component.rs \
  procedural2d/editor/component.rs
git diff --check -- <the same authorized shared/P2 paths>
bun -e <standalone JSON parser/assertions for the three retained law ledgers>
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | wc -l
```

The standalone Bun check reported:

```json
{"valueTags":24,"lifecycle":12,"mutations":14,"nestedOwners":11}
```

The raw census command reported `10`. A scoped `git diff --name-only` inspection found no uncommitted source path in the authorized shared/P2 scope before this audit report was added; this audit created only this ticket-local Markdown file.

## Blocking Defects

None found at source/static scope.

