# Puzzle Publication Authority Correction

## Outcome

Puzzle now fails closed at registration rather than claiming a migration and failing later at Store publication. The retained admission surface is limited to completions that emit no Artifact, Config, Draft, Presence, Transient, or Child items:

- Puzzle2d: no admitted route.
- Puzzle3d: `openAddObjectDialog`, `worldPointerDown`.
- Puzzle5d app actions: `canvasPointerDown`, `worldPointerDown`.
- Puzzle5d reserved actions: `copy`.

All other previously retained routes are `BatchOnlyPendingRewrite`. Their dormant cursor implementations remain as bounded evidence, but their IDs are absent from factory keys, proofs, publication contracts, and `Migrated` manifest annotations.

## Store-Lane Authority

No Puzzle owner currently implements the app-owned one-item preparation and bounded root-retirement factories required for Artifact, Config, Draft, Presence, or Transient publication. Therefore no Store-emitting route is registered.

Puzzle5d `cut`, `paste`, and `import-media` emit Artifact mutations and are no longer factory types or registry entries. `build_reserved_tool_job` rejects every non-`copy` route before raw-wire preflight/decode. `copy` remains source-visible as an exact `ToolJobFactory` plus `ArtifactOwnedToolJobFactory` with `EditorApp<Puzzle5dPlayApp>` ownership and a single `HostOnly` contract.

## Exact Emitted-Lane Inventory

The schema-first inventory groups every audited app and reserved route by the lanes its current completion emits. It records exact blockers for each batch-only group. Important corrections to the earlier provisional metadata include:

- Puzzle3d effect/empty completions `engagementRepeatLast`, `fillBuildTick`, `registerBrushMesh`, `suggestionsTick`, `transformBegin`, and `transformEnd` are HostOnly but incomplete semantically, so remain batch-only.
- Puzzle3d `engagementSubmit` emits Config plus host effects, not Artifact.
- Puzzle5d `focusSelection`, `setFillCount`, and `engagementSubmit` emit Config.
- Puzzle5d `registerBrushMesh` and the current no-op `selectSameKind` completion are HostOnly but incomplete semantically, so remain batch-only.

The authoritative files are:

- `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️schema.json`
- `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️component.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`

Existing retained-command fixtures now distinguish `toolIds` (currently admitted generic retained routes) from `evidenceToolIds` (bounded dormant implementations and their lifecycle vectors). The serde_json oracle validates that distinction without erasing progress/cancel/freshness/ACK/checkpoint/incremental-close/terminal evidence.

## Independent and Hostile Validation

The permanent Bun command validates the fixture through Ajv and then runs an independent semantic oracle over the canonical Rust sources. It enforces:

- exact manifest census and disposition for every app route;
- exact retained-ID, proof, publication-contract, and registration bijections;
- source-visible fully qualified factory implementations and exact owner type;
- exclusive HostOnly contracts for admitted routes;
- absence of Store preparation claims for HostOnly factories;
- Puzzle5d reserved authority before raw decode;
- 16 KiB close-page invariant and lifecycle laws.

Hostile cases activate a blocked app route before decode, inject a blocked reserved factory, remove the reserved authority guard, remove a publication contract, duplicate/omit owners, alter the close-page budget, and forge a Migrated group with a blocker. All fail closed.

## Validation

```text
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript
bun './📜️script.ts' publication-authority-audit
```

Exit 0: `validated Puzzle publication authority; admitted=openAddObjectDialog,worldPointerDown,canvasPointerDown,copy,worldPointerDown; schema=Ajv; oracle=independent`.

```text
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript
bun './📜️script.ts' retained-audit
```

Exit 0: `validated 40 CAD routes; admitted=loadRawRequest; schema=Ajv`.

`git diff --check` over the exact Puzzle/CAD files exited 0. No Cargo, Nx, rustfmt, or compiler process was started because the Store agent still owns the exclusive compiler lease.

The retained fixture admission/evidence parse also exited 0: Puzzle2d `0/3/31`, Puzzle3d `2/53/113`, and Puzzle5d `2/43/78` for admitted routes/evidence routes/vectors. Log: `🧪️puzzle-retained-fixture-admission-parse-r1.log`.

## Pending Native Gate

After the compiler lease is explicitly returned: run rustfmt on the exact Rust files, then the focused Puzzle5d, Puzzle3d, and Puzzle2d native gates. The prior Puzzle5d retained gate remains green 8/8; Puzzle3d/Puzzle2d verification is still pending.
