# Demonstrator Playground Command Migration

## Scope

The Demonstrator playground editor owns one persistent action, `changeSchema`. This packet replaces its declaration-only bounded proof with a concrete app-owned retained-command factory and builder.

## Runtime Shape

- The exact public route is `s.demonstrator.playground@1/*#editor` + `changeSchema` + `playground.playground.tool-command.v1`.
- `PlaygroundCommandJobFactory` is registered by the owning `ArtifactEditor` and builds `ArtifactRetainedCommandJob<EditorApp<PlaygroundEditor>>`.
- The operation context retains app instance, parent document, operation/generation, and canonical base revision authority.
- Work extent is exactly one admitted item. Schema tags above 8,192 bytes fail before the job is built and again during work preflight.
- Wire admission returns oversized or checkpoint owners on rejection; this bounded route does not accept a resumable checkpoint.
- The existing language-neutral `change-schema` JSON quintet remains the mutation/output oracle. A new language-neutral limits fixture pins exact/max+1 admission and is parsed independently with `serde_json` in the Rust law and Bun in the coordinator check.

## Evidence

- `rustfmt --edition 2021 <playground editor component>`: exit 0.
- Bun parse and value check of `🧪️fixtures/🎯️retained-command-limits.json`: exit 0.
- `git diff --check` over the Demonstrator packet: exit 0.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json --output <official report>`: expected workspace exit 1 because unrelated global/scan/import/remaining ledgers are still open; `remainingCommands` is 733 and contains zero Demonstrator rows. No forged-factory failure remains.

## Pending Runtime Gate

The focused Demonstrator Rust test has not yet run because the workspace uses one serialized compiler lease and Puzzle currently owns it. Static closure is not treated as runtime completion.
