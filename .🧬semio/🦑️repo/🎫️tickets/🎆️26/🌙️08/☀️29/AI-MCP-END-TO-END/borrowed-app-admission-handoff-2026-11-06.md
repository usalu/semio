# Borrowed App Admission Handoff — 2026-11-06

Root inspected this lane but has made no production edits in it. Launch execution was asked to take it after its actual Session type/custody audit, while Root starts actual MCP consumer wiring. Guest execution retains ingress/main exchange/transaction/document/relay ownership.

## Exact Files

All below are relative to framework OS plugin modules:
- ⚛️reactor/🔌️app/🦀️.rs: PluginApp trait defaults and signatures.
- 🧵️instance/🔌️plugin-app-impl/🦀️.rs: actual VcsArtifactApp trait implementations for config and archive.
- 🧵️instance/🔌️plugin-app-impl/🗃️document-load/🦀️.rs: actual archive validation and registry admission.
- 🧵️instance/🔌️plugin-app-impl/🎞️media-transfer: actual media implementations (inspect exact child file).
- ⚛️reactor/🏃️runtime/🧰️ops/🎞️media/🦀️.rs: plugin_consume_media currently constructs a temporary owned MediaArtifact and consumes its archive result.
- ⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/{⚙️config-commands,🎞️media-commands,📦️document-archive-commands}/🦀️.rs.

## Actual Mismatches and Custody

All three arm files now borrow &mut protocol::AppCommand. Archive arms pass &mut DocumentArchivePack to existing owned trait APIs; config constructs ArtifactPackFiles from borrowed fields; media arm takes an owned descriptor while main exchange borrows it from retained State.media_input. These are not type-coherent yet.

begin_document_archive_load currently validates member count/payload size/slot availability/nonempty parent, sorts the consumed archive by ordinal, validates ten member identity fields and envelope presence, reverses members, constructs document_archive_history_decoder, then inserts ActiveDocumentArchiveLoad::new into the fixed registry. On any early failure the consumed archive drops. A borrowed adoption must retain caller data through all fallible work and quote/pay final actual donor clearing/registry placement. Validation, sorting and history creation also need real bounded progress/cancellation; do not hide the issue behind an owned mem::take before the existing method. Merge currently calls the same owned begin method and marks the admitted registry row afterward.

load_config_pack currently borrows ArtifactPackFiles and parses its pack/spr to a ParsedDocumentText then resets config_store and clears history cache. Raw binary AppCommand fields are Vec bytes in the channel contract, so confirm the actual input meaning before assuming String or flattening/copying payloads. A borrowed pack/spr interface can preserve donors through parsing; the relevant source constructor must remain retained before later reset failures.

plugin_consume_media currently takes owned descriptor + Vec data, constructs local MediaArtifact, passes it by value to PluginApp.consume_media, and on MediaConsumption::DocumentLoad(archive) calls the owned archive admission. The trait default matches artifact.descriptor.wire by value and whole_document_archive(&artifact.data), or builds a typed MediaArtifactError. Preserve actual input, partial consumption and full error before any subsequent fallible app/registry operation.

Guest confirms OriginalCommandExchangeState already owns media_input:Option<(protocol::AppCommand,MediaArtifactDescriptor)> and media_artifact:Option<MediaArtifact>. It permits the callee-lane owner to add media_consumption:Option<MediaConsumption> and an exact config/archive cursor slot plus empty initializer narrowly; Guest will preserve them in its main exchange edits. State derives RetireOwned, so every added owner needs its genuine retirement graph. Main currently passes a borrowed descriptor from media_input into arm_media_in. Guest owns that main file; coordinate exact signature/call changes.

No test, compile, or successful server behavior is claimed by this handoff.
