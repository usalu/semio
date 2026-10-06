# Own14 Whole Replacement Codec Authority

Source-only; all new family regions remain held and incomplete. No production change or gate run.

The actual own14 binary registry accepts `fn(&PdfMutation) -> Option<Result<Vec<u8>, String>>`; text accepts `fn(&PdfMutation) -> Option<String>`. Copying a PDF17 leaf with a different printer signature would not join this registry.

Existing own14 `binary::Reader::number` rejects nonfinite values; InsertPage also rejects nonfinite geometry. Preserve those existing operation rules. A whole replacement carrying an already accepted Snapshot must instead retain raw width/height words and must not call that finite-only reader. Existing leaf text uses hex-encoded Pack JSON, whose full Snapshot raw-word preservation has not been established. No whole-Set JSON codec is approved by this audit.

Own14 Snapshot already has genuine `ArtifactDsl::{print_dsl,parse_dsl}` and `ArtifactPack::{encode_pack_with,decode_pack_with}` using its own Record spec and envelope identity. Those are candidate original-domain seams for Set payloads, subject to an independent full word corpus and caller option propagation; no default/unlimited policy substitution is authorized. Core Record Float emits little-endian raw f64 and DSL controlled Float has a `nan64_<bits>` spelling, but that source evidence alone does not prove the proposed whole operation codec.

The next coherent held extension is the own14 leaf protocol declaration, exact raw-word literal fixture, matching aggregate registry/variant/facet/descriptor joins and signed-zero/nonfinite operation roundtrip demand. The current two-file Set draft declares codec modules that are not supplied; it is not mountable independently. Identity stays frozen by the actual editor/Set check, with no new Diff schema lane or whole replacement finite guard.
