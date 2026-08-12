# W3 Verify Follow-up

## Unblocked
- Framework plugin E0499 already fixed (child_ptrs pattern in `dispatch_emit_group`).
- `semio-s-plugin-stdio` lib compiles again after:
  - restored missing `📎️note` DSL/pack fixtures under semio `✳️any` (and text subset copy)
  - csv/tiff/xml artifact `io_registry` imports retargeted to `subsets::any::engine::io_registry`
- Accidental move of `🖍️draw/🔄️fsm` into subset engine tree restored to plugin-root `🔄️fsm` (workspace Cargo.toml was failing).
- Kernel harness recheck: `assert_subset_harness` passed (1 test).

## Still verifying
- CAD `demo_subset_integrated_roundtrip` re-run after path/`os_store::test_support` import fix and FSM restore.
- EN1990 previously failed compiling other norm apps (`SetSnapshot` missing on several EN199x command enums) — peer/app surface issue, not the en1990 subset body itself.
- DOCX + semio mesh reference worker still in flight.

## Reports from prior workers (structure done, cargo was blocked)
- w3-cad.md, w3-en1990.md, w3-csv.md, w3-tiff.md, w3-xml.md
