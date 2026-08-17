# W1 mechanism design — how "artifacts only" becomes the *only* way

Authority: UCAS ceded registration consolidation to APA in full on 2026-08-12 (their `declare_artifact!`/`plugin!` macro plan is deleted). This document is the design W1 agents execute from. Exhaustive symbol lists come from `📓️w0-d-sdk-surface.md`; this file defines the *shape* and the *rules*.

## The problem, stated precisely

A plugin can today reach process-global mutable registries by calling free functions:

```rust
pub fn register_lowpoly_exports() {                       // ✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs
    crate::artifacts::lowpoly::engine::register();
    semio_framework_os::register_mesh_exporter("3d.lowpoly", …);
    semio_framework_os::register_mesh_exporter("3d.mesh", …);   // a kind lowpoly does not own
}
```

Three separate things are wrong and each needs its own countermeasure:

| wrong thing | countermeasure |
|---|---|
| registration is an arbitrary *call*, so it can happen anywhere, in any order, doing anything | **M1** make it *data* the framework walks |
| the registry functions are `pub` on crates plugins depend on | **M2** require a token plugins cannot mint + **M3** cut the dependency |
| nothing checks that a plugin owns the kind it registers | **M1** ownership derives from the declaration's `kind` |

## Defence in depth — four layers, each independently sufficient to catch a regression

1. **Structural (compile-time).** Registration functions take `&Registrar`. Plugin code has no way to obtain one.
2. **Graph (build-time).** Plugin crates may not depend on any crate that exposes a `Registrar` constructor. Enforced by the Cargo dependency allowlist policy.
3. **Lint (gate-time).** Policy rules ban `register_*(`, `.setup(`, and `Registrar` in `✏️s/🔌️plugins/**`.
4. **ABI (runtime).** WASI is already sealed (`WasiCtxBuilder::new().build()`, no preopens); every WIT host import becomes capability-gated.

Layer 1 alone would be defeated by a plugin that depends on `semio-framework` directly (some do today — raster, cad). Layer 2 alone would be defeated by a re-export. Together they close.

## M1 — `ArtifactDeclaration`

New region `🔖️ArtifactDeclaration` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`.

An artifact declares, as data, everything it currently registers by calling:

```rust
pub struct ArtifactDeclaration {
    kind: ArtifactKindId,
    schema: &'static ArtifactSchemaDescriptor,
    inferences: &'static [ArtifactInferenceDescriptor],
    composers: &'static [ComposerEntry],
    formats: &'static [FormatDescriptor],
    subset_validators: &'static [SubsetValidatorEntry],
    languages: &'static [LanguageSpec],
    document_codec: Option<DocumentCodecSpec>,
    migrations: &'static [DialectMigrationSpec],
    child_slots: &'static [ChildSlotSpec],      // PRIVATE — no public setter, see below
    link_slots: &'static [LinkSlotSpec],        // PRIVATE — no public setter, see below
    capabilities: &'static [CapabilityRequirement],
}
```

### Composition slots take the *type*, never a list (UCAS review, 2026-08-12)

`child_slots`/`link_slots` must **not** be settable from a hand-written slice. Those tables are already derived: `#[derive(ArtifactSchema)]` emits an `ArtifactCompositionFields` impl by reading the snapshot struct's real `ArtifactChild<T>` / `Vec<ArtifactChild<T>>` / `ArtifactLink` fields, with `#[child(kind = "…")]` supplying the kind. **The struct definition is the truth.** A hand-written list could silently disagree with it — UCAS's `compose_from_children` derivation would read one table while the manifest/UI/policy path read the other, and nothing would catch the drift until a composer built the wrong thing. That is the same defect class as an artifact's identity being spelled six ways, which is what this ticket exists to remove.

So the builder captures the snapshot type once and pulls both tables from the trait:

```rust
ArtifactDeclaration::builder("s.cad.cad")
    .schema(&CAD_SCHEMA)
    .composition::<CadSnapshot>()   // <CadSnapshot as ArtifactCompositionFields>::{child_slots, link_slots}()
```

The fields stay private with **no** public slice setter, so a wrong list is *unwritable* rather than discouraged — the same standard the ownership check applies to dialects, and stronger than any policy rule. If an escape hatch ever proves necessary, `build()` must assert it equals the trait's tables and fail hard on mismatch.

Two mechanical notes before committing to the signature: `ChildSlotSpec`/`LinkSlotSpec` live in `semio-framework-schema` (`🧬️schema:93-140`) — **verify that crate is reachable from `semio-framework-plugin` by compiling, not by reasoning** (the `🚪️io` dual-mount already proved dependency intuitions here are unreliable). And `ChildSlotSpec.kind` is deliberately `&'static str`, not `ArtifactKindId`, so that `semio-framework-schema` need not depend on `semio-framework`; convert at the boundary if the builder wants the newtype.

Built by a typestate/consuming builder mirroring `PluginBuilder`'s style, so a malformed declaration is a compile error rather than a runtime panic. Exact field set is finalized against the registration-function census in `📓️w0-d-sdk-surface.md` §6 — **every** function listed there must have a corresponding declaration field, or the escape hatch survives.

`child_slots`/`link_slots` exist from day one so UCAS's composition spec has a declared home and never needs a second pass through every plugin. Coordinate the types with their `ArtifactCompositionFields` (`🧬️schema:93-140`) — reuse, do not redefine.

`capabilities` moves to the artifact because **IO is artifact-owned**: an artifact that reads assets declares `Asset/Read`, not its plugin. `PluginBuilder::build()` unions them into `PluginManifest.capabilities`. `Plugin::capability()` stays for genuinely plugin-scope rights (backbone storage).

### Plugin-side shape

Each artifact's `⚙️engine/🦀️component.rs` replaces its side-effecting `register()` with a pure data function:

```rust
// before                                        // after
pub fn register() {                              pub fn declaration() -> ArtifactDeclaration {
    crate::artifacts::cad::io_registry::register();   ArtifactDeclaration::builder("s.cad.cad")
    register_artifact_schema();                           .schema(&CAD_SCHEMA)
    register_artifact_inferences();                       .inferences(&CAD_INFERENCES)
    register_pilot_languages();                           .composers(crate::artifacts::cad::io_registry::ENTRIES)
    …register_document_codec_for_app::<CadPlayApp>(…);     .languages(&CAD_LANGUAGES)
}                                                         .document_codec::<CadPlayApp>(CAD_DOCUMENT_SCHEMA)
                                                          .build()
                                                     }
```

and the plugin root loses `.setup` entirely:

```rust
Plugin::builder("cad").label("CAD").version("0.1.0")
    .artifact(crate::artifacts::cad::engine::declaration())
    .register_document_app::<CadPlayApp>(crate::apps::cad::create_cad_app())
    .build()
```

### Builder changes (`🏗️builder/🦀️component.rs`)

- Delete the `setup: Option<fn()>` field, `PluginBuilder::setup()`, and the `if let Some(setup) = self.setup { setup(); }` at `:143-145`. Thread-through in `label()`/`version()` goes too.
- Add `artifacts: Vec<ArtifactDeclaration>` and `pub fn artifact(mut self, decl: ArtifactDeclaration) -> Self` (repeatable, `Ready` state only).
- `build()` mints one `Registrar` and walks the declarations, performing every registration itself, in a **fixed deterministic order** (schema → inferences → formats → subset validators → composers → languages → document codec → migrations). Ordering is currently implicit in call order inside 33 hand-written setup functions; making it explicit and uniform is half the value of this change.
- `build()` **validates ownership**: every dialect/composer entry in a declaration must have `artifact_kind == decl.kind`, and `decl.kind` must be `s.<plugin_id>.<artifact>` matching the builder's `plugin_id`. A plugin registering IO for a kind it does not own is now a hard error. **This single check is what makes the named lowpoly violation impossible rather than merely absent** — and it does structurally what UCAS's W4 is doing by hand: the duplicate kind ids they are cleaning up (`3d.mesh` claimed by both gis and lowpoly, `kit.catalog` claimed by puzzle and three block apps) become *unrepresentable*, so they cannot recur after that cleanup lands.
- `artifact_kind(ArtifactKindSpec)` — **the builder method** is retired now. The `ArtifactKindSpec` **type** is deleted later by UCAS's W6 media cleanup (alongside `OsMediaCapability` and `MediaClass×MediaForm`); retiring the method does not require the type to be gone, so APA does not wait on it. If APA's seal later needs the type gone sooner, UCAS can pull that one deletion out of W6 — it is separable.

## M2 — the `Registrar` seal

```rust
pub struct Registrar(());                        // 🔐️ proof that the framework, not a plugin, is registering

impl Registrar {
    #[doc(hidden)]
    pub fn __framework_internal_mint() -> Self { Registrar(()) }
}
```

Every global registration function gains `&Registrar` as its first parameter — the full list from `📓️w0-d-sdk-surface.md` §6, including at minimum `register_composer_entries`, `set_io_fallback_dispatcher`, `register_subset_validator`, `register_format_descriptors`, `register_artifact_schema_descriptor`, `register_artifact_inference_descriptor`, `register_app_schema_descriptor`, `register_language`, `register_document_codec`, `register_document_codec_for_app`, `register_dialect_migration`.

**Placement.** `Registrar` lives in `🎠️kernel`, which is mounted into `semio-framework`. `🚪️io` is dual-mounted into both `semio-framework` (as `io`) and `semio-framework-os-kernel` (as `os_io`), so a token defined in `🚪️io` would be visible in both crates — but `🎠️kernel` is mounted **only** by `🛂️manifest` inside `semio-framework`, so os-kernel cannot see it. Since os-kernel also hosts registries (`register_document_codec`, `register_language`), W1 must first confirm reachability. **If `🎠️kernel` is not reachable from os-kernel, put `Registrar` in `🚪️io` instead** — the dual mount makes it visible to exactly the two crates that need it, and to no plugin. Decide this with a compile, not by reasoning; record the result in the W1 report.

**Why the token is not security theatre.** It is not, by itself, unforgeable — `__framework_internal_mint` is `pub`. It becomes unforgeable in combination with M3: after the curated re-export list and the Cargo purge, a plugin crate has no dependency edge to any crate exposing it. The token converts "did you remember not to call this?" into "you cannot name this", which is the difference between a convention and a mechanism.

## M3 — SDK sealing

1. **Replace the two glob re-exports** in `🔌️plugin/🦀️component.rs` (`pub use semio_framework::*;` :9645, `pub use ui_wgpu::wgpu::*;` :9654) with an explicit, region-grouped list. The list is *derived*, not invented: scout D produced the deduplicated set of every symbol actually imported from `semio_framework_plugin` across all plugins, with per-symbol usage counts. Re-export exactly that set. Anything used by zero plugins is not re-exported; anything a plugin genuinely needs but that would drag a `Registrar` constructor into scope gets a purpose-built wrapper instead.
2. **Purge `semio-framework-os` from all 31 plugin/extension `Cargo.toml`s.** Data types plugins legitimately need (`DwgDrawing`, `DwgGeometry`, `rasterize_svg_to_png_base64`, mesh types) get explicit re-exports from `semio_framework_plugin`. Per CLAUDE.md: never export API that requires a type from outside the codebase, and re-export explicitly what the client needs.
3. Target end state for a plugin `Cargo.toml`: `semio-framework-plugin`, the os-kernel aliases (`store`/`dsl`/`protocol`/`pack`/`spr`/`vcs`), `semio-framework-3d` where genuinely needed, `serde`. Nothing else framework-side.

## M4 — app purity (framework half)

- Replace `ArtifactApp::seed(&mut ArtifactStore<Snapshot, Mutation>)` (`:4614`) with `fn genesis() -> Vec<Self::Mutation> { Vec::new() }`. `VcsArtifactApp` applies the returned mutations through normal dispatch at construction. This removes the only place an app touches a store directly.
- The Draft lane half is specified separately in `📓️draft-lane-spec.md` (settled with SMO) and executes in W3, per plugin.

## M5 — capability enforcement

- Generalize `HostState::has_backbone_access`/`has_engine_access` (`🔌️plugin/🖥️host/🦀️component.rs:288-295`) into `has_capability(ArtifactKind, Rights)` and gate **every** WIT host import: `read/write-artifact` → Document, `read-asset` → Asset/Read, `write-blob`/`read-blob` → Asset/Write|Read, `network-fetch` → Network, `open-window` → Window, `invoke-action` → Invoke, `io-dialects`/`io-compose` → Document/Read.
- Map every `HostEffect` variant to a `CapabilityRequirement` (proposed mapping in `📓️w0-d-sdk-surface.md` §5); the effect executor checks the grant before executing and returns a `Fault` for an undeclared effect. `HostEffect` is a parallel side-effect channel today — ungated, it makes the capability system decorative.
- Close the `CommandContext` hole recorded in the source ("commands don't yet carry a capability grant model") by threading `granted_capabilities` from `ActionContext` into command dispatch.

⚠️ `🔌️plugin/🖥️host/🦀️component.rs` is **UCAS's** `IoRouter` file — not the same as `💻️os/🖥️host/🦀️component.rs`, which is APA's. M5 needs UCAS's file; negotiate before entering it.

## Ordering and gates

| step | file | blocked on |
|---|---|---|
| M2 `Registrar` type + placement decision | `🎠️kernel` or `🚪️io` | UCAS claim — negotiate |
| M2 `&Registrar` params | `🚪️io`, `🧬️schema`, os-kernel registries | as above |
| M1 declaration + builder | `🔌️plugin/🦀️component.rs`, `🏗️builder/` | **UCAS C1 unfreeze** |
| M3 curated re-exports | `🔌️plugin/🦀️component.rs` | UCAS C1 unfreeze; scout D symbol list |
| M3 Cargo purge | 31 plugin `Cargo.toml`s | per-plugin clearance (SMO + UCAS) |
| M4 `genesis()` | `🔌️plugin/🦀️component.rs` | UCAS C1 unfreeze |
| M5 host gating | `🔌️plugin/🖥️host/` | UCAS negotiation |

Verification for every step: `CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p <crate>` then `cargo test -p <crate> --lib`, with real output pasted into the report. **Never bare cargo.** Plugin-side verification is impossible until UCAS broadcasts "roster frozen" (stdio red ⇒ every plugin red); treat plugin redness as churn, prove no error originates in your own boundary, and report `blocked-churn`.
