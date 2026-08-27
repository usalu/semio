# CAD 40-Route Source Audit

## Outcome

The canonical `CadPlayApp` owner has 41 command rows. `setActiveUtility` is framework-owned, leaving exactly 40 app-owned routes. Only `loadRawRequest` is admitted as migrated: it performs one bounded dispatch, emits a fixed host file-open effect, declares `HostOnly`, and uses the retained framework command owner for progress, cancellation, freshness, ACK, incremental close, and terminal-empty enforcement.

The other 39 routes are explicitly `BatchOnlyPendingRewrite`. No serialization-heavy route scans and then calls a legacy reducer under a migrated annotation.

## Exact Dispositions

- 1 migrated: `loadRawRequest`.
- 3 persistent serialization blockers: `saveSelected`, `saveInPlay`, `saveCurrent`.
- 1 persistent decode blocker: `importCadFile`.
- 1 document replacement/decode blocker: `setActiveExample`.
- 8 child-publication blockers: object CRUD and selection transforms.
- Artifact/config publication blockers cover the remaining routes because CAD supplies no app-owned one-item preparation factory for those durable lanes.
- `setActiveUtility` remains excluded from the app-owned census and factory proof.

## Contract Details

- Payload schema: `cad.scene.tool-command.v1`.
- Raw wire bytes: 8,192.
- Decoded items: 64.
- Work items: 1.
- Output bytes: 16,384.
- Step budget: 7,500 microseconds.
- Close page: 16,384 bytes on native and Wasm.
- Factory: `CadHostEffectJobFactory`.
- Publication contract: `loadRawRequest -> HostOnly`.
- Proof and manifest each contain exactly the admitted route.
- Wire checkpoints are rejected with exact input/checkpoint owner return because this one-step route has no resumable semantic cursor to restore.

## Schema-First Evidence

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️schema.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️component.json`
- The permanent `retained-audit` command in CAD's canonical `📜️script.ts` validates the fixture with Ajv, independently compares all 40 route IDs and dispositions to the Rust command/manifest source, checks the exact fully qualified factory/owner/registration/proof/publication tokens, and rejects ten hostile source mutations.
- The Rust owner includes an independent `serde_json` parity test against the typed command census, manifest dispositions, 16 KiB close invariant, and retained lifecycle-law list.

## Process-Global Owner Audit

`CadPlayApp` remains a unit struct. No process-global payload or session owner exists in the canonical editor owner. The sole editor `OnceLock` is immutable interaction-catalog metadata, not payload/session state. Gesture freshness continues to derive from persisted operation identity plus generation in `CadConfig`.

## Framework Publication Contract Update

The framework added mandatory exact `PUBLICATION_CONTRACTS` during this packet. CAD now exposes the official verifier's exact source tokens: fully qualified `ToolJobFactory` and `ArtifactOwnedToolJobFactory` implementations, exact `EditorApp<CadPlayApp>` owner, and exact `registry.register(CadHostEffectJobFactory::new(&controller))` registration. Puzzle publication authority was corrected separately: only genuine HostOnly completions remain registered; Store-lane routes and incomplete empty completions are batch-only.

## Validation

No Cargo, Nx, rustfmt, or compiler process was started; the Store agent owns the exclusive compiler lease.

Command:

```text
bun '✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts' retained-audit
```

Final result: exit 0, `validated 40 CAD routes; admitted=loadRawRequest; schema=Ajv`.

Log: `🧪️cad-retained-audit-bun-r5.log`.

Puzzle authority result: exit 0, schema=Ajv and independent source oracle. Log: `🧪️puzzle-publication-authority-bun-r2.log`; full correction report: `📓️sol-puzzle-publication-authority-correction-2026-08-27.md`.

Earlier red validator evidence is retained in `🧪️cad-retained-audit-bun-r1.log` and `r2.log`; `r3.log` is the first green run.

## Pending Compiler Gate

- Run rustfmt only after the compiler lease returns.
- Compile and run focused CAD retained-route tests.
- Re-run Puzzle 5d, 3d, and 2d focused native gates with the new mandatory publication-contract validation.
- Verify the exact lane unions against runtime publications; durable lanes are expected to fail closed until app preparation factories exist.
