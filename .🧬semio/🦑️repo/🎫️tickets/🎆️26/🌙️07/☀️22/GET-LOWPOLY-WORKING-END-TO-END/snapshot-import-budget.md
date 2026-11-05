# Snapshot Import Budget Audit

Read-only source audit plus independent neutral byte oracle, 2026-11-05. No Native/browser/server or production changes.

## Confirmed Boundary Gap

Both public importSnapshotJson and replaceSnapshotJson cannot carry a valid canonical one-layer nonwhite snapshot through the current admitted command route. A256² RGBA layer has262,144bytes; base64 alone is349,528ASCIIbytes. The small independent constant-color recipe produces349,838UTF8 JSON bytes,333,454over the ordinary16,384byte limit. Raw Pack stores string UTF8, so its complete command cannot be smaller than the JSON string; framing only adds bytes.

This is a size contradiction proven from actual source limits and measured bytes, not an executed Rust refusal or failing Native regression. A tiny snapshot with empty pixels can fit, but replacing a nonwhite document with that input would evade the required public roundtrip rather than fix it.

## Actual Public Route And Three Independent Gates

Subset/editor prefix: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

- `📋️manifest/🦀️.rs:72–73` publicly declares both actions; lines204–206 mark them migrated. Committed descriptor has both inPalette:true, args:[] and the actual editor controller s.lowpoly.lowpoly@1/*#editor. It does not mark them private/test-only. The palette metadata provides no JSON argument editor; direct public dispatch can still provide args via the action bridge. I did not execute a palette click or claim how an absent argument is presented.
- `🌉️args-bridge/🦀️.rs:160–161` folds value→json and decodes the real ImportSnapshotJson/ReplaceSnapshotJson payloads. Command structs in `🎮️commands/📄️document/🦀️.rs:28,45` contain json:String with no declared dimensional/byte limit.
- Public original command producer uses lowpoly_command_maximum_raw_bytes: `🧵️retained-commands/🦀️.rs:114–115` grants the larger ceiling ONLY to ImportMeshFile; these two variants receive16,384. Producer refuses an overlong OpBinary wire before dispatch effects.
- Same owner lines27–41 puts both JSON actions in LowpolyCommandJobFactory ordinary keys. lowpoly_contract line107, factory admission macro line453 and ordinary factory line457 all retain16,384wire ceiling. LowpolyImportJobFactory and its larger proof/producer ceiling apply exclusively to importMeshFile.
- Even a direct handler bridge which bypassed the wire cap would still face lowpoly_command_admitted lines140–141: payload.json.len≤LOWPOLY_RETAINED_RAW_BYTES for both. This JSON guard is16KiB, not the4,096byte ordinary scalar-field guard; widening only a producer would therefore remain insufficient.
- Handler reset_from_json lines14–18 uses owning lowpoly_json_decode and emits reset_document_effect/LoadDocument outside undo history on success. It cannot repair a command rejected at earlier producer/factory/payload gates.

Thus canonical texture validity is broader than this public whole-snapshot command envelope. The main texture schema's empty/full262,144octet validity and command String declarations do not advertise a16KiB snapshot subset. Runtime was notified before proposing any production change.

## Neutral Recipe And Third-Party Oracle

Ticket inputs: snapshot-boundary.json, snapshot-boundary.schema.json, snapshot-boundary.mjs. Recipe names Canonical Constant Color Snapshot,256×256,RGBA17/34/51/255; compact JSON contains one named object, transform, one Color layer and no mesh. Full expanded source/snapshot is not copied to the ticket.

Registered command executed:

```powershell
repo tools exec --light node -- './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️22/GET-LOWPOLY-WORKING-END-TO-END/snapshot-boundary.mjs'
```

Final exit0. Strict Ajv validates the neutral recipe and generated decoded pixel bytes against the actual canonical texture schema. Independent Node Buffer generates/roundtrips base64, measures UTF8 JSON and compares against the extracted current ordinary raw-byte constant. Observed262144pixelbytes,349528base64bytes,349838JSONbytes,16384ordinarylimit. This is explicitly labeled source-budget-and-independent-byte-oracle/nativeExecuted:false. It does not call a duplicated Rust handler/codec or assert Native TDD red. Initial invocation exposed a file-URL pathname decoding bug in the ticket oracle; corrected to standard fileURLToPath for emoji paths/cross-platform use, then final verification passed.

## Natural File / Global IO Is Distinct

The existing natural-file host helper uses typed binary file IO and bounded byte reads rather than passing a full JSON string through these command variants. That architectural lane does not inherit their16KiB payload string cap. However the current Lowpoly committed editor definition advertises importFormats:[] and exportFormats:[], and no openArtifactFile/saveArtifactFile action; naturalFileFormatV1 requires matching declarations. Therefore this audit does not claim current Lowpoly has a working natural-file substitute for these advertised palette commands.

Owning Lowpoly JSON/text/SQLite codecs and their direct roundtrip tests establish separate serialization interfaces, not public action-bus admission of a350KiB String. A codec test passing cannot close this gap.

## Smallest Required Follow-Up

Author public mounted boundary regression with the same named canonical-color recipe, dispatch each actual JSON action with its real controller and payload, require no partial document/history effects before admission/completion, then capture exactly one final LoadDocument effect and decode/compare pixels. The source proves current gate failure but genuine Native execution is still needed for that behavioral red.

Choose an explicit supported whole-snapshot input budget/transport and align producer selector, exact factory/proof contract, typed payload guard and UI argument metadata together. Do not globally enlarge every small command, reuse the mesh-only name cap blindly, bypass the public producer in the regression, or replace painted fixtures with empty pixels to fit16KiB. For broad valid documents, chunked/native document IO may be the proper declared feature; it must actually be published and tested before removing/reclassifying these public commands.
