# Current Architecture Audit

Read-only audit on 2026-10-14. Existing production-boundaries report was read, followed by actual Cargo manifests and physical source mounts. No compiler, gate or runtime test was executed in this audit. Counts below are direct whole-file measurements, not authenticated cfg-selected admission metrics.

## Recommended Next Cut

Extract CPU DrawList contracts first, then Prepared upload staging into distinct real Cargo owners. This is the largest directly evidenced downward UI cut without inventing worker interfaces: Prepared has 3,798 whole-file lines /207,982 bytes; draw-types has1,609 lines /75,909bytes. Combined they exceed262,144bytes, so they MUST NOT be one crate. Both are still mounted by the giant UI root. Each separately appears below production ceilings before cfg selection; actual Source helper admission must prove that, including their embedded inputs.

Exact original production sources:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs`
- Mount consumer `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`
- Manifest consumer `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`

Draw-types production imports only canonical scene math/records, Theme/GlassStyle/Rgba and geometry, plus existing bytemuck derives. Replace all `crate::wgpu` and `super::kernel_3d_scene` references with firstparty scene/theme owners. Keep record ownership with original UiInstance/VectorVertex/DrawLayer/DrawList implementations. Its bytemuck traits must remain an implementation detail: public draw records are firstparty structs, not external aliases. Do not pull renderer/GPU/Prepared upward into draw records.

Prepared production imports DrawLayer/DrawList/ScissorRect, Mesh3dLease/other scene records, Job contracts, ValueError/ValueRefusalKind and raster ownership. Its current raster references already point downward to the new canonical raster owner. Prepared therefore depends on draw-contract + scene + job + value + raster-owner. No Prepared -> GPU, Engine, Host, Paint or UI facade edge is required by the inspected source. Preserve original allocator/retirement functions and public firstparty Prepared types beside this owner.

Move pure tests to their real owner; leave renderer integration laws consuming declared ports. Original mounts requiring classification are draw-types affine unit, selection-marquee unit, draw-types inline scene/camera tests, Prepared unit and Prepared atlas tests. The last mount is `all(test, feature="wgpu-engine", not(target_os="wasi"))`; importing the giant renderer into a leaf test defeats the limit and must be converted to a kernel-owned integration law where necessary. The current concurrent prepared/helper hash changes are already documented: do not refresh expected hashes without semantic review.

## Subsequent SCC Inversions

UI layout still depends upward on Widgets for TREE_ROW_HEIGHT at layout line378. Move that canonical metric into Theme/metrics and have Widgets consume it. Mounted-layout imports Engine::UiSurfaceToken at line6, while Engine consumes mounted-layout; move the original identity record into a neutral surface identity/ownership contract. Mounted-layout also consumes Flex, Layout, Text, Theme, retained Tree and tree-window measure contracts. These are the exact dependencies to declare before a later mounted-layout Cargo cut.

Input mixes pure bounded input state and retained registration: retained row registration reaches mounted-layout/layout/chrome/tree/arena near lines828-953. Events reaches layout, mounted-layout, retained tree, widgets-derived slider/stepper helpers and chrome. Splitting files without cutting those owner edges is insufficient. Pure input state can be a leaf after moving retained registration to its actual input/layout integration owner. Layout/events/paint/engine require an explicit canonical metrics/worker/painting direction, not a fake UI facade dependency.

Store root visibly consumes snapshot_read, document_store_owners, replay preparation, retained codec catalog, composition group/open/hydration and paged edit owners while reexporting broad record contracts. Do not attempt a 54k-line Store leaf: snapshot-read retirement and ledger/replay execution remain ownership-coupled. Current envelope/command/cursor/projection cuts are genuine leaves; next Store cut needs exact original read/retirement ports, so UI Prepared is the lower-risk larger immediately evidenced cut.

## Admission And Test Gaps

The runtime `admitCargoSourceBoundariesV1` iterates every workspace compilation unit and tightens metadata budgets with Math.min against global policy. It does NOT skip test units. `actualCargoSourceAdmissionPortV1` correctly passes --test when unit.kind is test. Hence there is no evidenced runtime test-kind exemption in these inspected functions.

However the UI crate-boundary Bun gate uses one hardcoded aarch64/macOS production cfg snapshot without test or feature flags. Its success proves neither leaf native test closure size nor browser/Windows/Linux feature closures. Each newly extracted owner needs test-cfg admission and applicable target/feature witnesses. Registered build/check/test commands alone do not supply that evidence.

The gate labels rows "acyclic production contract", but checks dependency names and forbidden imports only in the entry file; it does not compute SCCs over the whole dependency graph or scan all selected mounted inputs for forbidden upward references. A child module can reintroduce a facade edge without this entry-token check noticing. Strengthen corpus checks to inspect actual selected closure and firstparty owner DAG, keeping public types original and rejecting duplicates.

The giant kernel root still names ui_wgpu and reexports many broad manifest/kernel contracts. UI root remains a wide feature aggregate with whole files above4k (Engine4,116; Draw5,645), and Paint3,909/Events3,388 are almost budget-sized before their closure inputs. These whole-file numbers are triage only. Existing report correctly refuses completion and distinguishes TOML/metadata/source gates from native compilation. No broader native success is inferred here.

## Completion Evidence Required

For each new owner: schema-first language-neutral ownership row, real manifest/Source entry and explicit consumer edge; ordinary Bun/Nx build/check/test launch routes in both modes; original production and test Source closure under4,000 selected lines /32files /262,144bytes; independent TOML/schema oracle; actual bounded native library/tests with unchanged original input hashes; removed enclosing physical mounts, no duplicate original implementation, and integration laws retained with their execution owner. Run only through admitted canonical controllers.
