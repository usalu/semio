# 📓️ terra — manifest-typegen — report

Packet `manifest-typegen` (wave W3): generate the TypeScript contract from `semio-framework-ui-contract`
and retire the hand-written `UiNode` mirror in `🛂️manifest/🟦️component.ts`.

## done

1. **New `tests/typegen_export.rs`** in the contract crate
   (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/tests/typegen_export.rs`) — a genuine
   `tests/*.rs` integration test, `#![cfg(feature = "typegen")]`, that calls
   `<Type as ts_rs::TS>::export_all_to(&dir)` explicitly for all 79 `#[derive(TS)]` types in the crate
   (enumerated by hand from `grep -n "cfg_attr(feature = \"typegen\"" 🦀️*.rs`, one call per type, grouped
   by source file in comments). Writing 79 explicit calls rather than relying on `export_all_to`'s own
   transitive-dependency walk from a few root types means completeness never depends on which types
   happen to be reachable from a chosen root.
   - **Why `tests/*.rs` and not a `#[cfg(test)] mod` inside the crate's own files, matching every other
     typegen crate in the repo (actor, manifest, async, shell):** this packet does not own the contract
     crate's `🦀️*.rs` files (see FORBIDDEN in the packet brief). A `tests/*.rs` file is a new file, not
     an edit to any of the 11 enumerated `🦀️*.rs` files, and Cargo auto-discovers `tests/*.rs` from the
     crate root regardless of the crate's custom `[lib] path = "📦️glue.rs"` — no `Cargo.toml` edit
     needed either (confirmed the crate already has `ts-rs` as an optional dep behind `typegen`, and the
     repo already has this exact pattern with plain-named files in
     `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/tests/*.rs`).
2. **`generate`/`check` commands** added to the contract crate's own
   `🧬️contract/📦️packages/🦀️rust/📜️script.ts` and registered in its `📋️project.json`, mirroring
   `@semio-tech/framework-rs:generate`/`:check`'s existing shape exactly (scratch `bindings/` dir → run
   the typegen test → strip ts-rs boilerplate → consolidate alphabetically, de-duplicated, into one file
   → write/byte-compare). Runnable as `bun nx run @semio-tech/ui-contract-rs:generate` /
   `bun nx run @semio-tech/ui-contract-rs:check`. (A peer session landed a `conformance` target in the
   same two files concurrently while I was editing; both edits are preserved — re-read from disk and
   reapplied after the first conflicting write, per U2.)
3. **Output file**: `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️ui-contract.ts` (not yet written —
   `generate` hasn't been run; see **acceptance**).
4. **`🟦️component.ts` surgical edit**:
   - Deleted the hand-written `UiNode` recursive-union mirror: `UiSectionNode`, `UiTreeActionPlacement`,
     `UiTreeItemAction`, `UiPeerMark`, `UiTreeItemNode`, `UiTreeSectionNode`, `UiTreeNode`,
     `UiControlNode`, `UiInputNode`, `UiSelectItem`, `UiSelectNode`, `UiToggleNode`, `UiGroupNode`,
     `UiKeyValueEntry`, `UiKeyValueNode`, `UiSliderNode`, `UiNumberStepperNode`, `UiRingNode`,
     `UiIconSelectNode`, `UiFieldNode`, `StyleSpec` (old shape), `UiButtonNode`, `UiTextNode`,
     `UiStackNode`, `UiDropOverlaySpec`, `UiSeparatorNode`, `UiImageNode`, `UiNode`,
     `UiInspectorFieldGroup` — confirmed by grep that every cross-reference to these 29 names was
     self-contained inside the block before deleting (no other part of the file touched them).
   - Deleted `PluginUiNode = Record<string, unknown> & { readonly type: string }` (one line, standalone).
   - Left the unrelated `WindowStackCorner`/`WindowLayoutWindowNode`/`WindowLayoutStackNode`/
     `WindowLayoutAxisNode`/`WindowLayout`/`NamedLayout`/`UtilityCategory`/`UtilityLeaf`/`UtilityNode`
     block (lines ~163–271, a *different* hand-written mirror, not named in the packet brief) untouched.
   - Added a new `// #region 🧬️GeneratedUiContract` block (same idiom as the existing
     `🧬️GeneratedMirror` region for `🟦️manifest.ts`): imports every one of the 79 contract types
     aliased `X as GeneratedX` from `./🤖️generated/🟦️ui-contract.ts`, then re-exports each under its
     bare Rust name — **except five that collide with an unrelated existing export already aggregated
     into the same `@semio-tech/framework` barrel** (`🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`),
     which are re-exported `Ui`-prefixed instead of shadowing:
     - `SurfaceKind` → `UiSurfaceKind` (collides with the artifact-editor `SurfaceKind` in the *other*
       generated bundle, `🤖️generated/🟦️manifest.ts`)
     - `WindowLayout` → `UiWindowLayout`, `WindowStackCorner` → `UiWindowStackCorner` (collide with this
       same file's own untouched OS-shell window-layout mirror, lines ~163–271)
     - `ActionId` → `UiActionId`, `Trigger` → `UiTrigger` (collide with `🔄️machine/🟦️component.ts`'s
       state-machine `ActionId`/`Trigger`, aggregated into the barrel via `export *` in `🟦️glue.ts`)
     All other 74 names keep their Rust name verbatim (`UiSnapshot`, `UiPatch`, `UiNodeRecord`,
     `Component`, `ContainerProps`, `TextProps`, `ButtonProps`, `StyleSpec` (new shape), `Label`,
     `Activity`, `Tone`, `Variant`, `LayoutSpec`, `BuiltNode`, `SurfaceDoc`, `SurfaceProps`, etc.).
   - **Collision-checking method**: not just against `🟦️component.ts`'s own remaining top-level exports
     and `🟦️manifest.ts`'s generated names — I also enumerated every top-level `export type/interface/
     const/function/class/enum` from all seven other modules `🟦️glue.ts` aggregates via `export *`
     (`🎯️action-bus`, `🧮️action-argument-resolution`, `🧬️schema`, `🖥️platform`, `🔺️mesh`, `🎠️kernel`,
     `🔄️machine`) and cross-checked all 79 contract names against that full ~311-name set. First pass
     only checked the two in-file sources and missed `ActionId`/`Trigger` (confirmed live via a real
     `tsc` run — see **acceptance**); the wider check found no others.

## acceptance

I do not run cargo (U4). No cargo command below was run by me; `sol` needs to run item (a).

- **(a) UNRUN — `bun nx run @semio-tech/ui-contract-rs:generate`** (runs
  `cargo test -p semio-framework-ui-contract --features typegen --test typegen_export`, then writes
  `🛂️manifest/🤖️generated/🟦️ui-contract.ts`). Also UNRUN: `:check` (byte-compare, second run should be
  identical — see **decisions** for why I believe this is idempotent by construction).
- **(b) RAN — repo-wide `bunx tsc -p tsconfig.json --noEmit`.** First run: exit 2, only 2 diagnostic
  lines, both a pre-existing fatal parse error in an unrelated concurrent packet's file
  (`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts(18,55)`:
  `runCargoTestBudgeted([], packageRoot, ["...rest]);` — an unterminated string literal, clearly a
  mid-save snapshot of someone else's live edit, not mine and not in my OWNS). This one fatal parse
  error aborts the whole `tsc` run before it reaches any other file, so I could not get a real
  repo-wide diagnostic list this way. Reran twice a few minutes apart; the file was still broken both
  times (not a transient single-frame snapshot).
  - Worked around it with a **scratch, uncommitted** tsconfig
    (`/private/tmp/.../scratchpad/scratch-tsconfig.json`, never written into the repo) that `extends`
    the real root `tsconfig.json` and adds one absolute-path `exclude` entry for that one broken file.
    `bunx tsc -p <scratch-tsconfig> --noEmit`: **exit 2, 10130 diagnostic lines across ~2131 files.**
    This matches ticket ruling U5's "workspace is externally RED" baseline — the vast majority of these
    8800+ errors are unrelated pre-existing breakage from the concurrent U-program asyncify pass, not
    from this packet. Full output saved at
    `/private/tmp/claude-501/-Users-ueli-Documents-semio/8fcf59e9-0317-475e-8aa4-dd949409752d/scratchpad/tsc-final.txt`
    (scratchpad, not the ticket folder — U8 rule 6 wants ticket-folder scratch as `.txt`/`.md`/`.json`;
    this is a large raw tool-output dump I'm treating as working material, summarized below and in the
    breakage inventory, not copied into the ticket folder verbatim).
  - Used this run twice: once right after the first edit (found the `ActionId`/`Trigger` collision, see
    **done**), once after the fix (confirmed 0 hits for that collision, same 63 deleted-name-related
    lines as before — the fix was purely additive-safe, it didn't touch or regress anything else).

## the breakage inventory

Two categories. Only the second is genuine downstream migration debt; the first is expected and
self-resolves the moment `sol` runs `generate`.

### 1. Pending-generate (1 file, resolves itself once `generate` runs)

- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts(197,8)`: `error TS2307: Cannot find module
  './🤖️generated/🟦️ui-contract.ts'`. Expected — I could not run cargo to produce that file. This did
  **not** cascade into files that import from `component.ts`; TypeScript only flags the one failing
  import statement.

### 2. Genuine `UiNode`-mirror breakage — 62 diagnostic lines across 16 files

Every line is `TS2305 has no exported member 'X'` or `TS2724 has no exported member named 'X'. Did you
mean 'Y'?` (the latter because `UiNodeId` — a real, still-exported contract type — is an edit-distance
neighbor of the deleted `UiNode`). Grouped by package/module:

| Package / module | File | Errors |
|---|---|---|
| `@semio-tech/framework-renderer-react` (`🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/`) | `📦️index.tsx` | 25 |
| ″ | `🧪️index.test.ts` | 1 |
| renderer-engine `🧱️elements/` co-location dirs (same product) | `Interpreter/🟦️component.tsx` | 6 |
| ″ | `ShellHelpers/🟦️component.tsx` | 3 |
| ″ | `ShellHost/🟦️component.tsx` | 1 |
| ″ | `Shell/🟦️component.tsx` | 1 |
| ″ | `PluginRuntime/🟦️component.tsx` | 1 |
| `@semio-tech/plugin-window-kits` (`🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/`) | `🌳️tree/🟦️component.ts` | 3 |
| ″ | `📄️document/🟦️component.ts` | 3 |
| ″ | `🖼️image/🟦️component.ts` | 2 |
| ″ | `🎬️media/🟦️component.ts` | 2 |
| ″ | `🧊️mesh/🟦️component.ts` | 1 |
| ″ | `📝️text/🟦️component.ts` | 1 |
| ″ | `📊️table/🟦️component.ts` | 1 |
| `@semio-tech/framework` barrel — `🔨️modules/🖥️platform/🟦️component.ts` | (same file) | 10 |
| `@semio-tech/framework` barrel — `🔨️modules/🎠️kernel/🟦️component.ts` | (same file) | 1 |
| **Total** | **16 files** | **62** |

Names most referenced: `UiNode` (13 sites, mostly `TS2724 … did you mean 'UiNodeId'?`), `UiTreeNode` (5),
`UiControlNode` (4), `UiStackNode` (3), `UiTreeItemNode` (3), `UiSectionNode` (3), `UiFieldNode` (2),
`UiImageNode` (2), plus one site each for most of the remaining leaf node types. This is the accurate
work list for whichever follow-up packet migrates these 16 files onto `UiSnapshot`/`UiNodeRecord`/
`Component`/`ContainerProps`/etc.

### 3. Bystander finding — pre-existing, not caused by this packet, flagged for awareness only

`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts` has two `TS2304 Cannot find name` errors
(`StyleSpec` at line 1142, `UiTreeActionPlacement` at line 1230) that **already existed before my
edit** and are unaffected by it: `manifest.ts` is a standalone ES module with no import statement for
either name, so it could never have resolved them via `🟦️component.ts`'s same-named exports regardless
of that file's contents — TypeScript modules don't share scope across files without an explicit import.
Root cause: `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs` (the old WGPU renderer's Rust
source, line ~2492) also `#[derive(TS)]`s its own `UiTreeActionPlacement`, and some other struct field
in `framework/🦀️component.rs` types as bare `StyleSpec`; neither ts-rs-emits into `🟦️manifest.ts`
itself nor gets imported by it. Not mine to fix (outside OWNS on both counts — the wgpu crate and
`framework/🦀️component.rs`), noted here only so it isn't mistaken for something this packet introduced.

## decisions

- **One bundled file, not ~80 per-type files** — same choice `@semio-tech/framework-rs:generate` already
  made for the 119-type `🟦️manifest.ts` bundle. The consumer is `🟦️component.ts`'s single `import
  type { ... } from "./🤖️generated/🟦️ui-contract.ts"` block; one import statement is far easier to keep
  correct by hand than ~80, and it matches the sibling file's shape exactly (same directory, same
  `Generated<Name>`-alias-then-re-export idiom).
- **`TS::export_all_to` from a driver test, not `#[ts(export_to = …)]` attributes** — the packet brief
  anticipated needing per-file `#[ts(export_to = …)]` attributes (which live in the contract crate's
  `🦀️*.rs` files, not owned by this packet) and asked me to either request them via registrar-request or
  find another way. `export_all_to` disregards `#[ts(export_to)]`/`TS_RS_EXPORT_DIR` entirely and writes
  every type (default filename `<TypeName>.ts`) straight into a directory I choose — so **no
  `#[ts(export_to)]` attributes are needed at all**, and there is no registrar-request for them (see
  **registrar-requests** below for the one request I do have, which is unrelated).
- **Verifying `number` not `bigint` for `UiNodeId`/`UiRevision`, `string` for `SurfaceId`/`Label`** —
  static verification only, since I cannot run cargo:
  - `🦀️document.rs`: `UiNodeId`/`UiRevision` both carry
    `#[cfg_attr(feature = "typegen", derive(ts_rs::TS), ts(type = "number"))]` — the explicit override
    that makes ts-rs emit `number` instead of its default `bigint` for a `u64` newtype. Confirmed by
    reading the source (not by running anything).
  - `SurfaceId` (`🦀️document.rs`) and `Label` (`🦀️component.rs`) are `#[serde(transparent)]` newtypes
    around a bare `pub String` with **no** `ts(type = ...)` override — none is needed, because ts-rs's
    default rendering of a transparent newtype around `String` is already `string`.
  - The crate already carries a dedicated regression test for exactly this claim,
    `document.rs::tests::wire_critical_newtypes_render_as_their_transparent_payload`, asserting
    `SurfaceId::inline() == "string"`, `UiNodeId::inline() == "number"`,
    `UiRevision::inline() == "number"`, `Label::inline() == "string"` via `ts_rs::TS::inline()` (written
    by a different packet, not this one). I read it and confirmed it checks precisely GOAL 4's claim; I
    did not run it (U4). `sol` should run
    `cargo test -p semio-framework-ui-contract --features typegen wire_critical_newtypes_render_as_their_transparent_payload`
    for runtime confirmation, and can additionally `grep` the emitted `bindings/UiNodeId.ts` /
    `UiRevision.ts` / `SurfaceId.ts` files after a `generate` run as a second, independent check (U8
    rule 8 — a query that can't report its own failure isn't evidence of absence; two independent checks
    here, one already in the repo).
- **Idempotency, argued but not measured** — `generate`'s pipeline (`rm bindings/` → fresh `cargo test`
  → `readdirSync().sort()` → strip boilerplate → de-dupe → join → `rm bindings/` again) has no
  timestamp, random ordering, or environment-dependent input anywhere I can see: ts-rs's own per-file
  header (`export.rs`'s `NOTE` constant) is a fixed string with no timestamp, and file content is a pure
  function of the Rust type definition. Two consecutive runs against an unchanged crate should be
  byte-identical. UNRUN (U4) — `sol`'s `check` run right after `generate` is the real proof.

## registrar-requests

One item, unrelated to `#[ts(export_to)]` (see **decisions** — that need didn't materialize):

- **Wire `@semio-tech/ui-contract-rs:generate` into the repo's "regenerate everything" entry point.**
  The closest thing to one is `runWorkspaceCodegen()` in the root `/📜️script.ts` (invoked by `setup`/
  `dev`), which currently calls `runNx("@semio-tech/framework-schema:generate")`,
  `runNx("@semio-tech/ui-styling-tokens:generate")`, `runNx("@semio-tech/graph-manifest:generate")`,
  `runNx("@semio-tech/plugin-registry:generate")` (around line 305 of root `/📜️script.ts` as of anchor
  `cb9bcce7a4`). Root `/📜️script.ts` is registrar-only (U7), so I can't add
  `runNx("@semio-tech/ui-contract-rs:generate")` there myself. Note that `@semio-tech/framework-rs:generate`
  (the sibling command that produces `🟦️manifest.ts`) isn't in that list either currently, so this
  isn't a regression — just an opportunity to fix both at once if `sol` wants to.

## deviations

- `stripTsRsBoilerplate`/`consolidateBindings`/`bindingsDir`-shaped helpers are duplicated (not
  imported) from `@semio-tech/framework-rs`'s `📜️script.ts` into the contract crate's own `📜️script.ts`.
  CLAUDE.md wants repeated code kept close together; the two copies are in different packages entirely,
  but the shared library package that could host a common helper
  (`🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`) is not in this packet's
  OWNS list, so extracting a shared helper there was out of scope. Flagging rather than silently
  accepting the duplication.
- Five re-exported names deviate from "same name as the Rust type" (`SurfaceKind`, `WindowLayout`,
  `WindowStackCorner`, `ActionId`, `Trigger` → all `Ui`-prefixed) — documented above under **done**, not
  optional given the barrel-wide collisions found by an actual `tsc` run.

## files touched

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/tests/typegen_export.rs` (new)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` (added `generate`/`check`;
  preserved a peer's concurrent `conformance` addition)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📋️project.json` (added `generate`/`check`
  targets; preserved a peer's concurrent `conformance` target)
- `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts` (deleted `UiNode` mirror block + `PluginUiNode`;
  added `🧬️GeneratedUiContract` re-export region)
- Not yet created: `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️ui-contract.ts` (written by
  `generate`, UNRUN)
