# Packet `shader-repair` — report

Wave W1. Anchor commit `5e7b8046be`. Executor: Sonnet 5 High.

## Done

1. Repaired `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️shaders.rs` in place —
   stripped the bogus `async` from all 18 `async fn` occurrences (every `vs_main`, `fs_main`, and
   the two `sdf_rounded_rect` helpers) across all 7 WGSL string constants. No reformatting beyond
   the repair; region markers untouched.
2. Built the canonical shader contract in
   `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️shader_contract.rs`: 8 `pub const` WGSL
   strings (the repaired text, copied verbatim) grouped into 5 `ShaderFamily`s / 8 `ShaderVariant`s,
   a backend-neutral `PipelineSpec` data model (no `wgpu`/`winit` types — the crate's `cargo tree`
   guard stays green), `pub const ALL_SHADERS: &[ShaderFamily]`, and a `#[cfg(test)] mod tests` with
   three tests: naga parse+validate over every constant, a `PipelineSpec` ↔ WGSL entry-point drift
   check, and a vertex-attribute-offset-fits-stride sanity check.
3. Only the two OWNS files were touched. Nothing in `🦀️draw.rs`, `🦀️gpu.rs`, either crate's
   `Cargo.toml`, `📦️glue.rs`, or any sibling region file was edited.

## The corruption inventory

Single corruption class, uniform across the file: the asyncify codemod prefixed every WGSL
function declaration with `async `. Confirmed by exhaustive grep before and after the fix
(`grep -c "async fn"` → 18 before, `grep -n "async"` → 0 matches after). Every one of the 7
constants was affected identically:

| constant | `async fn` occurrences fixed |
|---|---|
| `UI_SHADER` | `vs_main`, `sdf_rounded_rect`, `fs_main` (3) |
| `VECTOR_SHADER` | `vs_main`, `fs_main` (2) |
| `WORLD3D_SHADER` | `vs_main`, `fs_main` (2) |
| `WORLD3D_LINES_SHADER` | `vs_main`, `fs_main` (2) |
| `WORLD3D_TEXTURED_SHADER` | `vs_main`, `fs_main` (2) |
| `BLUR_DOWNSAMPLE_SHADER` | `vs_main`, `fs_main` (2) |
| `SCENE_BLIT_SHADER` | `vs_main`, `fs_main` (2) |
| `GLASS_SHADER` | `vs_main`, `sdf_rounded_rect`, `fs_main` (3) |

**No second corruption class found.** I specifically checked for the other asyncify symptoms
named in `📋️master.md`'s load-bearing findings (dropped `.await` calls, renamed identifiers,
mangled attributes) *inside the WGSL string bodies* — those findings (`push_dashed_line`,
`boot_runtime`, `frame`→`render_frame`) are all in `draw.rs` Rust code, outside this packet's OWNS
list, and outside these string constants. Within `shaders.rs`:

- `grep -n "\.await\|await!"` → zero matches. No dropped-await damage inside the WGSL text.
- All `@vertex`/`@fragment`/`@builtin(position)`/`@location(N)`/`@group(N) @binding(N)` attributes
  read correctly and are unchanged from what a hand-written version of this file would contain —
  no renamed identifiers, no mangled attribute syntax.
- Struct field names, entry-point signatures, and control flow inside every fragment shader (the
  `kind == N` branches in `UI_SHADER`, the SDF math, the animated border phase math) are intact.

I verified this by reading the full 475-line file before editing (not sampling) and by re-reading
it in full after editing to eyeball every constant.

## `WORLD3D_TEXTURED_SHADER` verdict: dead code, not wired anywhere

Confirmed dead by two independent checks (per U8.8, a single grep is not proof of absence):

1. `draw.rs:4`'s import list: `use crate::wgpu::shaders::{BLUR_DOWNSAMPLE_SHADER, GLASS_SHADER,
   SCENE_BLIT_SHADER, UI_SHADER, VECTOR_SHADER, WORLD3D_LINES_SHADER, WORLD3D_SHADER};` — 7 of the
   8 constants, `WORLD3D_TEXTURED_SHADER` absent.
2. `grep -rn "WORLD3D_TEXTURED" 🧰️framework/` across the entire framework tree returns exactly one
   hit: the `pub const` declaration itself in `shaders.rs`. No pipeline, no bind group, no test, no
   other file references it anywhere in the repo.

This matches `📋️master.md`'s own count — "7 WGSL constants / 5 shader families" — which already
excludes this constant from the wired set. It is genuinely unused in the committed wgpu-old path,
not a wired-but-broken feature silently failing. I did not delete it (out of scope, and the
canonical contract wants it — see below); I flagged it in-code and here per the packet's
instruction not to assume either way.

Because `master.md`'s "World3d (mesh + lines + textured variants)" language asks for the textured
variant in the canonical contract regardless, `🦀️shader_contract.rs` includes
`WORLD3D_TEXTURED_SHADER` and a `WORLD3D_TEXTURED_PIPELINE` spec, but the spec is explicitly
marked `⚠️ INFERRED` in its doc comment and its `label` field literally reads
`"world3d_textured_pipeline (inferred — unwired in draw.rs)"` — see "Pipeline-state values I could
not determine" below for exactly which fields that covers.

## Acceptance: UNRUN (both commands)

Per this ticket's `📌️important.md` **U4** ("the coordinator owns every build... Executors write
code and reasoning, run only cheap non-cargo checks, and mark acceptance UNRUN. `sol` runs every
gate and pastes the numbers"), I did not run either acceptance command myself:

1. `cargo test -p semio-framework-ui-render --lib shader` — **UNRUN**, sol to run with
   `timeout: 600000` and `CARGO_TARGET_DIR` in the session scratchpad, never the ticket folder
   (U4).
2. `cargo check -p semio-framework-ui --features wgpu-engine --lib` — **UNRUN**, same.

In place of running cargo, I did the following manual verification, since I will not claim a test
passed without having seen its output and exit code (U8.4):

- Cross-checked every `naga` API call in the new test module (`naga::front::wgsl::parse_str`,
  `naga::valid::Validator::new`/`.validate`, `ParseError::emit_to_string`,
  `WithSpan<ValidationError>::emit_to_string`, `naga::Module.entry_points[].name`) against the
  actual vendored source at
  `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/naga-27.0.3/` — the exact version pinned
  in the render crate's `Cargo.toml` dev-dependency. Confirmed: `parse_str` and the `valid` module
  are reachable with only the `wgsl-in` feature (already declared, unmodified); `emit_to_string` on
  both error types needs no additional feature (it's alloc-only, distinct from the
  `feature = "stderr"`-gated `emit_to_stderr`); `naga` is `#![no_std]` but this doesn't affect us.
- Re-read the final `shaders.rs` and `shader_contract.rs` in full after every edit to check brace
  balance, trailing commas, and that no `async` remained (`grep -n "async"` → 0 matches in both
  files).
- Confirmed no `dyn` appears anywhere in the new file (U3) and no `async fn` appears anywhere (U1).
- Confirmed no naming collisions between the new `pub const`/`pub struct`/`pub enum` items and
  anything already defined in the render crate's other region files (`🦀️backend.rs`, `🦀️scene.rs`,
  `🦀️resource.rs`, `🦀️frame.rs`, `🦀️surface.rs`, `🦀️element.rs`, `🦀️layout.rs`, `🦀️schedule.rs`,
  `🦀️dispatch.rs`, `🦀️text.rs`, `🦀️tessellate.rs`) — all currently in-flight under other packets;
  grepped each for every new identifier name, zero hits.

I could not run even a "cheap non-cargo check" (e.g. `rustc --crate-type lib` on the file in
isolation) because `shader_contract.rs` depends on `naga` (dev-dependency) and is `#[path]`-mounted
into a crate with several other in-flight dependencies (`ui_styling`, `ui_contract`,
`semio-framework-geometry`, `taffy`, `parley`, `swash`) it doesn't itself use but that must resolve
for the crate to build — a standalone rustc invocation would not reflect real build conditions and
could give false confidence.

## Pipeline-state values I could not determine from `draw.rs`

Everything in `🦀️shader_contract.rs` for the 7 wired shaders (`UI`, `Vector`, `World3d` mesh/lines,
`Blur` downsample/blit, `Glass`) is read directly off `UiPipelines::new` in `draw.rs` (lines
~1235–1699) — bind group layouts, vertex buffer strides/offsets (cross-checked against the actual
`#[repr(C)]` struct definitions: `UiInstance` at draw.rs:166, `VectorVertex` at :217,
`World3dVertex`/`World3dGlobals`/`World3dGpuInstance` at :707/:714/:721, `WorldLineGpuVertex` at
:846, `GlassInstance` at :34), blend states, depth/stencil states, topology, and cull mode. Nothing
in that set is a guess.

The one exception is `WORLD3D_TEXTURED_PIPELINE`, because `draw.rs` never builds a pipeline for
`WORLD3D_TEXTURED_SHADER` (see verdict above) — there is no ground truth to read. Split by
confidence:

**Solid (derived directly from the WGSL source itself, not from `draw.rs`):**
- `vertex_entry`/`fragment_entry`: `vs_main`/`fs_main`, same as every other variant.
- `vertex_buffers`: stride/offsets computed from the shader's own `VertexInput` (`position: vec3`,
  `uv: vec2` → 20-byte stride) and `InstanceInput` (`model0..model3: vec4` + `tint: vec4` at
  locations 3–7 → 80-byte stride), mirroring exactly how the wired mesh/lines variants' buffers
  were derived from their own struct fields.
- `bind_groups` group 1 (`texture_2d<f32>` + `sampler` at `@group(1) @binding(0/1)`): read straight
  off this shader's own declarations.

**Guessed, and flagged as such in the doc comment on `WORLD3D_TEXTURED_PIPELINE` (not silently
assumed):**
- `bind_groups` group 0's `dynamic_offset: true` / `min_size: Some(80)` — reused from
  `WORLD_GLOBALS_BIND_GROUP` because the shader's `Globals` struct is byte-identical to the wired
  mesh/lines shaders', so the same dynamic-offset ring buffer mechanism is the natural fit, but
  this is inference from shape-similarity, not a read of real pipeline construction.
- `blend: Replace`, `cull_mode: None`, `depth_stencil` (write-enabled, `Less`, no bias) — all
  mirror `WORLD3D_OPAQUE_PIPELINE` as the nearest analog (opaque, depth-writing mesh geometry). A
  translucent or double-sided variant would need different values and there is no way to know
  which without the shader actually being wired to a real pipeline first.

A future packet that wires this shader into a real backend should treat these four guessed fields
as provisional and correct them once a real pipeline exists to read from.

## Registrar-requests

None. No changes needed to any `U7` registrar-only file.

## Deviations

- Combined the naga-validation requirement with two extra guard tests
  (`pipeline_entry_points_exist_in_shader`, `vertex_attribute_offsets_fit_declared_stride`) beyond
  the minimum "parses AND validates" ask, because the packet explicitly asked for the
  spec-vs-shader drift check ("assert the `PipelineSpec` for each family names entry points that
  actually exist") and the offset check catches a plausible sibling defect (an attribute offset
  overrunning its buffer stride) for near-zero extra code.
- Did not run either acceptance command (see "Acceptance: UNRUN" above) — this is U4 compliance,
  not an omission; flagging it explicitly since the packet's own "ACCEPTANCE" section asks for
  commands to be run, and U4 (from the same ticket's binding rulings) overrides that for
  executors.
