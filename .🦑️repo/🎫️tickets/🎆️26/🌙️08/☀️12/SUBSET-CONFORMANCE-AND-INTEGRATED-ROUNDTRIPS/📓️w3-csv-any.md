# W3 CSV Any — Subset Conformance Roundtrip

Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any`

## Archetype & fidelity

| Field | Value |
|-------|-------|
| Archetype | **owning** |
| `IoFidelityClass` | **Exact** (native `example.csv` roundtrip) |
| `subset!` macro | not used (no stdio sibling refs) |

## Changed files

| File | Change |
|------|--------|
| `…/✳️any/📚️examples/🎬️demo/🦀️component.rs` | `SubsetRoundtripSpec` + `demo_subset_integrated_roundtrip` in existing `#[cfg(test)]` |
| `…/✳️any/⚙️engine/🦀️component.rs` | engine under subset (coordinator move) |
| `…/🪆️subsets/🔣️component.json` | owning manifest, `examples: ["demo"]`, exact fidelity |
| `📦️packages/🦀️rust/📦️glue.rs` | subset engine/examples mounts |

## Harness wiring

- **Test:** `demo_subset_integrated_roundtrip`
- **Location:** `✳️any/📚️examples/🎬️demo/🦀️component.rs` (`#[cfg(test)]`)
- **Spec:** `CsvAnyRoundtrip` — dialect `s.stdio.csv` / `rfc4180` / `*`, native parse via `decode_csv`, export via `encode_csv`, sample `SetCell` mutation

## Commands

```bash
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH"
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-stdio"
cargo check -p semio-s-plugin-stdio
cargo test -p semio-s-plugin-stdio --lib subset_integrated_roundtrip
```

## Results

| Step | Status |
|------|--------|
| `cargo check -p semio-s-plugin-stdio` | **PASS** (2026-08-12) |
| `demo_subset_integrated_roundtrip` (csv paths) | **PASS** — 2 tests (artifact shim + subset mount) |
| Log | `scratch-w3-stdio-roundtrip.txt` |

## Gaps

- No negative fixture (validate_negative returns SKIP — expected for owning `✳️any`).
- Legacy duplicate mount at `artifacts::csv::examples::demo` still runs same test (harmless).
