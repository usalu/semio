# 📋️ Fan-out brief — one plugin crate per agent

You are one of 31 concurrent wave-5 agents. Every agent works from this brief plus the normative
spec. Your own plugin folder is the only production tree you may touch.

## Read, in this order

1. `📜️normative-spec.md` — the binding contract. §2 layout, §3 formats, §4 naming, §5 state classes,
   §6 casing + scalar/cardinality mapping tables, §7 facet contents, §10 the artifact table with YOUR
   artifact's key / type prefix / current snapshot type, §13 glue wiring.
2. `📜️normative-spec.md` §15 — **the finished lowpoly pilot, all fifteen leaves verbatim, plus its
   `LowpolyDiff` runtime and glue diff.** This is your template. Diff against it; do not improvise a
   different shape.
3. `🧪wave4-report.md` — the pilot's own notes, including its final field inventory.
4. `🧪wave2-report.md` — the policy scanners that will judge your leaves, and exactly how each of the
   five field extractors parses your files.
5. `🧪fixup-document-app-snapshot-report.md` — the renamed plugin-SDK API you must call.
6. `/Users/ueli/Documents/semio/AGENTS.md`.

## Hard rules

- NEVER run a mutating git command (no `commit`, `stash`, `checkout`, `worktree`, `restore`). Others
  are working in this repo simultaneously. Use plain `mv` for file moves.
- Edit existing files in place. Never create a parallel "fixed" copy of a broken file.
- Structure code with `//#region 🔖️Name` / `//#endregion` matching surrounding style.
- Every docstring starts with a unique fitting emoji. No comments inside definitions. Concise code.
- Greenfield: no aliases, no `pub use Old as New`, no back-compat, no deprecations, no migration
  helpers. Rename outright.
- Do not create additional test files — extend the existing ones. Do not create additional example
  files.
- Temporary probes and logs go in the ticket folder, prefixed `🧪`.
- Stay inside your assigned plugin folder plus the ticket folder. If a shared framework surface blocks
  you, do NOT fix it — record it in your report as a fixup item and work around nothing.

## What to deliver, per artifact you own

### 1. Fifteen handcrafted leaves

```
🗿️artifacts/<artifact>/🧬️schema/{🦀️component.rs, 🟦️component.ts, 🔗️component.graphql, 🔣️component.json, 🛰️component.proto}
🗿️artifacts/<artifact>/📸️snapshot/🧬️schema/{same five}
🗿️artifacts/<artifact>/🔺️diff/🧬️schema/{same five}
```

Types `XArtifact` / `XSnapshot` / `XDiff` with the prefix from §10. `XSnapshot` **replaces** the
current snapshot type named in §10 — rename it out of the crate entirely, do not alias it.

The `🔣️component.json` JSON Schema leaf is normative; the other four mirror it exactly. All five must
agree on field set, optionality and cardinality per the §6 tables — the policy extractors compare them
literally.

### 2. Honest state classification

The artifact schema is the union of the persisted snapshot, the app's `DocumentApp::Config`, its
`DocumentApp::Draft`, and engine-derived values. Read your plugin's `🎛️apps/<app>/🦀️component.rs`,
its `🧮️config`/`🎚️config` component and its session/view components to inventory what is really there.
Classify from what the code actually does. Do not invent fields; do not omit fields that exist.

The snapshot facet must contain **exactly** the `persistent` fields of the artifact facet — equality,
not a rough subset.

### 3. `🎒️pack` relocation

Move `🗿️artifacts/<artifact>/🎒️pack/` to `🗿️artifacts/<artifact>/📸️snapshot/🎒️pack/` with all its
files. Contents change only where a type was renamed. The pack's
`📡️component.protocol.semio` envelope segment becomes `Snapshot`, not `Projection`.

### 4. `XDiff` as a sparse field delta

Per §7.3: one optional entry per non-`effect` artifact field plus an `artifact:` whole-replacement
entry that wins over everything. `Identified` collections become an
`added`/`removed`/`patched`/`reordered` delta reusing the artifact's existing patch type. Implement
`MutationDiff<XSnapshot>` over the `persistent` entries and `apply_to_artifact` over all of them.
`absorb` merges field-wise. Every existing mutation under `🧬️mutations/` must construct this delta
from its arguments instead of wrapping itself in a mutation list, and its `↩️inverse` must still
round-trip.

### 5. Engine

`⚙️engine/🦀️component.rs` implements `ArtifactEngine`, which now requires `type Artifact` and
`fn artifact(&self) -> &Self::Artifact`. The engine must own a real `XArtifact` and return only its
persisted subset from `snapshot()`. **Never write `type Artifact = XSnapshot`** to satisfy the
compiler — that defeats the entire point of the associated type.

### 6. Glue + registry

In `📦️packages/🦀️rust/📦️glue.rs`: mount `artifacts::<key>::snapshot` as a grouping module holding
`pack` and `schema`, and mount `artifacts::<key>::schema` and `artifacts::<key>::diff::schema`. The
taxonomy's `rustEntryPathRules` key documents that `#[path]` resolution is **cumulative** and that two
conventions are both valid in this repo — follow whichever your glue file already uses; the new nesting
adds one level. Mirror in `📦️packages/🟦️typescript/📦️index.ts`.

Register an `ArtifactSchemaDescriptor` for `s.<plugin_key>.<artifact_key>` with all fifteen leaves via
`include_str!` (API in `🧪wave3-report.md`) so the framework's table-driven parity test covers your
artifact at runtime. Apply `#[derive(ArtifactSchema)]` with `#[artifact_schema(id = …)]` and per-field
`#[state(…)]`.

## Gotchas the pilot hit — read these before writing a single leaf

1. **First type wins** in every leaf: declare `XArtifact` / `XSnapshot` / `XDiff` first, helper types
   after. The extractors take the first top-level declaration as the facet type.
2. **No top-level fixed-length list.** GraphQL and proto both flatten it to a plain list, which trips
   the cardinality-parity scanner. Flatten such a field into named components or nest it in a record.
3. **Optional lists in the diff** must be wrapped in a named scalar record in all five formats
   (`XStringList`-style), not expressed as an optional array.
4. **`Option<Option<T>>`** is JSON `{"oneOf":[{"type":"null"}, T]}`, TS `?: T | null`, Rust
   `Option<Option<T>>`.
5. **`XSnapshot` is declared in `📸️snapshot/🧬️schema`**, and the artifact root re-exports it. Do not
   define it twice.
6. Never redeclare the GraphQL `@state` directive in a leaf — it lives once in the framework schema
   module's shared preamble.
7. In the diff runtime file, import the facet types with `pub use super::schema::*;` — a bare
   `schema::` path can resolve to the extern crate instead.
8. Only the mutation **enum** carries `type Diff = XDiff`. Do not add a per-mutation `MutationDiff`
   impl unless that same file also has its `DiffCodec` / `dsl::DslDiff`.
9. Sparse `reordered` collapses duplicate ids — seed any `<prefix>-N` serial from the existing maximum
   id rather than from the collection length.
10. `DocumentApp` and its views now use `.snapshot`, and the "replace the whole document" mutation is
    `SetSnapshot { snapshot }`. If your artifact has a `set-projection` mutation folder, rename it to
    `set-snapshot` along with its types.

## Gates — run for real, iterate until green, paste verbatim tails

```
cd /Users/ueli/Documents/semio && DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p <crate>
cd /Users/ueli/Documents/semio && DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p <crate> --lib
cd /Users/ueli/Documents/semio && bun ./📜️script.ts policy 2>&1 | rg -i '<plugin-name>'
```

The third must be empty of artifact-schema breaches for your plugin. If your crate has TypeScript
tests, run them directly with `bunx vitest run` inside the package — the nx wrappers hit budget limits.

## Report

Write `🧪wave5-<plugin>-report.md` into the ticket folder: per artifact, the final field inventory with
state classes, the diff-delta shape, the glue convention you followed, the three gate tails verbatim,
and any shared-surface blocker for the fixup wave.
