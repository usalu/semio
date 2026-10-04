# S4-PACKFIX Runaway Sweep Audit (21:18–21:20)

The runaway applied the canonical kernel mapping (`🧪️s4-packfix-sweep.py`) to retired `PackError::<variant>` spellings and to the S4-TOOLS-B interim `Malformed { what: "pack", offset: 0 }` form. For each owner-tree file it touched, this lists how many current lines equal the canonical migration of HEAD (19:39) and are absent from HEAD. `current == canonical(HEAD)` means the file holds no edit that differs from the canonical table. `mixed` means the owner's own edits are present alongside. R1 lines are type-directed `PackError::from(<source>)` conversions, which only compile for `ValueError`, `TextError`, `PackRefusal` and `PackError` sources. R2 lines carry a non-`InvalidValue` authored kind.

```
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
## STROKES | mixed | 4 sweep lines | typed-from 1 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs
   R1 semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs
## STROKES | mixed | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🎚️config/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs
## STROKES | mixed | 4 sweep lines | typed-from 1 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs
   R1 semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs
## STROKES | current == canonical(HEAD) | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs
## STROKES | mixed | 4 sweep lines | typed-from 1 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config/🦀️.rs
   R1 semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## STROKES | mixed | 4 sweep lines | typed-from 1 | non-InvalidValue 0 | ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config/🦀️.rs
   R1 semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## STROKES | mixed | 4 sweep lines | typed-from 1 | non-InvalidValue 0 | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️config/🦀️.rs
   R1 semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🦀️.rs
## STROKES | mixed | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️transient/🦀️.rs
## TOOLS-B | mixed | 2 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## TOOLS-B | mixed | 2 sweep lines | typed-from 1 | non-InvalidValue 0 | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
   R1 EnergyModelPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(|error| store::PackError::from(error))
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🎚️config/🦀️.rs
## WIRES-MATH | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🦀️.rs
## WIRES-MATH | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️model/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## STROKES | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs
## STROKES | current == canonical(HEAD) | 4 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs
## TOOLS-B | mixed | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs
## WIRES-MATH | current == canonical(HEAD) | 3 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🦀️.rs
## TOOLS-B | mixed | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
## TOOLS-B | current == canonical(HEAD) | 2 sweep lines | typed-from 2 | non-InvalidValue 0 | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs
   R1 let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))?;
   R1 protocol::OpText::parse_op(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## TOOLS-B | mixed | 3 sweep lines | typed-from 2 | non-InvalidValue 0 | ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🦀️.rs
   R1 let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))?;
   R1 semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
## CONVERTED(txt/csv/tsv/png) | current == canonical(HEAD) | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs
## CONVERTED(txt/csv/tsv/png) | current == canonical(HEAD) | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs
## CONVERTED(txt/csv/tsv/png) | current == canonical(HEAD) | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs
## CONVERTED(txt/csv/tsv/png) | current == canonical(HEAD) | 1 sweep lines | typed-from 0 | non-InvalidValue 0 | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs
```
