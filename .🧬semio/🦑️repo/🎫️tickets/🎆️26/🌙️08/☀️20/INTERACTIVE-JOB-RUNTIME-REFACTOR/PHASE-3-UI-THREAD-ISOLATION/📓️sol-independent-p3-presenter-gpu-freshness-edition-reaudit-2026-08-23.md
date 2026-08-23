# Sol Independent P3 Presenter/GPU Freshness and Edition Re-Audit — 2026-08-23

## Verdict

**ACCEPT — the narrow P3 presenter/GPU freshness and Rust-2021 source repair.** The two blockers in
`sol-independent-p3-presenter-gpu-retirement-audit-2026-08-23.md` are repaired: all seven audited
sources parse and format under their manifests' Rust 2021 edition, and production admission now
uses a retained runtime authority that is independent of the prepared candidate.

This is an independent Sol High source re-audit. Terra admission was scheduler-blocked, so this is
not a Terra verdict. I made no production edit; this report is the only file authored by the audit.
Cargo, Nx, Wasm, browser, network, root lint, and runtime timing were not run.

## Edition and Parser Repair

Both audited crates still declare `edition = "2021"` and `rust-version = "1.88"`. Canonical
`rustfmt --edition 2021 --check --config style_edition=2021,skip_children=true` passed together on:

- framework UI WGPU `prepared.rs`, `draw.rs`, and `gpu.rs`;
- product renderer WGPU `os_host.rs`, `glue.rs`, `winit_app.rs`, and `browser_worker.rs`.

Each source also passed a separate edition-2021 `rustfmt --emit stdout` parser invocation. An exact
seven-file scan for `if let ... &&`, `while let ... &&`, and `&& let` returned zero. The earlier
Rust-2024 let-chains are now ordinary nested Rust-2021 conditionals; no manifest edition changed.

## Independent Freshness Authority

`RuntimeMailboxInner` owns `presentation_authority: RuntimePresentationAuthority`. The authority is
constructed with the mailbox, not with a frame or packet, and native plus dedicated browser
presenters each retain a clone of that mailbox authority. It owns independent atomic
`scene_revision` and `input_generation` values.

The revision changes at both accepted runtime-completion edges:

1. `enqueue` first retains the completion in the bounded queue, then calls
   `presentation_authority.mark_scene_changed()`; and
2. `finish` returns a completion to the bounded queue, then calls the same revision advance.

There are exactly two production `mark_scene_changed()` calls. Native frame observation calls
`observe_presentation_input_generation(build_generation.0)` before submitting/polling the frame.

`AppFrameTransaction::Build` calls `runtime.presentation_witness_for(self.generation.0)` before
`frame_before_input` constructs `PreparedRenderInput`. The independent witness supplies both
`scene_revision` and `input_generation`; a stale input generation faults before candidate
construction.

`AppPresentPhase::BeginGpu` rereads `self.presentation_authority.current()` and passes those values
to both `begin_prepared` and `begin_prepared_offscreen`. Immediately before exact acknowledgement,
`AppPresentPhase::Acknowledge` rereads the authority again and compares both values to the staged
packet. A mismatch aborts the exact pending packet into retained retirement and leaves last-valid
unchanged. Production source has zero packet-derived `let revision`, `let generation`, or
packet-derived expected-witness assignments.

## Preserved Ownership and Close Contracts

The previously accepted source invariants remain intact:

- `PreparedRenderPacket` is a single non-Clone owned value and has no production
  `Arc<PreparedRenderPacket>` authority;
- Stage transfers the exact packet, Render borrows it only through the exact non-Clone presenter
  witness, `finish_prepared` precedes ACK, and only exact one-shot ACK can replace last-valid;
- stale, missing, duplicate, superseding, render-fault, and close paths return the exact packet and
  frame owners to `abort_pending` / `AppPresentedRetirement`;
- previous packet and completed frame retirement remains cursorized after ACK;
- `MeshGpuTable` remains a fixed 256-slot registry with exact key/version ABA authority,
  capacity-before-buffer construction, exact +1 handback, one-entry retirement, and acknowledged
  same-key version preservation; the table region contains no `HashMap` or `.retain(`;
- `OsHostRetirement` still requires presenter close progress and terminal witness before dynamic
  World close.

## Independent Probes and Mutations

An independently reconstructed Bun source predicate passed **28/28** conditions. It covered packet
ownership, exact stage/render/ACK order, runtime authority construction and mailbox retention,
accepted/finished revision advancement, input observation, pre-candidate witness capture, native
and offscreen BeginGpu checks, ACK reread, zero packet-derived expected values, fixed mesh-table
handback/ABA behavior, and presenter-before-World close ordering.

A separate focused predicate denied **4/4** source mutations:

1. replacing the first independent presenter read with packet-derived revision/generation;
2. replacing the pre-candidate mailbox witness with candidate/job generation values;
3. deleting one runtime scene-revision advance; and
4. deleting native input-generation observation.

The permanent Rust fixture independently changes authority while retaining the candidate, then
changes only the candidate while retaining the authority. The repository source predicate requires
exactly one pre-candidate witness capture, exactly two presenter authority rereads, both native and
offscreen calls, the ACK comparison, exactly two revision advances, and native input observation.
The existing ACK/table/close mutation cohort remains present. Rust fixtures were inspected but not
executed because builds were prohibited.

## Executed Gates

| Gate | Result |
| --- | --- |
| manifests | PASS: both audited crates remain Rust 2021 / rust-version 1.88 |
| scoped edition-2021 rustfmt | PASS: 7/7 files |
| scoped edition-2021 parser | PASS: 7/7 files |
| Rust-2024 let-chain scan | PASS: zero |
| independent presenter/GPU source predicate | PASS: **28/28** |
| independent freshness mutations | PASS: **4/4 denied** |
| packet-derived expected-value scan | PASS: zero production assignments |
| scoped working/staged/HEAD diff checks | PASS |
| whole working/staged/HEAD diff checks | PASS |
| interactivity self-test | concurrent RED: five P1 DB-engine missing retained-history fixtures; zero P3 finding |
| interactivity plain DENY | concurrent RED later in the same audit window: two P1 DB-engine missing retained-history fixtures; zero P3 finding |
| Cargo/Nx/native/Wasm/browser/runtime | not run; RED/unverified |

The self-test/plain DENY counts changed while P1 was concurrently editing the DB engine. Every
reported finding was rooted at
`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:0`; none was in the seven
audited P3 files, and this audit made no allowlist or production change.

## Residuals

This ACCEPT is source-scoped only. Phase 3 remains RED for submit/surface-present timing, engine
realization, atlas and raster ownership, dynamic cache ownership, complete GPU/presenter/realm
teardown, semantic codecs, Cargo/type validation, native/Wasm/browser behavior, and runtime timing.
Opaque GPU destruction and platform-present latency remain unproved.
