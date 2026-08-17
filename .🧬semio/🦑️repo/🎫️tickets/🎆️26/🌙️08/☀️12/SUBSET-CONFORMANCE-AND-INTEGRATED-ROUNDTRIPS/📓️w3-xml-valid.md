# W3 XML Valid — Derived Subset Conformance Roundtrip

Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid`

## Archetype & fidelity

| Field | Value |
|-------|-------|
| Archetype | **derived** (`derivesFrom: "*"`) |
| `IoFidelityClass` | **Canonical** (DSL text; validation gates are the derived contract) |
| `is_derived()` | `true` in `SubsetRoundtripSpec` |
| Engine | reuses `✳️any` engine via builder gates (no separate engine module) |

## Changed files

| File | Change |
|------|--------|
| `…/✳️valid/🚪️io/🦀️component.rs` | `XmlValidRoundtrip` + `xml_valid_subset_integrated_roundtrip`; `validate_payload` / `validate_negative` call `check_valid_conformance` |
| `…/✳️valid/🚪️io/🟦️component.ts` | derived metadata mirror (archetype, hardCodes, import/export kinds, negativeExamples) |
| `…/✳️valid/🧬️schema/🟦️component.ts` | `checkValidConformance()` TS mirror (pre-existing, verified) |
| `…/✳️valid/📚️examples/🚫️no-doctype/` | negative fixture (`broken.xml` — no doctype) |
| `…/🪆️subsets/🔣️component.json` | derived manifest, `hardCodes`, `negativeExamples: ["no-doctype"]` |
| `📦️packages/🦀️rust/📦️glue.rs` | `valid::examples::no_doctype` mount |

## Harness wiring

- **Test:** `xml_valid_subset_integrated_roundtrip`
- **Location:** `✳️valid/🚪️io/🦀️component.rs` (`derived_composition::tests`)
- **Positive:** `✳️any/📚️examples/🎬️demo` DSL (conforming doctype)
- **Negative:** `✳️valid/📚️examples/🚫️no-doctype` — hard-reject path must emit declared code `stdio.xml.valid.doctype-missing`
- **Dialect:** `s.stdio.xml` / `1.0` / `valid`

## Commands

```bash
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH"
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio --lib xml_valid_subset_integrated_roundtrip
cargo test -p semio-s-plugin-stdio --lib subset_integrated_roundtrip
```

## Results

| Step | Status |
|------|--------|
| `cargo check -p semio-s-plugin-stdio` | **PASS** |
| `xml_valid_subset_integrated_roundtrip` | **PASS** (positive roundtrip + negative gate fail-closed) |
| Log | `scratch-w3-stdio-roundtrip.txt` |

## Gaps

- Vitest mirror for `🟦️component.ts` not executed in this worker (Rust harness is authoritative).
- Derived subset has no native binary IO — canonical DSL only.
