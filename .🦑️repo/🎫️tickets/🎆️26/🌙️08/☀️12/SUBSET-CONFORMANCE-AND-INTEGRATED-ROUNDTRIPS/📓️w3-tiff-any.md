# W3 TIFF Any — Subset Conformance Roundtrip

Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any`

## Archetype & fidelity

| Field | Value |
|-------|-------|
| Archetype | **owning** |
| `IoFidelityClass` | **Exact** (native `example.tiff` via `encode_tiff`/`decode_tiff`) |
| `subset!` macro | not used |

## Changed files

| File | Change |
|------|--------|
| `…/✳️any/📚️examples/🎬️demo/🦀️component.rs` | `TiffAnyRoundtrip` + `demo_subset_integrated_roundtrip` |
| `…/✳️any/⚙️engine/🦀️component.rs` | `fixture_honesty_law` asserts `encode_tiff(demo)` == `example.tiff`; `zzz_write_native_tiff_fixture` (`#[ignore]`) |
| `…/✳️any/📚️examples/🎬️demo/🖼️assets/example.tiff` | populated (168 bytes) from `encode_tiff(demo_tiff_snapshot())` |
| `…/🪆️subsets/🔣️component.json` | owning manifest |
| `📦️packages/🦀️rust/📦️glue.rs` | direct `pub mod engine` mount (E0499 glue fix) |

## Harness wiring

- **Test:** `demo_subset_integrated_roundtrip`
- **Location:** `✳️any/📚️examples/🎬️demo/🦀️component.rs`
- **Native asset:** `NATIVE_BYTES` = `include_bytes!("🖼️assets/example.tiff")`
- **Sample mutation:** `SetTag` on `TAG_IMAGE_WIDTH`

## Commands

```bash
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH"
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio --lib 'artifacts::tiff::standards::v6_0::subsets::any::engine::tests::conformance_laws::zzz_write_native_tiff_fixture' -- --ignored --exact
cargo test -p semio-s-plugin-stdio --lib subset_integrated_roundtrip
```

## Results

| Step | Status |
|------|--------|
| `cargo check -p semio-s-plugin-stdio` | **PASS** |
| `zzz_write_native_tiff_fixture` | **PASS** — wrote 168-byte `example.tiff` |
| `demo_subset_integrated_roundtrip` (tiff) | **PASS** — 2 tests |
| Log | `scratch-w3-stdio-tiff-gen2.txt`, `scratch-w3-stdio-roundtrip.txt` |

## Gaps

- Native fixture generator remains `#[ignore]` — run with `--ignored --exact` when DSL demo drifts.
- No negative fixture for owning `✳️any`.
