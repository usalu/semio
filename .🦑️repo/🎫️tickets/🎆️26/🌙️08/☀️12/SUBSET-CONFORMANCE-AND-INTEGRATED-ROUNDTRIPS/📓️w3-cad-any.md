# W3 CAD `✳️any` — Owning Reference Proof

Completed: 2026-08-12. Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`.

## Reference

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any`

## Summary

Owning subset `✳️any` for `s.cad.cad` / standard `1` is complete: manifest, engine, IO (import+export), examples, and inferences under `🧬️schema/💡️inferences` are present. Integrated roundtrip harness `CadAnyRoundtrip` passes on the real `demo` DSL fixture.

## Conformance checklist

| Item | Status |
|------|--------|
| `🪆️subsets/🔣️component.json` owning manifest | ✅ |
| `⚙️engine/` under subset | ✅ |
| `🚪️io/` import + export | ✅ |
| `📚️examples/🎬️demo/` | ✅ |
| Inferences under `🧬️schema/💡️inferences` (not top-level) | ✅ |
| `subset!` macro registration | ⏭️ deferred — manual `derived_composition` + `engine::register` pattern (same as other W3 stdio refs; macro not yet adopted in plugin IO leaves) |
| Integrated roundtrip test | ✅ **PASS** |

## Fidelity & harness

- **Fidelity class:** `Semantic` (`IoFidelityClass::Semantic`)
- **Dialect:** `s.cad` / `1` / `*`
- **Stages:** S0–S8 exercised; no stage skips declared
- **S4 validate_payload:** real DSL re-parse on exported bytes
- **S10 validate_negative:** `SKIP:owning subset has no negative fixture`

## Commands

```bash
export PATH=/usr/bin:/bin:/usr/sbin:/sbin:/Users/ueli/.bun/bin:/opt/homebrew/bin:$PATH
export RUSTC_WRAPPER=
TICKET=$(ls -d .🦑️repo/🎫️tickets/🎆️26/*/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS)
export CARGO_TARGET_DIR="$TICKET/🎯️target-w3-cad"
cargo test -p semio-s-plugin-cad demo_subset_integrated_roundtrip
```

## Result

```
test examples::art_cad_demo_tests::demo_subset_integrated_roundtrip ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Log: `scratch-w3-cad-roundtrip2.txt`

## Changed files (this wave)

| File | Change |
|------|--------|
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs` | `super::engine_component` + `v1::engine` shim |
| `✏️s/🔌️plugins/📐️cad/.../✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs` | `store::os_store::test_support` imports |
| `✏️s/🔌️plugins/🗄️stdio/.../✳️any/🚪️io/🦀️component.rs` | `SemioSubsetSnapshot::Text` validator dispatch |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | engine shims (`super::subsets`), note example path |

## Remaining gaps

- `subset!` owning registration not migrated (manual registration remains canonical for this plugin)
- Norm/stdio sibling fixes were required to unblock `semio-s-plugin-cad` compile (shared `semio-s-plugin-stdio` dependency)
