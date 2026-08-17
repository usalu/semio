# W3 CAD Subset Conformance

Generated: 2026-08-12  
Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Artifact: `s.cad.cad` · Standard: `1` · Subset: `✳️any` (`*`)

## Summary

`🏅️standards/🔖️1/🪆️subsets/✳️any` is now the **owning reference** for CAD standard 1: engine, examples, schema (snapshot/diff/mutations), inferences, and stdio IO all live under the subset. The owning manifest in `🪆️subsets/🔣️component.json` declares kind, IO fidelity, inferences, partial mutations, dialects, and examples. Inline integrated roundtrip tests were added on the real `demo` DSL fixture.

**Verification:** `cargo test -p semio-s-plugin-cad demo_subset_integrated_roundtrip` **did not pass** — blocked by a repo-wide `semio-framework-plugin` borrow-check error (`E0499` at `dispatch_emit_group`, line 5790). See [Verification](#verification).

## Moves

| Source | Destination | Status |
|--------|-------------|--------|
| `🏅️standards/🔖️1/⚙️engine/` | `🪆️subsets/✳️any/⚙️engine/` | ✅ moved (incl. `📥️geometry-import`, `🔄️transformation`, `🔍️construct`, `🕹️interaction`) |
| `📚️examples/🎬️demo` (artifact level) | `🪆️subsets/✳️any/📚️examples/🎬️demo` | ✅ moved |

Stale duplicate at `🏅️standards/🔖️1/⚙️engine/` removed after copy; standard level now contains only `🪆️subsets/`.

## Owning manifest (`🪆️subsets/🔣️component.json`)

- **kind:** `owning`
- **ioFidelity:** dsl/pack `exact`; json/step/ifc `semantic`; obj/stl/gltf/dwg/png `lossy`
- **ioFidelityDrops:** per-format field drops (references, nodes, geometry slices as applicable)
- **inferences:** `["bounds"]`
- **partialMutations:** 14 slugs (create/delete/rename/move/rotate/scale object, drag, node CRUD, move-reference, replace-references, change-active-model-definition)
- **importDialects / exportDialects:** `s.cad.cad.dsl`, `s.cad.cad.pack`, plus 8 stdio formats (json, obj, stl, step, ifc, gltf, dwg, png)
- **examples:** `["demo"]`

## Inferences + IO (both directions)

Already present under subset; no new codecs required for W3:

| Concern | Location |
|---------|----------|
| DSL snapshot (import/export) | `🧬️schema/📸️snapshot/📝️text/` |
| Pack snapshot (import/export) | `🧬️schema/📸️snapshot/💾️binary/` |
| Stdio import | `🚪️io/📥️import/🧩️deserializers/` (8 formats) |
| Stdio export | `🚪️io/📤️export/🧵️serializers/` (8 formats) |
| Inference `bounds` | `🧬️schema/💡️inferences/📦bounds/` |

## Inline tests (`🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs`)

| Test | Harness |
|------|---------|
| `primary_asset_is_nonempty` | fixture sanity |
| `inference_determinism_law` | inference law |
| `inference_default_law` | inference law |
| `demo_subset_integrated_roundtrip` | `store::test_support::assert_subset_roundtrip::<CadAnyRoundtrip>` |

`CadAnyRoundtrip` uses real `🗣️example.dsl.semio`, pack export/reimport at `Semantic` fidelity, `RenameObject` sample mutation, and DSL validation in `validate_payload`.

## Glue (`📦️glue.rs`) — compile-required path fixes

Outside path scope but required so moved modules resolve:

- Removed standard-level `v1::engine` mount; engine + examples mounted under `standards::v1::subsets::any`
- Shim: `pub mod engine { pub use super::standards::v1::subsets::any::engine::*; }`
- Example `#[path]` targets updated to subset `📚️examples/🎬️demo`

## Path fixes inside artifact tree

- `⚙️engine/🦀️component.rs`, `📥️geometry-import/🦀️component.rs`, `🕹️interaction/🦀️component.rs`: `include_str!` depth +2 for moved engine
- `🧬️schema/📸️snapshot/📝️text/🦀️component.rs`: `CAD_DEFAULT_EXAMPLE_TEXT` → `../../../📚️examples/...`

## Verification

```bash
export PATH=/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH
export RUSTC_WRAPPER=   # disable sccache when disk tight
export CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target-w3-cad"
cargo test -p semio-s-plugin-cad demo_subset_integrated_roundtrip
```

**Result (2026-08-12):** build failed before CAD tests ran.

```
error[E0499]: cannot borrow `self.children` as mutable more than once at a time
    --> 🧰️framework/.../🔌️plugin/🦀️component.rs:5790:41
```

Full log: `scratch-w3-cad-roundtrip.txt`

**Environment notes:**

- Initial run hit `No space left on device`; freed ~1.4 GiB by removing stale ticket `🎯️target*` dirs
- sccache fatal errors when disk tight — use `RUSTC_WRAPPER=` for clean builds

**Re-run after framework fix:**

```bash
cargo test -p semio-s-plugin-cad inference_
cargo test -p semio-s-plugin-cad demo_subset_integrated_roundtrip
```

## Files touched (in-scope + glue)

**Artifact tree (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/`):**

- `🏅️standards/🔖️1/🪆️subsets/🔣️component.json` — owning manifest
- `🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/**` — relocated engine
- `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/**` — relocated demo example + tests
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — example include path
- `🏅️standards/🔖️1/⚙️engine/**` — **removed** (stale duplicate)

**Glue (compile-required):**

- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs`

**Ticket artifacts:**

- `scratch-w3-cad-roundtrip.txt` — cargo log
- `🎯️target-w3-cad/` — partial build output

## Gate

| Check | Status |
|-------|--------|
| Engine under subset | ✅ |
| Examples under subset | ✅ |
| Owning manifest complete | ✅ |
| Inference + IO both directions | ✅ (pre-existing) |
| Integrated roundtrip test authored | ✅ |
| Integrated roundtrip test passing | ❌ blocked by `semio-framework-plugin` E0499 |
