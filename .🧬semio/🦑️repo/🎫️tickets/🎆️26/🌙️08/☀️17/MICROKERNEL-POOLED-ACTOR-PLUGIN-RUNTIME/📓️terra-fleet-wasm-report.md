# 📓️ terra-fleet-wasm — report

**Verdict: TOTAL, upstream, out-of-scope blocker. Zero fleet crates could be built. Zero production
edits made this packet** (read-only `cargo build`/`cargo test`/`git`/`grep` only, all outside
`✏️s/🔌️plugins/**` since nothing inside it was ever reached). Not caused by any packet on this
ticket — root cause is a **live, uncommitted, cross-ticket migration**
(`26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet `sdk-flip`) that currently leaves
`semio-framework-plugin` (the guest SDK **every** fleet crate depends on) unable to compile, on
**any** target, native or wasip2.

Per this ticket's own standing rule: *"If a fix needs a file outside your scope, STOP and report
it."* This report is that stop.

---

## 0. Why this makes the per-crate table trivial rather than useless

`cargo build -p <fleet-crate> --target wasm32-wasip2` compiles `semio-framework-plugin` **first**,
as a dependency, before it ever reaches a single file inside `✏️s/🔌️plugins/**`. That crate does
not compile right now — reproduced independently 3 times below, on 3 differently-shaped fleet
crates (small / large / extension), with two **different** concrete error sets surfacing depending
on cargo's job-scheduling race (both are real, both are outside my `path_scope`, neither is
mine to fix):

| root cause | where | errors | in my `path_scope`? |
|---|---|---:|---|
| **(A)** `🔌️plugin/🦀️component.rs` already imports `semio_framework_ui_contract` / `semio_framework_ui_runtime` (peer's in-flight source edit) but `🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (registrar-only) was never updated to depend on either — **E0432 unresolved import** | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` | 4–6 | **No** |
| **(B)** that same Cargo.toml still carries the OLD, not-yet-dropped `ui_wgpu = { …, package = "semio-framework-ui", features = ["wgpu"] }` dependency (unconditional, not target- or feature-gated) — and `semio-framework-ui`'s legacy `🎯️targets/🧊️wgpu/**` + `🎬️scene/🦀️component.rs` carry **846** pre-existing async-codemod-residue errors (`impl Future<Output = Vec3>` missing `.await`, the exact shape this ticket's own `gate-3d`/`number-green` packets already fixed in sibling crates — this crate has evidently not been picked up yet) | `🧰️framework/🔨️modules/🖱️ui/**` | 846 | **No** |

The SDK's own source comment (`🔌️plugin/🦀️component.rs:181-182`) already names the fix and flags it
as a pending registrar action: *"registrar-request: drop it \[`ui_wgpu`\] from this crate's
`Cargo.toml`, add `semio-framework-ui-contract` and `semio-framework-ui-runtime` in its place"* —
this is not a surprise to that packet, it is a known, self-documented pending dependency swap that
has not landed yet.

**Every one of the 58 in-scope fleet crates (59 declaring `component-guest`, minus `🗄️stdio`,
excluded per brief) depends on `semio-framework-plugin` to build a wasip2 component. This is total
and structural, not crate-specific — confirmed by hitting the identical wall from 3 different
dependency shapes.**

---

## 1. Reproduction — 3 representative samples, real exit codes, own target dir

Target dir: `…/scratchpad/target-wasm` (reused, per this ticket's env rules).

### 1a. Small plugin — `semio-s-plugin-demonstrator` (2,412 LOC, smallest fleet crate besides stdio)

```
cargo build -p semio-s-plugin-demonstrator --target wasm32-wasip2 --release --message-format=short
```
`EXIT: 101` — fails inside dependency **(B)**: `error: could not compile \`semio-framework-ui\` (lib)
due to 846 previous errors`. Sample of the shape (all 846 follow this pattern, `Vec3`/`f32`/`Ui`/
`Shell`/`Mesh3d` methods called on `impl Future<Output = T>`):
```
🖱️ui/🎯️targets/🧊️wgpu/../../../../🎬️scene/🦀️component.rs:795:32: error[E0599]: no method named
`normalize` found for opaque type `impl Future<Output = Vec3>` — method not found in
`impl Future<Output = Vec3>`
```
(Note: `demonstrator` also directly `Cargo`-depends on `semio-s-plugin-stdio` as a library —
explicitly out of my scope per the brief; not reached, since the build dies further upstream first.)

### 1b. Extension crate — `semio-s-plugin-flow-extension-bim` (828 LOC)

```
cargo build -p semio-s-plugin-flow-extension-bim --target wasm32-wasip2 --release \
  --features component-guest --message-format=short
```
(Extension crates, unlike plain plugin crates, DO declare their own `component-guest` feature —
confirmed by reading the manifest first, per W4 rule 1; `default = ["component-guest"]`.)
`EXIT: 101` — **identical** failure: `semio-framework-ui` (lib), 846 previous errors, same error
set byte-for-byte. This crate has no UI/wgpu dependency of its own at all — it only reaches the
blocker transitively through `semio-framework-plugin`'s `ui_wgpu` dependency edge, which is the
proof this is structural, not a bug in the fleet crate.

### 1c. Large plugin — `semio-s-plugin-norm` (90,277 LOC, largest fleet crate besides stdio)

```
cargo build -p semio-s-plugin-norm --target wasm32-wasip2 --release --message-format=short
```
`EXIT: 101` — fails inside dependency **(A)** this time (cargo's parallel job scheduling raced the
other way — `semio-framework-plugin`'s own local errors surfaced before `semio-framework-ui`'s did):
```
error: could not compile `semio-framework-plugin` (lib) due to 4 previous errors; 1 warning emitted
🔌️plugin/🦀️component.rs:189:9: error[E0432]: unresolved import `semio_framework_ui_contract`
🔌️plugin/🦀️component.rs:190:9: error[E0432]: unresolved import `semio_framework_ui_contract`
🔌️plugin/🦀️component.rs:191:9: error[E0432]: unresolved import `semio_framework_ui_runtime`
```
Never reaches a single file inside `✏️s/🔌️plugins/🕕️norm/**`.

**Per R21 (a negative needs independent reproduction): both failure modes reproduced across 3
structurally different crates (small/large/extension, one with no UI dep of its own at all) — the
blocker is airtight and universal across the fleet, so building the remaining 55 crates one-by-one
would only re-demonstrate the identical wall 55 more times. Not run, for that reason, stated
honestly rather than padded with a table of 58 identical "BLOCKED" rows obtained one at a time.**

---

## 2. Per-crate table

All 58 in-scope crates (`component-guest`-declaring, excluding `🗄️stdio` per brief) are in the
**same** state: **BLOCKED — cannot build, upstream SDK does not compile.** 3 verified directly
(§1); the remaining 55 are inferred from the proof in §0/§1 that the blocker sits entirely upstream
of any fleet crate's own files, not from assumption. Full name list (63 total minus the 4
non-component libraries luna's inventory already excluded, minus `🗄️stdio`):

```
semio-s-plugin-animate, architect, block, cad, cad-aec-building, cad-aec-building-energy,
cad-aec-building-structure, cad-spatial-shape, dag, demonstrator*, draw, energy, fem, flow,
flow-extension-bim*, flow-extension-brep, flow-extension-dictionary, flow-extension-draw,
flow-extension-list, flow-extension-logic, flow-extension-math, flow-extension-primitive, forms,
gis, imperative, imperative-control, imperative-effect, imperative-logic, imperative-math,
imperative-text, layout, lowpoly, mathematical, norm*, playbook, playbook-procedural, procedural,
process, process-concrete, process-metal, process-robotic, process-wood, puzzle, raster,
reasoning-mindmap, remodel, sequence, shooting, sourcing, sourcing-beams, sourcing-slabs,
sourcing-windows, space, trinity, vcs, writer
```
(`*` = directly verified this packet, §1.)

Excluded from scope, not measured: `🗄️stdio` (per brief). Excluded as non-component (luna's
inventory, re-confirmed by grep, unchanged): `draw-fsm`, `draw-fsm-macros`, `trinity-jack-shell`,
`trinity-jack-lsp` (0/4 declare `component-guest`, correctly — pure libraries/proc-macro).

---

## 3. async-lift verification (task item 3)

**Not possible this packet — zero fleet `.wasm` artifacts exist to inspect.** No component built,
so there is nothing to run `strings <file>.wasm | grep async-lift` against. The one artifact on
this ticket independently verified as genuinely async-lifted (`semio-framework-os-scale-fixture`,
all 7 exports `[async-lift]`, `world-collapse` packet + sol's re-derivation) is **outside**
`✏️s/🔌️plugins/**` — a fixture, not a fleet crate — and unaffected by this blocker since it does not
depend on `semio-framework-plugin`'s `ui_wgpu` edge the same way; it remains the only verified
evidence of the ABI's async-ness until the SDK blocker clears and a real fleet component can be
built and inspected.

---

## 4. Regression baseline — re-verified, with an honest new finding

Zero edits made by this packet, so nothing *I* changed could regress the baseline. But the ticket's
own rule ("never claim a test passed without pasting output") requires reporting what is
**actually** true right now, not what the brief's "CURRENT VERIFIED STATE" section assumed (that
section is several hours stale — it predates the live peer migration found here):

| check | required baseline | measured NOW | ✓/✗ |
|---|---|---|---|
| `cargo test -p semio-framework-os-kernel --lib` | 779 / 0 | **779 passed / 0 failed** | ✅ unmoved |
| `cargo test -p semio-framework-os-kernel-db --lib` | 424 / 0 | **424 passed / 0 failed** | ✅ unmoved |
| `cargo test -p semio-framework-plugin-host --lib` | 125 / 0 / 1 ignored | **could not compile** | ❌ **NEW break, not this packet** |
| `cargo check -p semio-framework-plugin --lib` (native) | EXIT 0 | **EXIT 101, 6 errors** | ❌ **NEW break, not this packet** |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | EXIT 0 | **EXIT 101** (§0/§1) | ❌ **NEW break, not this packet** |

`plugin-host`'s break is a **third, independent** instance of the same root migration: `EE0004
non-exhaustive patterns` at `🔌️plugin/🖥️host/🦀️component.rs:1644` — the peer ticket added an
`Event::UiIntent { .. }` variant to the shared kernel `Event` enum
(`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:818`, live-uncommitted, `git status` confirms `M`)
without updating every match site. `🔌️plugin/🖥️host/🦀️component.rs` itself shows `MM` (staged AND
unstaged changes) in `git status` — actively being edited right now, outside my `path_scope`
(`🔌️plugin/🖥️host/**` is SDK/host, not `✏️s/🔌️plugins/**`).

**os-kernel and os-kernel-db are genuinely unmoved (779/424, byte-for-byte).** `plugin-host` and the
SDK's guest-side compile gates are red **right now**, from a cause with git-status evidence
(uncommitted, in-progress edits, one file touched as recently as 13:12 today) that predates and is
unrelated to this packet's existence.

---

## 5. Provenance — why this is not stdio-finish's already-reported blocker

`stdio-finish` (12:44 today) reported a **different, narrower** instance of the same peer ticket's
migration: an unresolved `semio_framework_ui_contract` import in `🎠️kernel/🦀️component.rs` itself,
blocking `semio-framework` (the facade crate). That specific one **has since been fixed** — the
registrar-only `Cargo.toml`s for the facade, `semio-framework-graph`, and `semio-s-plugin-stdio`
now all carry a `semio-framework-ui-contract` dependency line (verified this packet, `git status`
shows all 3 as `M`, live). `semio-framework --lib` compiles clean now (confirmed, §4 above via the
downstream os-kernel/kernel-db tests that depend on it).

**What stdio-finish did not (and could not, from its `path_scope`) see: the migration continued
past the facade into `🔌️plugin/**` itself** (the guest SDK, `🔌️plugin/🖥️host/**`, and
`🧰️framework/🔨️modules/🖱️ui/**`) and is mid-flight there right now, per the fresh `MM`/`M` git
status and 13:08-13:12 mtimes measured this packet. This is a new, later, more severe instance of
the same underlying cross-ticket contention — worth its own report because it blocks a different
(and for this ticket, the critical) part of the tree: the entire fleet build.

---

## 6. Disk (task item 4)

```
df -h /System/Volumes/Data
```
`926Gi total, 593Gi used, 293Gi avail, 67% capacity` — **not** the 95%-full state the brief's
environment note described; that figure is stale for this volume as observed now. `target-wasm`
(reused per this ticket's env rules) grew **3.1G → 4.3G** this packet, from 3 failed compile
attempts against the 846-error `semio-framework-ui` crate (type-check-only failures — no `.wasm`
codegen reached, confirmed: only the pre-existing `semio_framework_os_scale_fixture.wasm` from
`world-collapse` exists under `target-wasm/wasm32-wasip2/release/`, no new fleet artifacts). Left
`target-wasm` as-is rather than pruning its incremental cache — the dir is explicitly marked
"reuse" for this ticket and the next packet to retry the fleet build will need the same dependency
graph recompiled regardless; deleting it now would only cost that packet time for no space benefit
given the healthy headroom.

---

## 7. What unblocks this (not mine to do — reporting for the coordinator/registrar)

Either of these clears the wall (not both required):
1. **Registrar action** on `🔌️plugin/📦️packages/🦀️rust/Cargo.toml` (registrar-only file): drop
   `ui_wgpu = { …, package = "semio-framework-ui", features = ["wgpu"] }`, add
   `semio-framework-ui-contract` + `semio-framework-ui-runtime` — exactly what the SDK's own source
   comment already requests. This is the `sdk-flip` packet's own stated intent; it just hasn't
   landed in the manifest yet.
2. **A dedicated packet fixing `semio-framework-ui`'s 846-error residue** in
   `🎯️targets/🧊️wgpu/**` + `🎬️scene/🦀️component.rs` — same shape, same fix pattern as this ticket's
   own `gate-3d`/`number-green` precedents (`Vec3`/`f32` methods called on un-awaited futures). This
   crate is outside `🧰️framework/🔨️modules/🎭️actor/`-style clean boundaries and does not appear
   claimed by either ticket as of this writing.
3. Separately, `🔌️plugin/🖥️host/🦀️component.rs:1644`'s new `Event::UiIntent` non-exhaustive match
   needs a match arm — small, but also outside `path_scope`.

None of the three are files I can touch under this packet's `path_scope`
(`✏️s/🔌️plugins/** EXCEPT 🗄️stdio`). **Re-run this packet once any one of the above lands** — the
fleet crates themselves showed zero world-collapse-specific breakage in the static inventory
(`📓️luna-fleet-wasm-readiness.md`), so once the SDK compiles again the actual fleet sweep should be
fast.

---

## Files touched

**None** inside `✏️s/🔌️plugins/**` — nothing was reachable. Read-only commands only
(`cargo build`/`cargo test`/`cargo check`, `git status`/`git diff`/`git log`, `grep`, `stat`, `du`,
`df`), all logged above with real exit codes. Full raw build logs kept in this ticket folder:
`terra-fleet-wasm-demonstrator-build.txt`, `terra-fleet-wasm-bim-build.txt`,
`terra-fleet-wasm-norm-build.txt`.
