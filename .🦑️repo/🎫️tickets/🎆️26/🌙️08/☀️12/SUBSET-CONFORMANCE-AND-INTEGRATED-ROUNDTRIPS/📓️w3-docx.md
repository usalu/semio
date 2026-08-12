# W3 DOCX Subset Conformance

Generated: 2026-08-12  
Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Artifact: `s.stdio.docx` · Standard: `ecma-376` · Subset: `✳️any` (`*`)

## Summary

`🏅️standards/🔖️ecma-376/🪆️subsets/✳️any` is the **owning reference** for stdio DOCX: engine, demo example, schema, inferences, and IO all live under the subset. Integrated roundtrip harness is wired in the existing demo example test module. Native fixture `example.docx` was regenerated from `encode_docx(demo_docx_snapshot())` (was 0-byte placeholder).

**Verification:** `cargo test -p semio-s-plugin-stdio docx` — **PASS** (62 passed, 1 ignored).

## Owning body (already under subset)

| Concern | Location |
|---------|----------|
| Engine | `🪆️subsets/✳️any/⚙️engine/🦀️component.rs` |
| Demo example | `🪆️subsets/✳️any/📚️examples/🎬️demo/` |
| Schema / mutations / inferences | `🪆️subsets/✳️any/🧬️schema/` |
| IO (zip import/export) | `🪆️subsets/✳️any/🚪️io/` |

## Owning manifest (`🪆️subsets/🔣️component.json`)

- **archetype:** `owning`
- **examples:** `["demo"]`
- **ioFidelity:** `exact`, drops `[]`
- **import / export:** `zip/2.0/*`

## Inline tests

| Test | Location | Harness |
|------|----------|---------|
| `demo_subset_integrated_roundtrip` | `📚️examples/🎬️demo/🦀️component.rs` | `store::os_store::test_support::{SubsetRoundtripSpec, assert_subset_roundtrip, ExampleAsset, IoFidelityClass}` |
| `fixture_honesty_law` | `⚙️engine/🦀️component.rs` | DSL/pack/native byte parity vs `demo_docx_snapshot()` |

`DocxAnyRoundtrip` uses `decode_docx` / `encode_docx` at `Exact` fidelity on `NATIVE_BYTES` from `example.docx`.

## Fixture fix

| Asset | Before | After |
|-------|--------|-------|
| `🖼️assets/example.docx` | 0 bytes (P0 blocker) | 1648 bytes — genuine `encode_docx(demo_docx_snapshot())` OPC/ZIP |

Populated from failing `fixture_honesty_law` assertion output (left side of `encode_docx(demo)` vs empty right). Ignored helper `zzz_write_native_docx_fixture` remains for future regeneration.

## Glue / path fixes

- Engine `include_str!` / `include_bytes!`: `../../../📚️examples` → `../📚️examples` after subset-local examples
- `📦️glue.rs`: docx `any` engine + `demo` example mounted under `standards::v_ecma_376::subsets::any`

## Compile glue (repo-wide)

- `derive_artifact_facets!` macro: `$crate::NoChildren` → `$crate::app::NoChildren` in `semio-framework-plugin` (required for clean rebuild; `NoChildren` lives in `app` module)

## Verification

```bash
export PATH=/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio docx
```

**Result (2026-08-12):** `test result: ok. 62 passed; 0 failed; 1 ignored`  
Log: `scratch-w3-docx-verify2.txt`
