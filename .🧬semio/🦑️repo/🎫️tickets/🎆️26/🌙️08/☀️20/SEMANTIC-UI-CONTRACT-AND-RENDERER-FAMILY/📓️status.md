# Status — coordinator log (append-only, `sol` only)

Roles: `sol` = Opus coordinator (main chat). `terra-*` = Sonnet executors. `luna-*` = Haiku auditors.

---

## 2026-08-20 — W0 opened

Anchor commit `5e7b8046be44badd61d563b1fb0907b4b955851c`.
Ticket opened against goal `r2602/runningsketchpad`, issue
https://github.com/usalu/semio/issues/2570. `ticket.json` records `llm: opus-4-7` because the repo's
LLM enum predates Claude 5; the real coordinator is Opus 5 High and executors are Sonnet 5 High.

### Baseline capture (evidence: `📝️baseline-workspace-check.txt`, `📝️baseline-lib-check.txt`)

**The workspace is RED at baseline, from external churn — not from us.**

| Scope | Result |
| --- | --- |
| `cargo check --workspace --all-targets` | RED — `semio-framework-actor` (lib test), 499 errors |
| `cargo check --workspace` | RED — `semio-framework-number` (lib), 620 errors |
| `cargo check -p semio-framework-ui-styling` | GREEN |
| `cargo check -p semio-framework-ui` | GREEN |
| `cargo check -p semio-framework-geometry` | GREEN |

Attribution (per `feedback-live-predicate-not-derived-artifact`, settled with `git log`, not file
mtimes): both failures are the concurrent MICROKERNEL asyncify program. `🔢️number` and `🎭️actor`
were last committed in `d16fc1017c` (2026-08-19 15:51 +0200), well before our anchor, and carry no
uncommitted changes of their own. Every error is the asyncify signature — missing `.await`,
`impl Future` where a value is expected. The tree additionally carries substantial *uncommitted*
peer work right now (plugins `🧱️block`, `🏗️fem`, `📐️cad`, plus `Cargo.lock`).

### Ruling: gates are per-crate, not workspace-wide

`cargo check --workspace` cannot be a gate for this program while the peer's asyncify program is
in flight — it would make our waves hostage to their crates. Binding rules recorded in
`📌️important.md`:

- Every packet's acceptance is scoped `-p <crate>` to crates we own or created.
- The W-gate "no worse than baseline" is measured against the failing-crate set above
  (`semio-framework-number`, `semio-framework-actor`), not against zero.
- If a *new* crate name joins the failing set and it is not ours, that is F1 external red: record,
  poll, do not chase.

Our W1 dependency footprint (`serde`, `ui_styling`) is green, so W1 is unblocked.

### Environment

- Installed rust targets at open: `aarch64-apple-darwin`, `wasm32-unknown-unknown`, `wasm32-wasip1`,
  `wasm32-wasip2`. Cross triples for D3D12/Vulkan compile-checks added in W0.
- **Disk pressure: 77 GiB free of 926 GiB (92 % used).** The MICROKERNEL ticket previously filled
  this disk with per-ticket target caches. Do not introduce a separate `--target-dir`; reuse the
  shared workspace target dir. Re-check before the backend waves (W3), which add four crates.

---

## 2026-08-20 — W0 closed, W1 sub-wave A+B landed and GATED GREEN

Owner rulings taken this session (both recorded in `📌️important.md` as U1/U2): the new UI crates use
**literal sync `fn`**, overriding the U-program's R2 for exactly those crates; and **full speed**,
absorbing the peer's working tree rather than waiting for their SDK window.

### Registrar work (sol)
Scaffolded both crates so no executor ever touches the root manifest: `semio-framework-ui-contract`
(`🖱️ui/🧬️contract/📦️packages/🦀️rust`) and `semio-framework-ui-render`
(`🖱️ui/🖼️render/📦️packages/🦀️rust`) — `Cargo.toml`, `📦️glue.rs` with every module mount,
`📋️project.json`, `📜️script.ts`, and a compiling stub per region file. Root `Cargo.toml` gained two
members and two `workspace.dependencies` entries. Cross targets `x86_64-pc-windows-msvc` and
`x86_64-unknown-linux-gnu` installed for the later D3D12/Vulkan compile-only checks.

### Packets landed (6, all reported acceptance UNRUN per U4 — sol ran every gate)
`contract-doc` · `contract-layout` · `contract-action` · `render-scene` · `shader-repair` ·
`backend-iface`.

### Gate results — evidence in `📝️gate-w1-*.txt`

| gate | result |
| --- | --- |
| `cargo test -p semio-framework-ui-contract --all-features` | **61 passed / 0 failed** |
| `cargo test -p semio-framework-ui-render --all-features` | **45 passed / 0 failed** |
| contract `--target wasm32-wasip2` | GREEN |
| contract `--target wasm32-unknown-unknown` | GREEN |
| contract `--target wasm32-wasip2 --features typegen` | GREEN |
| `📜️script.ts boundaries` (ui-render) | GREEN — wgpu, winit, semio-framework-actor all absent |

The boundary gate was **proved to have teeth** before being trusted: the same `cargo tree --invert`
probe run against `semio-framework-ui --features wgpu-engine` exits 0 and prints the dependency path,
so the passing state is a real absence and not a query that can never fail (U8.8 / U-program rule 21).

### Cross-packet defects sol found and fixed at the gate (none were visible to any single packet)

1. **`SurfaceId` defined twice** (`document.rs` + `surface.rs`). `contract-layout` had pre-declared
   the tiebreak in its own docstring, so `document.rs`'s richer definition won and `surface.rs`
   re-exports it.
2. **`LayoutSpec`/`SurfaceProps`/`SurfaceKind` lacked `Default`** while sibling tests assumed it.
   `LayoutSpec`'s default is `Leaf` via a hand-written impl — `#[derive(Default)]` cannot express it
   because `#[default]` only accepts unit variants.
3. **`SeparatorProps` carried `rename_all` on a unit struct**, which ts-rs rejects outright.
4. **`UiPatchOp::Remove`/`SetRoot` were newtype variants over an integer under `#[serde(tag)]`** —
   serde cannot serialize that shape. It compiled clean and failed only at runtime, which is exactly
   the defect class the U-program logged as its rule 12. Converted to struct variants, then swept the
   whole crate with a script for sibling instances: these two were the only ones.
5. **The resource registry queued duplicate uploads** (dedup skipped only `Resident`, not
   `PendingUpload`) and left a redundant upload queued after a resource went resident. Fixed by
   coalescing — a second request in the same frame *replaces* the queued payload rather than being
   dropped, so changed pixels still win — and `mark_*_resident` now drops the stale op. Applied to
   textures and atlases; the mesh path was already correct (content-versioned).
6. **A backend test encoded the wrong protocol**: it assumed `push_raster_quad` makes a texture
   uploadable. It only *interns* — drawing references a texture by id, uploading is a separate
   request, which is what makes a raster quad cheap to emit every frame. The code was right; the test
   was fixed.
7. **The TypeScript mirror would have lied about every node id.** ts-rs renders a bare `u64` as
   `bigint`, but serde writes a JSON number and `JSON.parse` yields a JavaScript `number` — a
   declared type that never occurs at runtime, which would have broken the React renderer on its
   first `nodes.get(id)`. Found by probing `TS::inline()` rather than assuming. `UiNodeId`/
   `UiRevision` are now pinned to `number` and **locked by a permanent test**
   (`wire_critical_newtypes_render_as_their_transparent_payload`) asserting the rendering of every
   wire-critical newtype. The 2^53 ceiling is documented and unreachable for per-surface monotonic ids.

### The shader corruption is confirmed and now permanently guarded

`shader-repair` found **18 corrupted WGSL entry points** (`async fn vs_main`/`fs_main`) — one uniform
corruption class, no second class. Three naga tests now run on every build:
`all_canonical_shaders_parse_and_validate`, `pipeline_entry_points_exist_in_shader`,
`vertex_attribute_offsets_fit_declared_stride`. `WORLD3D_TEXTURED_SHADER` was traced and confirmed
**dead** (unimported in `draw.rs`, no other references) — kept in the contract, flagged as inferred.

Two more dropped-future bugs were confirmed at `draw.rs:520` and `:527`
(`push_dashed_line`/`push_dashed_line_overlay` calling `push_line` without `.await`, so those calls
did nothing). Both vanish by construction in the sync port.

### External red — NOT ours, do not chase (F1)

`semio-framework-ui --features wgpu-engine` fails with **854 errors**. Attribution settled by file,
not by guess: **zero** are in `🦀️shaders.rs` (the only file we touched there); they are spread across
`🎬️scene` (269), `draw.rs` (248), `paint.rs` (174), `events.rs` (132), `widgets.rs` (104) and the rest
of the engine, and classify as the asyncify signature — 462 × E0308 mismatched types, 110 × "no
method found for **opaque type**", 107 × "no field on type", "is not an iterator". That is the
U-program's incomplete `.await` insertion, identical to what broke `semio-framework-number` and
`semio-framework-actor`.

**This means the current pixel path does not compile at all**, independently of the shader corruption
that would have stopped it at pipeline creation anyway. The architecture's premise — that the
committed wgpu-engine path is not a working baseline to preserve — is now measured fact, not
inference.

Known-red named set for U5 is therefore: `semio-framework-number` (lib),
`semio-framework-actor` (lib test), `semio-framework-ui --features wgpu-engine` (lib).

---

## 2026-08-20 — W2 landed and GATED GREEN. Three crates, 228 tests.

Third crate scaffolded by sol as registrar (same pattern, no executor touched the root manifest):
`semio-framework-ui-runtime` at `🖱️ui/🧠️runtime/📦️packages/🦀️rust`, plus its two root `Cargo.toml`
entries. Nine packets landed this wave.

### Packets: `contract-builder` · `runtime-entity` · `runtime-present` · `runtime-gateway` · `render-frame` · `render-dispatch` · `render-text` · `render-surface` (+ 2 reconciliations)

| gate | result |
| --- | --- |
| `cargo test -p semio-framework-ui-contract --all-features` | **73 passed / 0 failed** |
| `cargo test -p semio-framework-ui-runtime` | **34 passed / 0 failed** |
| `cargo test -p semio-framework-ui-render --all-features` | **121 passed / 0 failed** |
| `--all-targets` on runtime and render | **0 errors** each |
| runtime + contract on `wasm32-wasip2` and `wasm32-unknown-unknown` | GREEN |
| `📜️script.ts boundaries` (ui-render) | GREEN |

Evidence: `📝️gate-w2-*.txt`. **228 tests across the three new crates.**

### Seam defects sol resolved at the gate

Two packets that ran concurrently produced a runtime crate with 12 errors, and the render crate had
15. None was visible from inside a single packet — this is what the coordinator gate is for.

1. **`UiApp` did not exist.** My own packet brief invented it in a `Context::defer` signature. Ruled:
   a deferred effect receives `&mut EntityStore`, which is the thing it actually needs. Owning the
   mistake in the reconciliation brief mattered — the executor had correctly refused to invent the
   type and left it unresolved rather than stubbing a duplicate.
2. **`EntityId` was about to be defined twice.** `runtime-present` put it in `🦀️tracking.rs` and said
   so; `runtime-entity` had its own. Ruled one definition in `tracking.rs`, `entity.rs` imports it —
   two identity types that must be kept in sync is a bug generator, not a design.
3. **`PresentCx` wanted `&Context<T>`** — a genuinely wrong expectation, not a mismatch: `Context<T>`
   is a per-lease handle, the wrong shape for read-only multi-entity traversal. Fixed on the
   *consumer's* side to `&EntityStore`, which is the correct direction here.
4. **`ListenerEntry` was private inside a `pub(crate)` field.** Fixed by promoting the type, not by
   exposing its internals — the encapsulation was right.
5. **taffy 0.9 API had been guessed at.** `render-frame` flagged the risk in its own report and was
   right to: in 0.9 `Dimension`/`LengthPercentage` are constructor functions (`length()`, `percent()`,
   `auto()`), not enum variants, and `Style`'s grid field is `GridTemplateComponent<S>` — the missing
   `<String>` was the last error in the crate. Corrected against taffy's vendored source rather than
   inferred from the error text.
6. **`ElementId: Default` was demanded by a derive.** Deliberately refused: a manufactured default id
   could alias a real hashed one, which would surface later as a mysterious element collision. Fixed
   by removing `Default` from the struct that never legitimately needed it.

### Notable engineering judgement inside the packets

- `render-text` **verified the pre-corruption source's parley/swash/fontique calls against the
  vendored crates rather than trusting them**, and established they were already correct — only
  `async fn`/`.await` was corruption. It also replaced a per-character shaping hack with whole-string
  `parley::Layout` shaping and built grapheme-safe cursor/selection on `parley::Cursor` with no new
  dependency.
- `render-dispatch` found that `is_plain_stack_container`'s real rule generalises to an explicit
  `LAYOUT_CONTAINER` opt-in — verified against the ported tests, where plain `Text`/`Button` nodes
  must stay hit targets even with zero bindings. It ported all seven `events.rs` test regions (34
  tests) and flagged its `From<Vec<Hitbox>>` adapter as lossy, since `Hitbox` carries no parent /
  overlay / listener data. **That adapter is a registrar-request to resolve before the cutover** —
  it is the seam between `frame.rs` and `dispatch.rs` and must carry real tree data, not geometry.
- `runtime-entity` caught its own `flush_effects` live-draining its queue, which would let one call
  spin unboundedly if a listener re-notifies; changed to snapshot-then-dispatch so each call is one
  bounded cycle and the EffectStorm budget stays genuinely owned by the transaction loop.
- `render-surface` and `render-frame` independently converged on the same erasure mechanism —
  fn-pointer vtable plus `Box<dyn Any>`, never `dyn Trait` — satisfying U3 without `unsafe`.

### Open items carried to W3

- `runtime-reconcile` and `runtime-transact` are the last two runtime packets; until they land, the
  runtime produces no `UiPatch` and nothing exercises `Present` end to end.
- The `DispatchTree: From<Vec<Hitbox>>` adapter above.
- `contract-builder` asks whether its `BuiltNode` and the runtime's `ComponentTree` should be one
  type. They currently duplicate. Deliberate for now (the dependency runs contract → runtime, never
  back), but worth revisiting once `runtime-reconcile` shows what the conversion actually costs.
- `ImageBuilder::build` panics when neither `.alt()` nor `.decorative()` was called. Accessibility
  enforcement is right; a runtime panic is the wrong mechanism — revisit as a typestate.

---

## 2026-08-20 — the headless runtime is COMPLETE. 251 tests, zero warnings.

`runtime-reconcile` and `runtime-transact` landed, closing the runtime crate.

| crate | tests |
| --- | --- |
| `semio-framework-ui-contract` | **73 passed / 0 failed** |
| `semio-framework-ui-runtime` | **57 passed / 0 failed** |
| `semio-framework-ui-render` | **121 passed / 0 failed** |
| **total** | **251** |

Zero warnings in all three. Both guest targets green for contract and runtime. `--all-targets` clean.

The architecture now runs end to end in the headless direction: entity state → `Present` →
`ComponentTree` → keyed reconciliation → a minimal `UiPatch`, applied through the **same**
`apply_patch` every renderer will use. `runtime-reconcile`'s round-trip property test is the proof —
for a sequence of trees, every emitted patch replayed through the contract's own applier reproduces
the reconciler's snapshot exactly. Producer and consumer cannot drift without that test failing.

### Two defects sol fixed at this gate
- A test helper was monomorphised to one `CommandSink` while a backpressure test needed another —
  made generic.
- `restore_after_lease` was flagged dead. Investigated rather than deleted, because a dead restore
  path would have meant the lease was not actually panic-safe. It is: `update` holds a `PutBack` drop
  guard that restores the value on the normal path and on unwind alike, and the method exists only so
  a test can take a lease, provoke a failure, and assert nothing was lost. Marked `#[cfg(test)]`.

### Contract feedback from the reconciler — act on this before the fleet migrates
`UiPatchOp` has field-targeted setters for component / layout / activity / children, but **not** for
style, accessibility, bindings or menu. A change to any of those four falls back to a whole-node
`Upsert`. That is correct but wasteful, and it will show up as oversized patches exactly where UI
churns most (hover-driven style, focus-driven accessibility). Add the four missing ops in W4, before
33 plugins start generating traffic against the current shape.

### Still open before a cutover can be attempted
1. **`DispatchTree: From<Vec<Hitbox>>` is a lossy adapter.** `Hitbox` carries no parent, overlay or
   listener data, so the seam between `frame.rs` and `dispatch.rs` currently cannot carry a real
   dispatch tree. `render-dispatch` flagged it; it must be resolved before input works at all.
2. The four missing patch ops above.
3. `ImageBuilder::build` enforces accessibility with a runtime panic; should be a typestate.
4. `BuiltNode` (contract) and `ComponentTree` (runtime) duplicate a shape. Deliberate — the
   dependency runs contract → runtime and must not reverse — but revisit now that the reconciler
   shows what the conversion actually costs.
