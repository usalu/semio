# Nine Owner Pack Wire Classification

Read-only source evidence; no codec execution.

All artifact paths below start ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts. Standard/subset paths match 📚️cold-example-owner-capabilities.md.

| Owner | Actual ArtifactPack representation | Existing native receiving evidence |
| --- | --- | --- |
| DOCX | Custom OpcNativeWriter/Reader binary, revision1 then schema + OPC package + XML parts; not Pack Record | No NativeReceive/DslReceiving found on authoritative root graph |
| PPTX | Custom OPC/XML native binary through native::encode_standalone/decode_binary | No NativeReceive/DslReceiving found |
| XLSX | Custom OPC/XML native binary through encode_standalone/decode_binary | No NativeReceive/DslReceiving found |
| ZIP | Typed DslRecord via __dsl_spec/__dsl_to_record/__dsl_from_record | RetainedClone/DslRecord present; no NativeReceive/DslReceiving found |
| PDF | Hand-authored multi-field Record; schema/version plus many Shape::Value members | No NativeReceive/DslReceiving found; many Value fields are not one Value envelope |
| EPW | Typed DslRecord via __dsl_spec/__dsl_to_record/__dsl_from_record | Root and multiple nested records already DslReceiving; no value::NativeReceive found |
| Binary | Raw bytes: encode clones bytes, decode constructs schema+bytes; no Semio envelope | Existing synchronous ArtifactPackReceiving at binary snapshot :41; no staged OriginalArtifactPackReceiving established |
| Deflate | Semio envelope around custom encode_deflate_snapshot/decode_deflate_snapshot zlib representation | No NativeReceive/DslReceiving found |
| IDML | Semio envelope around DslValue Pack, exact one field1 named value, Shape::Value | No NativeReceive found for actual IdmlSnapshot graph |

## Exact codec owners

For DOCX/ZIP/PPTX/XLSX/PDF/EPW/Binary/Deflate, actual implementation is the standard/subset 🚪️io/💾️binary/📸️snapshot/🦀️.rs:
DOCX :18, ZIP :13, PPTX :16, XLSX :12, PDF :14, EPW :12, Binary :21, Deflate :15.

DOCX native custom wire is nested 🧩️native/🦀️.rs write/read. write emits literal stdio.docx.pack v1, revision byte1, schema, package, then XML parts. Its authoritative root holds RetainedOpcPackage and DocxXmlParts containing RetainedXmlDocument. Paragraph/Run projections do not replace that ownership graph.

PPTX/XLSX nested 🧩️native/🦀️.rs likewise emit literal stdio.pptx.pack v1 / stdio.xlsx.pack v1 custom native wire. These methods cannot be decoded by supplying a generic RecordSpec to RetainedRecordReceiving.

PDF's sibling text snapshot spec explicitly lists many separate Value fields. take_value_into requires exactly one total Record field and therefore is inapplicable unchanged.

IDML is exceptional: codec lives at artifact root 📚️idml/🚪️io/🦀️.rs, not its editor subset. native_record_spec :18 declares exactly field1 value Shape::Value. encode_pack_with :42 calls self.to_value().encode_pack_with; decode :47 reverses through DslValue::decode_pack_with and FromValue. This matches the existing retained dynamic Value transfer shape, but bounded typed construction still needs actual NativeReceive implementations and package validation.

EPW schema/📸️snapshot/🦀️.rs root :290 plus nested records :19/:37/:50/:63 already declare DslReceiving. This is the strongest existing typed-record receiver candidate among these nine; validate the complete nested graph and authored record metadata rather than adding duplicate derives blindly.

A mandatory hydration receiver must preserve each current wire format. Uniform DslReceiving additions cannot decode custom OPC, raw Binary, or Deflate bodies. Uniform dynamic-value decoding would change eight actual source contracts.
