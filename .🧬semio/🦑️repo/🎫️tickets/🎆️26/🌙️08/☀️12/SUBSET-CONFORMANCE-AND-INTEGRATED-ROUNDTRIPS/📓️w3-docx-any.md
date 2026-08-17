# W3 DOCX Any — Subset Conformance Roundtrip

Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`  
Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any`

## Archetype & fidelity

| Field | Value |
|-------|-------|
| Archetype | **owning** |
| `IoFidelityClass` | **Exact** (native ZIP `example.docx` via `encode_docx`/`decode_docx`) |
| `subset!` macro | not used |

## Changed files

| File | Change |
|------|--------|
| `…/✳️any/📚️examples/🎬️demo/🦀️component.rs` | `DocxAnyRoundtrip` + `demo_subset_integrated_roundtrip` (rewritten) |
| `…/✳️any/⚙️engine/🦀️component.rs` | `fixture_honesty_law` + `zzz_write_native_docx_fixture` (`#[ignore]`) |
| `…/✳️any/📚️examples/🎬️demo/🖼️assets/example.docx` | populated (1648 bytes) |
| `…/🪆️subsets/🔣️component.json` | owning archetype, exact fidelity, zip import/export, `examples: ["demo"]` |
| `📦️packages/🦀️rust/📦️glue.rs` | engine mount + `docx::examples::demo` shim → subset path |

## Harness wiring

- **Test:** `demo_subset_integrated_roundtrip`
- **Location:** `✳️any/📚️examples/🎬️demo/🦀️component.rs` (mounted as `artifacts::docx::examples::demo`)
- **Dialect:** `s.stdio.docx` / `ecma-376` / `*`
- **Native:** `include_bytes!("🖼️assets/example.docx")`

## Commands

```bash
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH"
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-stdio"
cargo test -p semio-s-plugin-stdio --lib 'artifacts::docx::standards::v_ecma_376::engine::tests::conformance_laws::zzz_write_native_docx_fixture' -- --ignored --exact
cargo test -p semio-s-plugin-stdio --lib subset_integrated_roundtrip
```

## Results

| Step | Status |
|------|--------|
| `cargo check -p semio-s-plugin-stdio` | **PASS** |
| `zzz_write_native_docx_fixture` | **PASS** (prior run) — 1648-byte `example.docx` |
| `demo_subset_integrated_roundtrip` (docx) | **PASS** |
| Log | `scratch-w3-stdio-native-gen2.txt`, `scratch-w3-stdio-roundtrip.txt` |

## Gaps

- Native fixture generator `#[ignore]` — regenerate when demo DSL/pack drifts.
- No negative fixture for owning `✳️any`.
