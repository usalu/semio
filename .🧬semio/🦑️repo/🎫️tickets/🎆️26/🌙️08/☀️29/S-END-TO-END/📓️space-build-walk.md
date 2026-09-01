# 🪐️ Walking `semio-s-plugin-space` to a `wasm32-wasip2` build

Once `semio-s-plugin-stdio` went green (see `📓️serde-restore-wave.md`), the `space` build exposed a
second, longer chain. Each fix moved the break to the next crate down, so the useful record is the
walk itself and *why* each crate broke — several were not what the error first suggested.

## The walk

| Crate | Errors | Root cause |
|---|---|---|
| `semio-framework-os` (host) | 128 → 7 | **Reachability masking.** A single unresolved import — `pub use store::sync as store_sync` in the host's glue, re-exported without the target guard that the module it re-exports already carries — hid 121 unrelated diagnostics. `store::sync` is gated off for `wasm32-wasip2` because the sync actor's transport is `tokio`/`tokio-tungstenite`, which a WASI-P2 guest never links. Aligning the guard on the re-export dropped 128 to 7 in one step. |
| `semio-framework-os-infinite` | 7 → 0 | `🗺️surface/🏔️terrain/🦀️component.rs` is path-mounted into **two** crates (`surface` mounts it normally; `infinite` mounts it by `#[path]` because surface depends on infinite and a real edge would be a cycle). It decodes Terrarium elevation PNGs via `image`, which only `surface` declared. A shared source file needs its dependencies declared in every crate that mounts it. |
| `semio-framework-os` (host body) | 127 | The real work, now unmasked: stale `.await`/missing `.await` from the async-convention sweep, plus half-migrated serde→`ToValue` bounds. Fixed by two agents (host core, host `space` module). |
| `semio-framework-os-kernel` | 20 → 0 | A sweep had added `#[derive(ToValue, FromValue)]` + `#[value(...)]` to three space-history mutation files whose own comments, three lines above, say the derives are impossible there (orphan rule — `SpaceAlternative`/`SpaceCheckpoint` embed foreign types) and which **already had** hand-written impls. The derives were both unresolvable and duplicate. Reverted. |
| `semio-framework-ui` | 10 → 0 | `ActionDescriptor.args: Option<DslValue>` on a serde-deriving struct. Gave `DslValue` `Serialize`/`Deserialize` by delegating to the `From` conversions it already has in both directions — that makes the encoding identical to `serde_json::Value`'s *by construction*, which is the property that matters since both share a wire. |
| `semio-framework` | 2 → 0 | `dsl::to_dsl_value`'s bound moved from `Serialize` to `ToValue`. Routed `optional_json_to_dsl` through the existing `From<serde_json::Value>`, and kept `ManifestField::default_value(impl Serialize)`'s public signature by bridging through `serde_json` rather than narrowing the API. |
| `semio-framework-plugin` | 117 → 48 → 0 | A peer was mid-edit adding `#[derive(ToValue, FromValue)]` across `🔌️plugin/🦀️component.rs` without importing the macros. Fixed per inline module — see the note below on why this needed three attempts. Then two agents cleared the remaining 48 (E0119 duplicate impls, E0277 missing impls). |
| `semio-framework-replication` | 10 → 0 | An agent's derives landed in `⚠️diagnostic/🦀️component.rs` + `📍️span/🦀️component.rs`, both path-mounted into `replication` — **the one crate that cannot depend on the derive macro's target crate**, exactly as the existing `FaultCode`/`Severity` notes in that same file explain. Replaced with hand-written impls for `FaultOrigin`, `FaultScope`, `FaultCause`, `Fault`, `TextSpan`, mirroring each type's serde attributes. |
| `semio-s-plugin-stdio` (regression) | 34 → 0 | A peer moved the 34 viewer/editor window templates from `pack::json!` to `ToValue::to_value`, so `MeshData` needed the trait, not just the `From` impl added earlier. `pack` now re-exports `protocol::value` so `mesh-engine` can implement a foreign trait for its own local type without a second dependency edge; the impl delegates to the `From` impl so the two paths cannot drift. |

## Two things worth remembering

**Brace counting is unreliable in this repo.** Inserting a `use` into each inline module of
`🔌️plugin/🦀️component.rs` failed twice: once because the import landed *before* a module's inner
`//!` doc comments (which must come first — E0753), and once because a brace-depth walk ran off the
end of the file at a raw string containing `{{`/`}}` (line 29710), silently skipping every module
after it. What worked was driving off the compiler's own error line numbers and finding each
enclosing module by **indentation** rather than by counting braces.

**A nested module's import does not cover its parent.** A "does this module already import it?" check
must exclude nested `mod` blocks — a `use` inside a child does not bring a derive macro into the
parent's scope, and counting it silently skips the parent that actually needs it.
