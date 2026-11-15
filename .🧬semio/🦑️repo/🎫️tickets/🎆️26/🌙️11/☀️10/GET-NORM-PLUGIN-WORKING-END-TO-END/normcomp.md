# normcomp: hub norm component to wasm (interim)

Status: blocked, no compile result yet.

## Invocation
- Target `wasm32-wasip2` (PLUGIN_WASM_TARGET), cargo profile `wasm-release`.
- Check: `repo cargo --store=shared --configuration=prototype -- check --keep-going --target wasm32-wasip2 -p semio-hub-norm-component` from the crate workspace directory (script: `normcomp-check.sh <check|build> <logname>`, extra flags via `EXTRA`).
- Build: `EXTRA="--profile wasm-release" normcomp-check.sh build <logname>`; whether `repo cargo` accepts `--profile` next to `--configuration=prototype` is unverified.

## Finding 1: missing generated icon-name file
- `repo cargo check` fails in the Rust source scan: `module icon_name has no single matching file` for `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🧬️schema/🦀️.rs`.
- Cause: it includes `../🤖️generated/🪪️icon-name/🦀️.rs`, a gitignored output of the `@semio-tech/assets:build` target; it was never generated on this host.
- `repo run @semio-tech/assets:build` was cancelled twice for host memory pressure (below the 256 MB floor) while other agents ran graph builds; running `repo ./📜️script.ts build` directly refuses without a parent process envelope.
- Coordinator decision: agent hostup is the single graph driver and generates the file; this agent only polls for it.

## Static review (no build)
- Norm component source is one `plugin_exports!` line; no wasm32 cfg problem visible.
- Reactor-context exports: `new_bound` is `pub fn`; `ValueRefusalKind` is imported once. The frame borrow fix was not reviewed.

## Finding 1 update: generated inputs
Generated without a graph run (repo-owned generators via `repo tools exec --light bun`): `T/normcomp-generate-icons.ts` (icons: icon-name, icon_svgs, shortcodes), `T/normcomp-generate-more.ts` (schema entity-kinds, metabolism icons). `T/normcomp-missing-generated.py` lists further missing generated inputs (plugin registries, hub flow/stdio, actor typegen); none was reported by the norm check yet.

## Progress of `check` (logs normcomp-c3..c5)
- c3: `semio-plugin-decl-app` `has_failure` -> `failure_untaken` (already fixed by another agent).
- c4: `visual-chrome` `ramp_for` non-exhaustive `ResultField` (already fixed by another agent).
- c5 (`T/🗑️generated/normcomp-c5-keep.log`): framework and plugin libraries all compile; 23 norm artifact crates fail with 536 errors. Nothing wasm-specific yet.

## Remaining errors (c5), by root cause
1. Norm artifacts still use `BTreeMap` in `#[derive(RetainedClone)]` types (vdi3805 about 120, iso16757 about 250, plus en1998/en1999 and others). The framework offers only `RetainedOrderedMap` (`🌱️value/🧬️retained-clone/🗺️ordered-map`); gis and procedural plugins already migrated. Needs a per-crate type migration (145 norm files mention BTreeMap).
2. iso16757: `CatalogueValue`/`CatalogueId`/`PartNumberRule: DslField`, `LocalizedText: Serialize/Deserialize` missing.
3. en1997 (E0252 duplicate names, E0308), en1999/en1998/en1994 E0308 mismatched types, en1990 `ChangeTestTheoryVariation: MutationLeaf`, en1998 `NativeSnapshotFailure: From<SqliteStage..>`.
4. din4108 / din16798 / viewers: E0117 orphan impls on foreign types, missing `semio_framework_job` dependency in the crate manifest.

## Next
Per-crate norm Rust wave (one agent per crate group, one cargo at a time), then `build --profile wasm-release` for the component.
