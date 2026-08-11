# F6c Independent Verification Report — bcf, png, deflate, obj, gltf, pptx, pdf1.7

**Method**: trusted nothing from the 7 sub-agents' self-reports. For every artifact: grepped the
live `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` files on disk myself for the real
trait impls and for leftover `serde_json` stubs, then independently re-ran
`cargo test -p semio-s-plugin-stdio --lib "<scoped filter>"` myself and read the actual test-runner
output (not the self-reports' claimed numbers). Finished with one full `cargo test -p
semio-s-plugin-stdio --lib` (whole crate) run of my own.

## Per-artifact results

| Artifact | Scoped tests (mine) | diff_codec_present | op_text_binary_present | serde_json_stub_gone | Notes |
|---|---|---|---|---|---|
| bcf | 18 passed / 0 failed | yes | yes | yes | `impl protocol::DiffCodec for BcfDiff` at `🔺️diff/🦀️component.rs:858`; `impl protocol::OpText`/`OpBinary for BcfMutation` at `🧬️mutations/🦀️component.rs:375/385`. Law tests (`op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`) are NOT colocated in the diff/mutations files like the other 6 artifacts — they live in `🏅️standards/🔖️2.1/⚙️engine/🦀️component.rs:966/1017` instead. Confirmed real and substantive (exercises every `BcfMutation` variant incl. the `BcfCamera` enum payload and tri-state `viewpoint_ref`; diff law exercises every collection triple and the camera-variant transition). Structural variation, not a gap. |
| png | 24 passed / 0 failed | yes | yes | yes | `impl protocol::DiffCodec for PngDiff` at `🔺️diff/🦀️component.rs:1562`; `OpText`/`OpBinary for PngMutation` at `🧬️mutations/🦀️component.rs:315/325`. Both law tests present and passing (`op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`). |
| deflate | 19 passed / 0 failed | yes | yes | yes | `impl protocol::DiffCodec for DeflateDiff` at `🔺️diff/🦀️component.rs:253`; `OpText`/`OpBinary for DeflateMutation` at `🧬️mutations/🦀️component.rs:100/122`. Both law tests present and passing. |
| obj | 19 passed / 0 failed | yes | yes | yes | `impl protocol::DiffCodec for ObjDiff` at `🔺️diff/🦀️component.rs:1348`; `OpText`/`OpBinary for ObjMutation` at `🧬️mutations/🦀️component.rs:267/288`. Both law tests present and passing. |
| gltf | 37 passed / 0 failed | yes | yes | yes | `impl protocol::DiffCodec for GltfDiff` at `🔺️diff/🦀️component.rs:2284`; `OpText`/`OpBinary for GltfMutation` at `🧬️mutations/🦀️component.rs:383/394`. Metabolism fixture (`artifacts::gltf::examples::metabolism::*`, 5 tests) confirmed still passing. Spot-checked `diff_codec_text_binary_roundtrip_law`'s fixture (lines 2740-2780 of the diff file): it deliberately documents covering a REPRESENTATIVE (not literal 42-of-42) subset of tri-state fields via two combined fixtures — `sweep_a/sweep_b` (every top-level `GltfDiff` field, `Perspective` camera, every `GltfAssetDiff` tri-state field `Some->None`) plus `tristate_snapshot_a/b` (node `mesh/camera/skin/matrix` `Some(Some)->Some(None)`, `translation/rotation/scale` `Some(None)->Some(Some)`, accessor `sparse` `None->Some`, all 4 material sub-diffs `None->Some`, buffer `uri` `Some->None`, an `Orthographic` camera, and `GltfJson`'s `Null`/`Number`/`Array` variants). This is a real, non-degenerate exercise, not an all-default case. |
| pptx | 50 passed / 0 failed | yes | yes | yes | `impl protocol::DiffCodec for PptxDiff` at `🔺️diff/🦀️component.rs:1686`; `OpText`/`OpBinary for PptxMutation` at `🧬️mutations/🦀️component.rs:277/287`. Both law tests present and passing. |
| pdf1.7 | diff scope: 19/19; whole v1_7 standard: 105/105; `bachelor_thesis` fixture: 6/6 | yes | yes | yes | `impl protocol::DiffCodec for PdfDiff` at `🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:1388`; `OpText`/`OpBinary for PdfMutation` at `.../🧬️mutations/🦀️component.rs:417/427`. Bachelor-thesis fixture (`artifacts::pdf::examples::bachelor_thesis::*`, the mandated real ~6.3MB fixture) confirmed still passing: `fixture_is_real_pdf_not_a_stub`, `real_decode_has_many_pages_and_real_extracted_text`, `decode_encode_decode_is_structurally_equal_at_page_level`, `analyzer_to_builder_round_trip_reproduces_equivalent_pages`, `codec_retention_law_bachelor_thesis_decode_encode_decode` all `ok`. |

No `serde_json::to_string`/`serde_json::to_vec`/`serde_json::from_str`/`serde_json::from_slice` stubs
were found anywhere in any of the 14 diff/mutations files searched. None of the 7 artifacts used
`#[derive(dsl::DslDiff)]` — all 7 are hand-rolled `DiffCodec` (each file documents, in a comment right
above the impl, that the derive was tried for real and rejected for a specific structural reason —
tri-state fields, non-struct shape, etc. — consistent with the F6 recon report's §3 finding that
`DslDiff` is struct-only and cannot express tri-state `Option<Option<T>>` semantics).

## Full crate test suite (my own run, final)

```
cargo test -p semio-s-plugin-stdio --lib
test result: ok. 1061 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.42s
```

**1061 passed, 0 failed.** This matches the highest final count independently claimed by the
gltf/bcf/pdf1.7 self-reports (1061) — png/pptx/deflate/obj self-reports show lower numbers because
they ran their whole-crate check earlier, before later-landing sibling sub-waves' own new tests were
present; all of them explicitly noted the count only went up over the session, never down, which is
consistent with what I observed running last.

## Conclusion

All 7 self-reports for the F6c fan-out (bcf, png, deflate, obj, gltf, pptx, pdf1.7) check out under
independent re-verification: every artifact has real hand-rolled `DiffCodec` + `OpText`/`OpBinary`
(no `serde_json` stubs), real (not degenerate) round-trip law tests, all scoped and full-crate test
runs pass, the gltf metabolism fixture passes, the gltf tri-state fixture is genuinely comprehensive
(not all-default), and the pdf1.7 bachelor-thesis fixture passes. No discrepancies found between
self-reports and disk/test reality.
