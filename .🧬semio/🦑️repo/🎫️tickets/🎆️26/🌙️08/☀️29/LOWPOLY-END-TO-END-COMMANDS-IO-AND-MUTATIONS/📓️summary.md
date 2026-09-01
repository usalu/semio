# 💠️ Lowpoly End To End — Commands, IO and Mutations

Consolidated result. Per-area detail lives in `📓️research/`.

## 1. The headline defect

Lowpoly declared 47 editor commands via `app_commands!`, but only **19** were
`InteractiveJobClassification::Migrated`. The other **28 were unreachable at runtime**: the framework's
`validate_ui_dispatch_classification` rejects any non-`Migrated` tool with
`Fault("interactive-job.not-ui-safe")` before a handler runs, and `build_tool_job` returns `Ok(None)`
for them, producing a second hard error `interactive-job.missing-owned-builder`.

Everything a user would call "lowpoly editing" was in that dead set — every mesh operation
(extrude, inset, bevel, loopCut, subdivide, triangulate, mirror, decimate, flipFaces, merge, dissolve,
snap, toggleSmooth), every selection transform, every UV operation, most painting, and `addPrimitive`.
The handlers were all fully implemented; they simply could not be dispatched.

**Now 47/47 `Migrated`, 0 `BatchOnlyPendingRewrite`.** `LOWPOLY_BATCH_ONLY_TOOL_IDS` is deleted.

## 2. What that required

Per command: exact admission (`lowpoly_command_admitted`), exact Store publication authority
(`lowpoly_artifact_mutation_retained_bytes` is now exhaustive over all 17 `LowpolyMutation` variants,
where before it admitted only 4), a disposition, a `ArtifactToolPublicationContract`, a
`ToolExecutionContract`, a reduce arm, a route-table entry, and the manifest classification flip.

Two dispositions had to be added beyond the original six:
- `ArtifactConfig` — `addPrimitive` emits both a `CreateObject` artifact mutation and a
  `SetActiveObject` config mutation.
- `ArtifactConfigTransient` — `addPrimitive` also reads *and writes* the session `LowpolyScratch`
  (`build_doc` / `set_mesh_workspace_map`). Without a `Transient` lane it would run against a blank
  scratch and silently no-op after any prior mesh edit.

A non-obvious correctness finding drove the whole shape: every command that builds a compute session
needs `LowpolyScratch` **rehydrated from the live persisted `LowpolyTransient`**, not
`LowpolyScratch::default()`. Otherwise `LowpolyDocument::reload_meshes` fails `StaleMeshWorkspace` and
the command returns `Emit::default()` — a silent no-op. Handled by a `threaded!` macro that rehydrates,
runs the untouched handler, then republishes the scratch as an ephemeral `Transient` mutation.

## 3. IO — the advertised-but-broken layer

Reconciling two contradictory audits established the truth. At `HEAD`, **obj/ply/png/dwg/stl/gltf/las
all compiled but always failed at runtime**: `encode_pack::<LowpolySnapshot>` was piped into
`decode_pack::<TargetFormat>`, guaranteeing a pack-envelope mismatch. Only `txt` was an honest stub.
So the earlier "8 of 9 formats work" audit had mistaken *compiles* for *works*.

Current state — 5 real, 4 honest stubs, all 9 registered in both directions:

| Format | Export | Import |
|---|---|---|
| txt  | real — DSL body verbatim | real — `parse_dsl` |
| json | real — `serde_json` | real — `serde_json` |
| obj  | real — DSL in `ObjUnknownStatement` retention slot | real |
| ply  | real — DSL in `PlySnapshot.comments` | real |
| png  | real — paint-layer RGBA raster + DSL in a `tEXt` chunk | real |
| stl / gltf / dwg / las | explicit `Err` | explicit `Err` |

The four stubs are an **architectural limit, not laziness**: `LowpolyObject.mesh` is an
`Option<ArtifactChild<S>>` content-addressed handle, and `ComposeSource` carries only
`{dialect, payload}` with no store resolver — so a synchronous io serializer genuinely cannot reach
mesh vertices. They now fail loudly instead of emitting empty-but-valid files, and none were quietly
dropped from the advertised kinds list.

## 4. Schema truth

- All 17 mutations and the domain types now exist consistently across json-schema / rust / typescript /
  protobuf / graphql. The mutations proto/graphql/json previously held a verbatim *wrong copy* of the
  artifact-lane schema.
- Removed a stale `meshJson: string` field that no longer exists in the Rust model, across the artifact,
  snapshot and diff levels in all four non-Rust representations.
- Fixed three per-mutation payload schemas (byte fields typed as int arrays instead of base64 strings;
  `create-object` missing `mesh`) → ajv now validates **17/17** recorded fixtures.
- Fixed a misnested `paintLayers` in the diff TS and a dangling `$ref` to an undefined `LowpolySelection`.

## 5. TypeScript

- `📦️index.ts` exported `🪓️decomposer/🟦️component.ts`, a path deleted in commit `0e2007af53` — a hard
  module-resolution failure for any consumer. Removed; remaining export paths corrected.
- `🚪️io/🟦️component.ts` was an `export {}` stub behind a stale "lands in W7" marker (that ticket closed).
  Implemented against cad's pattern: format list, accept filter, export menu, host-bridge interface and
  installer, async import/export.
- `package.json` was a verbatim copy of cad's — CAD description, and `scripts` that ran
  `@semio-tech/cad-js` targets. Rewritten against actual imports (`ajv` only).

## 6. Tests

Discovered cases **169 → 172**. Added `io-lowpoly-1` (9 format round-trips),
`command-lowpoly-1` (14 scenarios across all command groups), and `io-lowpoly-png-oracle-1`.

The last one closes CLAUDE.md's third-party-validation requirement, which lowpoly previously did not
meet (its only oracle was a second in-house implementation). PNG is the one format with
third-party-verifiable output, and **Pillow 12.2.0** is genuinely available in `.venv`. The oracle was
proven real by fail-injection: corrupting the pixel-equality check produced a real `AssertionError`,
and the file was restored byte-identically (MD5 confirmed).

`🧪️oracle/🔣️.json` records honestly what the oracle does *not* cover and why.

## 7. Deliberately unchanged

The viewer's 14 empty facet folders and its placeholder-mesh fallback were audited against 9 viewer
instances across cad/block/space/puzzle: every one shows the identical all-empty pattern, explained by
the `NoConfig`/`NoPresence`/`NoTransient` sentinel types every viewer uses. Composed-child mesh
resolution is structurally unreachable — `LowpolySnapshot` never implements `ArtifactRefs` and nothing
calls `register_child`, so `doc.children` is provably always empty. No files were invented to make
folders look populated.

## 8. Repo-wide incident fixed

An agent left a standalone `[workspace]` overlay with absolute paths in
`📦️packages/🦀️rust/Cargo.toml`. That made cargo report `multiple workspace roots found` for **every
crate in the repo**, breaking all concurrent sessions. Restored to committed content via
`git show HEAD:<path> > <path>`. Also removed all 9 `[DEBUG]` markers under the plugin (5 pre-existing
temporary Rust diagnostics including an assert-nothing fixture-capture scaffold that sibling plugins had
already retired, and 4 newly-introduced misuses on permanent messages).

## 9. Handoffs

- `raster` and `remodel` carry the identical dead-decomposer export bug in their own `📦️index.ts`.
- The PNG oracle validates Rust's declared intent, not yet its actual `encode_png()` bytes — needs one
  cargo-backed run to close.
- `command-lowpoly-1` asserts command identity rather than dispatch: `ArtifactView`/`ConfigView` are not
  linkable from any generated Rust test host, for any plugin — a Cargo dependency-graph gap.
- The diff proto/graphql keep their own shallow `LowpolyObject`; a repo-wide `Option<T>`-null convention
  gap remains outside the diff lane.

## 10. Verification verdict

| Gate | Result |
|---|---|
| `nx run @semio-tech/lowpoly-js:test --skip-nx-cache` | **PASS** — 47 Migrated / 0 BatchOnly, Ajv hostile oracle rejects duplicates, empty lanes, non-null blocker on Migrated, lane/preparation mismatch |
| `bun ./📜️script.ts test discover` | **PASS** — 172 cases (was 169), all three lowpoly cases discovered with their adapters |
| ajv over recorded mutation fixtures | **PASS** — 17/17 |
| Python third-party PNG oracle (Pillow 12.2.0) | **PASS**, and proven falsifiable by fail-injection + byte-identical restore |
| Scoped `tsc --noEmit` on edited TS | **PASS**, proven real by typo-inject/revert |
| `cargo check / clippy / test -p semio-s-plugin-lowpoly` | **BLOCKED — not run to completion** |

The cargo gates are blocked transitively, not failing. The run of 2026-09-01 17:20–17:32 produced 2199
error lines: **2196 in `✏️s/🔌️plugins/🗄️stdio`, 0 in `🧰️framework`, and 0 in `✏️s/🔌️plugins/💠️lowpoly`**,
terminating at `error: could not compile 'semio-s-plugin-stdio' (lib)`. stdio is a hard dependency of
lowpoly's io layer, so lowpoly is never compiled. Its working tree holds 1410 uncommitted files from a
peer session's serde-trait-bound refactor, parked with no `.rs` write for 30+ minutes.

Consequently every Rust test added or changed by this ticket is **written but unrun**, including the
`addPrimitive` regression test and the io format-honesty tests. That is stated plainly rather than
claimed as passing. Re-run the four cargo commands above once the peer session lands stdio.

Machine conditions throughout: up to 12 concurrent peer sessions, load average peaking at 143, and
sccache serialising every build behind a shared target-dir lock. An isolated `CARGO_TARGET_DIR` with
`RUSTC_WRAPPER=""` was tried to escape both and was OOM-killed twice with ~60 MB free.
