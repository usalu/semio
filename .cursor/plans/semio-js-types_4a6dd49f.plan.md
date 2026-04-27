---
name: semio-js-types
overview: Refactor `semio/js` so public and internal TypeScript surfaces use exact domain, wire, and GraphQL operation types with no remaining `Record<>` or `unknown`, extending `semio/rs` and regenerated `semio/graphql` where the current schema forces opaque typing.
todos:
 - id: inventory
   content: Inventory all `Record<` and `unknown` occurrences in `semio/js` and classify each by exact replacement type.
   status: completed
 - id: rust-graphql
   content: Replace opaque Rust GraphQL scalar boundaries with concrete GraphQL object/input/union types where needed, then regenerate `semio/graphql/schema.graphql`.
   status: in_progress
 - id: js-wire
   content: Refactor `semio/js/index.ts` GraphQL, event, patch, and store APIs to exact types with no forbidden tokens.
   status: pending
 - id: tests
   content: Extend existing Rust and JS tests to cover typed command/event/read behavior and run relevant checks.
   status: pending
isProject: false
---

# Semio JS Exact Types Refactor

## Scope

- Continue under the existing active Semio type/control-plane work rather than creating an unrelated track, most closely aligned with `Refactor Semio TypeScript Object Model` and `Rust Control Plane Refactor`.
- Primary files: [semio/js/index.ts](semio/js/index.ts), [semio/rs/lib.rs](semio/rs/lib.rs), [semio/graphql/schema.graphql](semio/graphql/schema.graphql), [semio/graphql/project.json](semio/graphql/project.json).
- Success condition: `semio/js` has zero `Record<` and zero `unknown` tokens, with exact replacements rather than `any` or compatibility shims.

## Implementation Plan

- First inventory all loose TS sites with a focused search and keep a local checklist: patch mappers, GraphQL helpers, event normalization, `KitStore` execute/read results, scoped transaction payload builders, and tests embedded in [semio/js/index.ts](semio/js/index.ts).
- Replace structural bags with named exact aliases. For example, `SemioKitWireStructDto` already uses an index signature and can replace many object-bag cases; patch functions should accept dedicated patch types or `Partial<PieceDiff>` / `Partial<ConnectionDiff>`-style exact shapes instead of open keys.
- Replace `unknown` boundaries with explicit JSON and result types: define `JsonPrimitive`, `JsonArray`, `JsonObject`, `JsonValue` if needed, then narrow through typed parsers and zod schemas so GraphQL/event inputs are not untyped.
- Tighten GraphQL calls in `KitStore`: make `kitGraphqlRun`, `kitGraphqlData`, read-scope variables, mutation variables, batch results, event stream payloads, and metadata rows generic or operation-specific, with exact result and variable interfaces.
- Fix the source of unavoidable looseness in [semio/rs/lib.rs](semio/rs/lib.rs): replace opaque GraphQL scalars such as `ChangeKitCommand`, `KitEvent`, `KitFullSnapshot`, `TypeShallowList`, and `DesignShallowList` where practical with concrete async-graphql objects, enums, unions, and input objects. Keep serde wire types as the data authority, but expose their fields through GraphQL instead of serializing them as arbitrary values.
- Regenerate [semio/graphql/schema.graphql](semio/graphql/schema.graphql) through the existing build path in [semio/graphql/project.json](semio/graphql/project.json), then update `semio/js` against the richer SDL.
- Extend existing tests only: add compile-time and runtime assertions inside current JS/Rust test sections for typed GraphQL events, typed batch command inputs, patch conversion, and metadata row reads. Do not add new standalone test files.
- Verify with targeted checks first, then broader package checks: `npx nx build semio/graphql`, Rust tests around `kit_graphql`, `semio/js` typecheck/tests, and any affected React/sketchpad tests if public APIs shift.

## Key Refactor Targets

```813:836:semio/js/index.ts
/** @emoji 🧾 Converts a piece field patch into nested `changePieceCommands` wire entries. */
export function piecePatchToWireCommands(patch: Record<string, unknown>): ChangePieceCommandWire[] {
  const out: ChangePieceCommandWire[] = [];
```

Replace this family with exact patch types and typed coercion helpers.

```1183:1287:semio/js/index.ts
function kitGraphqlData(response: unknown): Record<string, unknown> {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
```

Replace this with generic, operation-specific GraphQL response parsing.

```27300:27333:semio/rs/lib.rs
// #subregion GqlControlPlaneScalars
/// 🧾 `ChangeKitCommand` wire (externally tagged JSON; GraphQL name is not `JSON`).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GqlChangeKitCommand(pub ChangeKitCommand);
```

Replace opaque scalar wrappers with concrete GraphQL types wherever this is what forces TypeScript looseness.
