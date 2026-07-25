---
name: factories to commands
overview: Generalize `FactorySpec` to a `CommandSpec` with declarative selection inputs (raw + analytic topology kinds), per-state `selection.accept` filters, and `CommandResponse { ok, errors, warnings, infos, diff, data }` outputs (compose-style nullable `TopologyDiff`), letting read-only commands like Distance/Area participate in the same pipeline.
todos:
 - id: rename
   content: Rename `factory` → `command` across core/stately/r3f/play + fixtures + schema string + preset keys
   status: completed
 - id: diff
   content: "Add `🧮Diff` region: `EntityDiff`, `*RecordDiff`, `TopologyDiff`, `applyTopologyDiff`, `invertTopologyDiff`, `isEmptyTopologyDiff`"
   status: completed
 - id: response
   content: "Add `📨Response` region: `CommandMessage`, `CommandResponse`, `EMPTY_COMMAND_RESPONSE`"
   status: completed
 - id: selection-spec
   content: "Add `🪪Selection` region + per-state `selection: SelectionSpec` in `CommandSpec.machine.states[*]`; expose `listActiveSelectionAccept`"
   status: completed
 - id: kernel-diff
   content: Extend `KernelAdapter` with `*Diff` writes + `vertexDistance`/`edgeLength`/`faceArea`/`cellVolume`; implement in `BrepjsKernel`
   status: completed
 - id: commit-rewrite
   content: Rewrite `CommandRuntime.commit` to produce `CommandResponse`, apply diff to topology, record `DocumentCommand` with inverse diff; gate `selection.changed` by `selection.accept`
   status: completed
 - id: fixtures
   content: Rename 3 fixtures to `*.command.json`, add `distance.command.json` + `area.command.json`, update `listSpatialCommandPresets` keys q/j/k/d/a
   status: completed
 - id: play-app
   content: Update `play/main.tsx` to consume `CommandResponse` (display `data` for Distance/Area, apply `diff` for write commands), rename symbols
   status: completed
 - id: tests
   content: Extend existing test regions in core/stately/kernel-brepjs to cover diff round-trip, selection filter, distance/area, parity
   status: completed
 - id: verify
   content: "`bun nx run-many -t test -p @spatial/js-core @spatial/js-machine-stately @spatial/js-kernel-brepjs` + manual play-app smoke with `[DEBUG]` logs"
   status: completed
isProject: false
---

## Concept

A `Command` = the previous `Factory` + first-class **selection inputs** + first-class **outputs**. Every command transitions through its state machine and on commit produces a `CommandResponse`:

```
CommandResponse = { ok, errors[], warnings[], infos[], diff: TopologyDiff, data: unknown }
TopologyDiff    = { vertices, edges, wires, faces, shells, cells, cellComplexes, clusters }
                  each = { added: Record<id, Record>, modified: Record<id, RecordDiff>, removed: string[] }
RecordDiff      = partial nullable shadow (compose rule: nullable field present = update; `removeX: true` to clear by-default-nullable field)
```

Read-only commands (Distance, Area, Volume) emit an empty `diff` and populate `data` with the numeric/structured result. Write commands emit a non-empty `diff`; the runtime applies that diff to `ModelDocument.topology` and records a `DocumentCommand` whose `undo` is the inverse diff.

Selection is declared per state via `selection: { accept: TopologyEntityKind[], multiple, prompt? }`. The host filters picks against `accept` (mixing raw `vertex|edge|wire|face|shell|cell|cellComplex|cluster` and analytic `surface|part`) and only fires `selection.changed` for matches; the spec remains portable.

## Naming refactor (`factory` -> `command`, full rename)

Everywhere in `spatial/`:

- Schema string `spatial.factory/v1` -> `spatial.command/v1`.
- Fixture filenames: [`spatial/fixture/factory.json`](spatial/fixture/factory.json), [`extrude.factory.json`](spatial/fixture/extrude.factory.json), [`offset-surface.factory.json`](spatial/fixture/offset-surface.factory.json) -> `box.command.json`, `extrude-wire.command.json`, `offset-surface.command.json`. Add two new fixtures `distance.command.json` and `area.command.json` (read-only).
- Types/classes in [`spatial/js/core/index.ts`](spatial/js/core/index.ts): `FactorySpec`->`CommandSpec`, `FactoryRuntime`->`CommandRuntime`, `FactoryRuntimeOptions`->`CommandRuntimeOptions`, `FactorySnapshot`->`CommandSnapshot`, `FactoryEvent`->`CommandEvent`, `FactorySpatialInteractionConfig/Resolved`->`Command...`, `FactoryKeybindRow`->`CommandKeybindRow`, `SpatialFactoryPreset`->`SpatialCommandPreset`. Replace `buildBoxFactorySpec` / `buildExtrudeFactorySpec` / `buildOffsetSurfaceFactorySpec` with `buildBoxCommandSpec` etc. plus new `buildDistanceCommandSpec`, `buildAreaCommandSpec`, `listSpatialCommandPresets`, `loadSpatialCommandPreset`, `resolveSpatialCommandPresetKey`, `createCommandRuntime`. The XState helper kind `factoryKind` -> `commandKind`.
- Region renames: `🏭Factory` -> `📜Command`, `📦Factories` -> `📦Commands`. `🎬Statechart` keeps name.
- Workspace package names stay (`@spatial/js-core`, `@spatial/js-machine-stately`, `@spatial/js-kernel-brepjs`, `@spatial/js-renderer-r3f`) — the user said "spatial:" not "rename packages", and a package rename adds noise without value.

## 1. Core: `Selection`, `Diff`, `Response` in [`spatial/js/core/index.ts`](spatial/js/core/index.ts)

Add a new `🪪Selection` region (before `🗺️Expr`):

```ts
export interface SelectionTarget {
 readonly kind: TopologyEntityKind;
 readonly id: string;
 readonly editable: boolean;
 readonly derivedFrom?: readonly { kind: EditableEntityKind; id: string }[];
}
export interface SelectionEvent extends CommandEvent {
 readonly kind: "selection.changed";
 readonly targets: readonly SelectionTarget[];
}
export interface SelectionSpec {
 readonly accept: readonly TopologyEntityKind[];
 readonly multiple?: boolean;
 readonly prompt?: string;
}
export function filterSelectionTargets(spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[];
export function selectionEventMatches(spec: SelectionSpec, ev: SelectionEvent): boolean;
```

Add a new `🧮Diff` region (after `🧱Topology`):

```ts
export type VertexRecordDiff = Partial<Omit<VertexRecord, "id">> & { id: VertexRef };
export type EdgeRecordDiff = Partial<Omit<EdgeRecord, "id">> & { id: EdgeRef };
// + wire / face / shell / cell / cellComplex / cluster variants

export interface EntityDiff<TAdded, TMod, TId extends string> {
 readonly added?: Record<TId, TAdded>;
 readonly modified?: Record<TId, TMod>;
 readonly removed?: readonly TId[];
}
export interface TopologyDiff {
 readonly vertices?: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef>;
 readonly edges?: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef>;
 readonly wires?: EntityDiff<WireRecord, WireRecordDiff, WireRef>;
 readonly faces?: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef>;
 readonly shells?: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef>;
 readonly cells?: EntityDiff<CellRecord, CellRecordDiff, CellRef>;
 readonly cellComplexes?: EntityDiff<CellComplexRecord, CellComplexRecordDiff, CellComplexRef>;
 readonly clusters?: EntityDiff<ClusterRecord, ClusterRecordDiff, ClusterRef>;
}

export const EMPTY_TOPOLOGY_DIFF: TopologyDiff;
export function applyTopologyDiff(topo: TopologyGraph, diff: TopologyDiff): TopologyDiff; // returns inverse
export function invertTopologyDiff(forward: TopologyDiff, before: TopologyGraph): TopologyDiff;
export function isEmptyTopologyDiff(d: TopologyDiff): boolean;
```

Add a new `📨Response` region (before `📜Command`):

```ts
export interface CommandMessage {
 readonly code: string;
 readonly message: string;
 readonly path?: string;
}
export interface CommandResponse<TData = unknown> {
 readonly ok: boolean;
 readonly errors: readonly CommandMessage[];
 readonly warnings: readonly CommandMessage[];
 readonly infos: readonly CommandMessage[];
 readonly diff: TopologyDiff;
 readonly data: TData | null;
}
export const EMPTY_COMMAND_RESPONSE: CommandResponse<null>;
```

## 2. Spec: per-state `selection` + extended `commit.operation`

In `CommandSpec`:

- `machine.states[*].selection?: SelectionSpec` — host-side filter. Runtime exposes via `getActiveSelectionSpec(spec, state)` and `listActiveSelectionAccept()`; renderer/host uses it to gate pointer picks.
- `commit.operation.kind` extended with the new read/write operations below; `commit.outputDataPath?: string` (optional context path whose value is copied into `response.data` for read-only commands).

`parseFactorySpec` -> `parseCommandSpec` validates the new fields, returns `null` on mismatch as today.

## 3. Kernel: diff-producing operations

Extend `KernelAdapter` in [`spatial/js/core/index.ts`](spatial/js/core/index.ts):

```ts
export interface KernelAdapter {
 // existing methods kept
 createBoxFromCornersDiff?(input): Promise<{ diff: TopologyDiff; cell: CellRef }>;
 extrudeWireDiff?(input): Promise<{ diff: TopologyDiff; cell: CellRef }>;
 offsetFacesDiff?(input): Promise<{ diff: TopologyDiff }>;
 vertexDistance?(a: VertexRef, b: VertexRef): Promise<number>;
 edgeLength?(e: EdgeRef): Promise<number>;
 faceArea?(f: FaceRef): Promise<number>;
 cellVolume?(c: CellRef): Promise<number>;
}
```

`BrepjsKernel` in [`spatial/js/kernel-brepjs/index.ts`](spatial/js/kernel-brepjs/index.ts) implements the `*Diff` variants by computing the same mesh face it currently writes in `appendCommittedMeshFaceToTopology` and returning it as a `TopologyDiff.faces.added` (+ supporting vertex/edge/wire entries). `cellVolume` uses existing `measureVolume`; `vertexDistance` reads graph + `vec3Distance`; `faceArea` sums planar/mesh triangle areas; `edgeLength` sums curve segments.

`appendCommittedMeshFaceToTopology` is refactored to `meshFaceDiff(mesh, idTag): TopologyDiff` + a one-line wrapper that calls `applyTopologyDiff` — single source of truth for "kernel mesh -> diff".

## 4. Command runtime: diff-driven commit + selection filter

Rewrite `commit()` in `CommandRuntime` ([`spatial/js/core/index.ts`](spatial/js/core/index.ts), `🏭Factory` region):

```ts
async commit(): Promise<CommandResponse> {
  if (!this.canCommit()) return { ok: false, errors: [...], warnings: [], infos: [], diff: EMPTY_TOPOLOGY_DIFF, data: null };
  const ctx = this.sm.getContext();
  const operation  = this.spec.commit.operation;
  const params = resolveTemplate(operation.params, { context: ctx }) as Record<string, unknown>;

  let diff: TopologyDiff = EMPTY_TOPOLOGY_DIFF;
  let data: unknown = null;
  switch (operation.kind) {
    case "cell.createBox":      ({ diff } = await kernel.createBoxFromCornersDiff!(params as any)); break;
    case "wire.extrudeToCell":  ({ diff } = await kernel.extrudeWireDiff!(params as any)); break;
    case "face.offset":         ({ diff } = await kernel.offsetFacesDiff!(params as any)); break;
    case "measure.distance":    data = await kernel.vertexDistance!(params.a as VertexRef, params.b as VertexRef); break;
    case "measure.area":        data = await kernel.faceArea!(params.faceId as FaceRef); break;
    case "measure.volume":      data = await kernel.cellVolume!(params.cellId as CellRef); break;
  }
  if (this.spec.commit.outputDataPath) data = getPath(ctx, this.spec.commit.outputDataPath) ?? data;

  const inverse = applyTopologyDiff(this.opts.document.topology, diff);
  if (!isEmptyTopologyDiff(diff)) this.recordDiffCommand(diff, inverse);

  await this.sm.send({ kind: "confirm" }, this.opts.kernel);
  const response: CommandResponse = { ok: true, errors: [], warnings: [], infos: [], diff, data };
  this.lastResponse = response;
  this.emit();
  return response;
}
```

`CommandSnapshot` gains `readonly lastResponse: CommandResponse | null`. The legacy `commit(): Promise<CellRef|null>` return path is removed (greenfield rule — no backward compat); the play app reads `response.diff` and `response.data`.

`send(event)` filters `selection.changed` against the active state's `selection.accept` before invoking `applyTransition`; mismatched targets are dropped with an `info`-level entry added to the next snapshot. `getActiveSelectionSpec()` and `listActiveSelectionAccept()` are exposed for hosts/renderers.

## 5. New + updated fixtures

- [`spatial/fixture/box.command.json`](spatial/fixture/factory.json) (renamed): `schema: "spatial.command/v1"`, no `selection` block (pointer-driven).
- [`spatial/fixture/extrude-wire.command.json`](spatial/fixture/extrude.factory.json) (renamed): `machine.states.selectWire.selection = { accept: ["wire"], multiple: false, prompt: "Pick wire" }`.
- [`spatial/fixture/offset-surface.command.json`](spatial/fixture/offset-surface.factory.json) (renamed): `machine.states.selectSurface.selection = { accept: ["surface", "face"], multiple: false, prompt: "Pick surface" }` (raw `face` allowed as a fallback for kernels without derived view).
- New `spatial/fixture/distance.command.json` — read-only. Two selection states each `accept: ["vertex"]`; commit `kind: "measure.distance"`, `outputDataPath: "distance"`.
- New `spatial/fixture/area.command.json` — read-only. One selection state `accept: ["face", "surface"]`; commit `kind: "measure.area"`, `outputDataPath: "area"`.

`listSpatialCommandPresets()` returns the 5 entries with keys `q,j,k,d,a`.

## 6. Stately + renderer wiring

- [`spatial/js/machine-stately/index.ts`](spatial/js/machine-stately/index.ts): rename `FactoryEvent`->`CommandEvent`, `factoryKind`->`commandKind`, `factoryState`->`commandState`, `factoryContext`->`commandContext`, update import list, update test descriptions. Behaviour unchanged.
- [`spatial/js/renderer-r3f/play/main.tsx`](spatial/js/renderer-r3f/play/main.tsx): rename all `factory*` identifiers to `command*`, update imports, consume `CommandResponse` from `await rt.commit()` (display `response.data` for Distance/Area in the debug overlay, log `response.diff` for write commands), gate pointer-pick events through `rt.listActiveSelectionAccept()` so analytic vs raw kinds are filtered host-side, and add presets `distance` / `area` to the catalog.
- [`spatial/js/renderer-r3f/index.tsx`](spatial/js/renderer-r3f/index.tsx) (symbol rename): `useFactorySnapshot`->`useCommandSnapshot`, `FactoryCanvas`->`CommandCanvas`, `FactorySpatialView`->`CommandSpatialView`.

## 7. Tests (extend existing test regions only)

Per workspace rules: no new test files. Extend the `🧪Tests` region in each of the three TS files.

- `core`: cases for `applyTopologyDiff` round-trip (apply then apply-inverse restores original), `selectionEventMatches` filter behaviour, distance command commit (`response.data === 5`), area command commit (`isEmptyTopologyDiff(response.diff) === true`), box command commit (`response.diff.faces.added` non-empty), per-state `selection.accept` filter rejection (event dropped, no transition).
- `machine-stately`: extend the existing parity tests to also drive distance + area commands and assert both providers return equal `CommandResponse.data` and equal `CommandResponse.diff` shapes.
- `kernel-brepjs`: tests for `createBoxFromCornersDiff` (one face in diff), `cellVolume`, `faceArea`, `vertexDistance`.

Run `bun nx run-many -t test -p @spatial/js-core @spatial/js-machine-stately @spatial/js-kernel-brepjs`, then load the play app and confirm a Distance command produces `data: <number>` and an Offset command produces a non-empty `diff` via `[DEBUG]` logs.

## What stays unchanged

- Pure-TS and XState backend semantics (only renames + dropping selection events filtered out before `applyTransition`).
- `TopologyGraph` shape, `DerivedViewService`, expression evaluator (`evalExpr`), display resolver.
- `DocumentHistory` API (still records a `DocumentCommand` per commit; the command's `do/undo` now apply forward/inverse `TopologyDiff`).

## Out of scope

- Real Surface/Part merging beyond the existing 1:1 `DerivedViewService` — commands rely on the existing derivation; refinement is a follow-up ticket.
- Renaming workspace packages (`@spatial/js-*`) — only type and fixture identifiers change.
- Persisting `CommandResponse` history across sessions.
