# Sol Independent Audit: P3 Presenter/GPU Retirement — 2026-08-23

## Verdict

**REJECT — source packet.** The owned presentation/retirement design is substantially present, but
the live source does not pass its declared Rust-edition parser/format gate and its production stale
revision/generation check is tautological. No production source was changed by this audit.

Terra admission was scheduler-blocked, so this is an independent Sol High source audit rather than
a Terra verdict. Cargo, Nx, Wasm, browser, network, runtime timing, and root lint were not run.

## Blocking Findings

### 1. The packet does not parse under the crates' declared Rust edition

Both audited manifests declare `edition = "2021"` and `rust-version = "1.88"`:

- `framework/ui/.../rust/Cargo.toml:4`
- `os/renderer/.../wgpu/Cargo.toml:4`

The live packet uses Rust-2024-only let chains at:

- renderer `📦️glue.rs:7784` and `:7791`;
- renderer `🦦️winit_app.rs:175`.

`rustfmt --edition 2021 --emit stdout` therefore rejects those files. The edition-2021 scoped
`rustfmt --check` also reports formatting drift in `prepared.rs`, `draw.rs`, `gpu.rs`, and
`os_host.rs`. The earlier report's edition-2024 parser result cannot establish source coherence for
crates whose manifests still select edition 2021.

### 2. Production stale admission validates the packet against itself

The only production calls to `GpuContext::{begin_prepared, begin_prepared_offscreen}` are in
`AppPresentPhase::BeginGpu`. That branch computes:

```rust
let revision = packet.scene_revision();
let generation = packet.preview_generation();
```

and immediately passes those same values as `live_revision` and `live_generation`. Consequently,
`PreparedRenderGate::validate` cannot reject a packet stale relative to the actual app/frame
authority on the live route. The unit gate fixture supplies independently different values, but the
production caller does not. This leaves stale revision/generation owner preservation unproved and
permits an obsolete candidate to proceed to upload/stage/render.

An independently reconstructed 28-condition source probe scored **27/28**; the only failing
condition was an independent live freshness authority. The permanent Rust source-mutation fixture
contains 13 mutations and does not mutate this self-validation call shape, so it cannot catch this
regression.

## Source Findings That Pass

| Contract | Result | Independent evidence |
| --- | --- | --- |
| Single owned prepared packet | PASS by source inspection | `PreparedRenderPacket` has no `Clone` derive and the packet/gate path has zero `Arc<PreparedRenderPacket>` occurrences. Receiver, frame, pending gate, last-valid gate, replacement, and retirement use owned values. |
| Exact presenter witness | PASS | `PreparedPresenterWitness` is non-Clone and carries sequence, scene revision, and preview generation. `acknowledge_presented` compares all three before `last_valid.replace`. |
| Stage/render/ACK order | PASS apart from live freshness blocker | `Stage` transfers the exact packet into the gate; `Render` borrows only `pending_presented(witness)`; `finish_prepared` precedes `acknowledge_presented`; only the exact ACK swaps last-valid. |
| Missing/stale/duplicate/supersede/fault owner paths | PASS for witness/ownership behavior | Missing/stale witness branches call `abort_pending`; a pending gate rejects a superseding owned packet; duplicate ACK returns the scalar witness; aborted/completed frame owners enter `AppPresentedRetirement`. Old last-valid remains visible until exact ACK. |
| Deferred presenter retirement | PASS | `AppPresentedRetirement` retains the previous packet and completed frame. It closes an active upload first, scans acknowledged upload versions one at a time, applies one eviction/table step, retires the old packet incrementally, then closes the completed frame. |
| Fixed mesh table | PASS within this packet | `MESH_GPU_TABLE_CAPACITY = 256`; storage is `[Option<MeshGpuEntry<T>>; 256]`; the table region contains zero `HashMap` and zero `.retain(`. Fixed 256-byte keys plus exact key/version identity prevent stale ABA lookup. |
| Admission and exact handback | PASS by source inspection | `meshes.is_full()` is checked before GPU buffer creation. Final insertion returns the exact rejected entry; both buffers are restored to the retained upload cursor. The capacity-plus-one fixture checks pointer identity. |
| One-entry/key/all retirement | PASS | Exact, key-except, and all selectors share a cursor that inspects one slot or destroys one buffer per call. Closing first retires the active upload, then scans all fixed slots. |
| Same-key new version survival | PASS | After ACK, the presenter scans one upload per grant into a fixed 256-version set. `KeyExcept` selects only same-key versions absent from that set, and the source fixture distinguishes versions 7, 8, and 9 plus another key. |
| OsHost close ordering | PASS for the named seam | `OsHostRetirement::close_step` requires `presenter.close_world_owners_step()` and `world_owners_terminal_is_empty()` before `runtime.close_world3d_dynamic_step()`. Presenter close drains pending/staged/last-valid owners, active upload, and fixed mesh table first. |

## Fixtures and Mutation Quality

The authored runtime-free Rust fixtures are meaningful by inspection but were not executable under
this audit's no-build instruction:

- exact one-shot ACK, old-visible-before-ACK, stale/duplicate ACK, missing ACK handback,
  superseding packet pointer identity, and gate close terminal witness;
- fixed table capacity plus one with exact pointer handback, stale ABA version lookup, and
  acknowledged same-key version preservation;
- source-order checks for presenter close before World close.

The permanent presenter-retirement predicate has 13 mutation variants: ACK-phase removal,
unchecked ACK, abort removal, Arc packet authority, direct old-packet drop, stage removal, aborted
phase removal, keep-version bypass, unbounded capacity, HashMap replacement, whole-close rename,
terminal-witness bypass, and admission bypass. Each targets a real required symbol/path. However,
the claimed 28-rule Bun predicate is not a permanent repository command or fixture, and neither it
nor the 13 permanent mutations rejects deriving the alleged live freshness values from the packet
itself. That omission is material, not only an evidence-format concern.

## Commands and Results

```text
rustfmt --edition 2021 --check --config skip_children=true <six audited Rust files>
# FAIL: formatting drift; renderer source also contains Rust-2024-only let chains

rustfmt --edition 2021 --emit stdout --config skip_children=true <six audited Rust files>
# FAIL: glue.rs:7784/7791 and winit_app.rs:175 are invalid in edition 2021

bun ./📜️script.ts verify interactivity --self-test --format json
# exit 0; DENY mode clean; one recorded blocking-bridge finding

bun ./📜️script.ts verify interactivity --format json
# exit 0; DENY mode clean; one recorded blocking-bridge finding

bun -e <independent 28-condition presenter/GPU source probe>
# 27/28; independent live freshness authority FAIL

rg/sed exact ownership, table-region, call-site, close-order, and mutation censuses
# packet Arc 0; table HashMap/retain 0; permanent mutations 13;
# OsHost close order presenter step -> presenter terminal -> World close

git diff --check; git diff --cached --check; git diff HEAD --check
# scoped and whole working/staged/HEAD checks PASS
```

## Required Repair

1. Make the packet parse and format under the manifest-selected edition 2021, or atomically change
   the actual crate edition with the repository-wide consequences validated. A source-only audit
   cannot accept edition-2024 parsing as a substitute.
2. Give `AppPresenter` an independently owned current scene revision and preview generation (or an
   equivalent exact frame authority), revalidate them immediately before GPU admission and again
   before the ACK publication boundary, and return the exact candidate to retained retirement on a
   stale result.
3. Add a permanent mutation that replaces the independent live authority with packet-derived
   values and proves the predicate rejects it. Add live-route stale revision and generation fixtures,
   not only direct `PreparedRenderGate::validate` unit calls.

## Explicitly Unaccepted Residuals

Regardless of this rejection repair, Phase 3 remains **RED** for the separately scoped submit and
surface-present timing, engine realization, atlas ownership, raster ownership, dynamic cache
ownership, full `GpuContext`/presenter/realm teardown, semantic codecs, Cargo/type validation,
Wasm/browser behavior, and runtime timing. This audit makes no acceptance claim for those areas.
