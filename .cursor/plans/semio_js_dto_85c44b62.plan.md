---
name: semio js dto
overview: "Refactor `semio/js` so the TypeScript data-transfer surface matches the Rust DTO naming model: remove `*Wire` duplicate types, rename ID/data aliases to `*Dto`, and make DTO types read-only. Update direct TypeScript consumers and verify the JS/React type surfaces."
todos:
 - id: inventory-wire-dto
   content: Inventory all `*Wire`, `*WireDto`, and plain `*Id` exported symbols in `semio/js/index.ts` and confirm the Rust-aligned target names.
   status: completed
 - id: rename-js-dtos
   content: Rename `semio/js` DTO, ID, command, read, and event types to the single Rust-aligned DTO surface and remove duplicate aliases.
   status: completed
 - id: readonly-dtos
   content: Apply recursive read-only typing to all DTO aliases, including Zod-inferred full/shallow/metadata DTOs and command/event payload DTOs.
   status: completed
 - id: update-consumers
   content: Update `semio/react` and any other TypeScript consumers to use the new DTO names without legacy aliases.
   status: completed
 - id: extend-tests
   content: Extend existing embedded tests/type checks for the renamed read-only DTO surface and Rust-shaped command/read/event payloads.
   status: completed
 - id: verify
   content: Run focused JS and React build/test commands, then any affected layering check.
   status: completed
isProject: false
---

# Semio JS DTO Naming Refactor

## Scope

- Continue under the existing open ticket `SEMIO-JS-EXACT-GRAPH-QL-AND-WIRE-TYPING`; it already covers `semio/js` GraphQL/wire typing and has the current plan file `[.cursor/plans/semio-js-types_4a6dd49f.plan.md](.cursor/plans/semio-js-types_4a6dd49f.plan.md)`.
- Primary implementation file: `[semio/js/index.ts](semio/js/index.ts)`.
- Direct consumers to update: `[semio/react/index.tsx](semio/react/index.tsx)` and any `@semio/js` import sites found by search.
- Naming authority: `[semio/rs/lib.rs](semio/rs/lib.rs)`, where IDs and transfer rows are named `KitIdDto`, `DesignFlattenMapEntryDto`, `KitColoredConnectorRowDto`, `IncludedDesignInfoDto`, `KitMetadataDto`, and command/event concepts omit `Wire`.

## Implementation Plan

- Inventory all exported `*Wire`, plain `*Id`, and `*WireDto` symbols in `[semio/js/index.ts](semio/js/index.ts)`, then group them into: true DTOs, command/event DTOs, JSON tree helper types, and internal-only helpers.
- Replace duplicate ID aliases with Rust-style DTO names: `KitIdWire` and `KitId` become `KitIdDto`; likewise `TypeId` to `TypeIdDto`, `DesignId` to `DesignIdDto`, `PieceId` to `PieceIdDto`, etc. Keep one exported type per concept and update static helpers such as `Kit.createId` to return the DTO name.
- Rename wire DTO rows to Rust-aligned names: `DesignFlattenMapEntryWireDto` to `DesignFlattenMapEntryDto`, `KitColoredConnectorRowWireDto` to `KitColoredConnectorRowDto`, `DesignIncludedDesignWireDto` to `IncludedDesignInfoDto`, and `KitCatalogKitMetadataWireDto` to `KitMetadataDto` unless an existing schema DTO already owns that exact name.
- Rename command/event data types by dropping `Wire` where they match Rust concepts: `ChangeKitCommandWire` to `ChangeKitCommand`, nested `Change*CommandWire` similarly, `KitChangeWire` to `KitChange`, `ConflictBatchRecordWire` to `ConflictBatchRecord`, and `*KitEventWire` to `*KitEvent`. Preserve `KitEvent` itself if it already represents the public event union.
- Make DTOs read-only at the type level. Add or reuse a local recursive readonly helper in `[semio/js/index.ts](semio/js/index.ts)` and apply it to Zod-inferred DTO aliases (`*IdDto`, `*MetadataDto`, `*Shallow`, `*FullDto`, read rows, command payload DTOs) so nested arrays and object fields are readonly.
- Keep JSON transport helper names only where they are truly transport-shaped. If `SemioKitWireTreeDto` is only a generic serde/GraphQL JSON tree, rename it to a neutral DTO name or keep it as an internal helper, but do not leave public duplicate `Wire` data aliases.
- Update all function names, comments, and parser helper names that expose old names, for example `piecePatchToWireCommands` and `semioParseKitIdWireArrayWire`, so exported API names no longer advertise `Wire` duplicates.
- Update direct consumers in `[semio/react/index.tsx](semio/react/index.tsx)` to import/re-export the new names, including `ChangeKitCommand` and renamed read batch aliases if those are retained.
- Extend existing embedded tests in `[semio/js/index.ts](semio/js/index.ts)` to assert the public DTO surface is read-only at compile time and that command/read/event round-trips still accept the Rust/GraphQL camelCase payload shape. Update existing React tests only if public imports change.
- Verify with `npm run build` and `npm run test` in `[semio/js](semio/js)`, then `npm run build` and `npm run test` in `[semio/react](semio/react)`. If the rename affects package layering, run the root dependency-cruiser layering check too.

## Key Current Targets

Current duplicate/legacy names in `[semio/js/index.ts](semio/js/index.ts)` include:

```ts
export type KitBackboneConfigWire = { readonly Memory: null } | { readonly Dev: { readonly path: string } } | { readonly Local: { readonly folder: string } } | { readonly Remote: { readonly url: string; readonly sessionId: string } };

export type BackboneConfig = KitBackboneConfigWire;
export type KitCheckpointWire = SemioKitWireStructDto;
export type KitIdWire = { readonly id: string };
```

And the command/event region currently exposes the main `Wire` duplication:

```ts
export type ChangePieceCommandWire = { readonly name: { readonly name?: string | null } } | { readonly description: { readonly description?: string | null } } | { readonly plane: { readonly plane?: SemioKitWireTreeDto | null } };

export type ChangeKitCommandWire = { readonly name: { readonly name: string } } | { readonly description: { readonly description?: string | null } };
```

These become the single DTO/command names used everywhere, with no compatibility aliases.
