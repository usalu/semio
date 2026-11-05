# Snapshot Input Review

Read-only source audit, 2026-11-05. No Native, server, production edits, or compile/test-body claims.

## New Input Wiring Is Coherent

Editor retained-command owner now has ordinary45keys and import3keys: importMeshFile/importSnapshotJson/replaceSnapshotJson. The JSON IDs were removed from the ordinary list/publication table and added to the import owner with HostOnly publication lanes.

Three enforcement layers:
1. Public OpBinary producer selector returns2MiB+32 for JSON variants, mesh2MiB+4096+32 for importMeshFile,16KiB for ordinary commands.
2. LowpolyImportJobFactory raw admission and execution contract use the common larger2MiB+4096+32 owner ceiling. This slightly exceeds JSON-specific producer wire limit but is a valid common upper bound; it does not weaken JSON field validation. Proof rows for all3keys use lowpoly_import_contract and with_factory_type::<EditorApp<LowpolyPlayApp>,LowpolyImportJobFactory>, so actual registered factory TypeId/owner/contract matches. Ordinary proof rows retain their own concrete factory type/16KiB contract.
3. lowpoly_command_admitted caps JSON UTF8 text at LOWPOLY_SNAPSHOT_JSON_BYTES2MiB. reset_from_json independently checks the same constant, returns localized lowpoly.snapshot-json.capacity on excess, parses the actual owning JSON codec, validates canonical paint, then emits LoadDocument. Invalid parse uses lowpoly.snapshot-json.invalid; texture failures have the separate notice.

Required public json text argument declarations with native EN “Snapshot JSON” / DE “Snapshot-JSON” now exist for both actions in manifest source. This is source wiring, not an executed assembled manifest/browser check. Descriptor regeneration and actual red→green oracle are owned by root/Verify.

## Remaining Valid-State Execution Closure Gap

The handler validates canonical layer shape, but does not establish that loaded target can execute future Lowpoly tools. The model schema does not declare object-count64, layer-count8 or string4096limits. Those are retained execution envelope assumptions:

`✏️editor/🧵️retained-commands/🦀️.rs:122–136` lowpoly_snapshot_admitted checks≤64objects,≤8layers/object,≤4096bytes for schema/IDs/names/meshhandle/layername/blendmode. Every command first requires that CURRENT snapshot predicate, including replacement/import/delete/export. build_tool_job and work step both enforce it.

Concrete neutral model-shaped snapshots measured through registered Node Buffer, with canonical empty layers and no mesh:
-65objects:16,736UTF8 JSON bytes.
-1object/9layers:905bytes.
-1object with4097ASCIIbyte name:4388bytes.
All fit new2MiB input budget and pass paint extent validation. The public handler can emit LoadDocument for them, but if applied, the next retained command's current-snapshot predicate fails. Even replacement intended to recover is blocked by that same precondition. General host load/resource admission was not executed; this is a deterministic source-level handler/envelope mismatch, not a claimed live locked browser session.

The generic refusal is Fault::from("lowpoly-retained-command-capacity"), with no new dedicated bilingual notice explaining object/layer/name limit. The new three notices cover JSON byte capacity/JSON syntax/texture extent, not this target-state closure. Do not misreport the2MiB declaration as fixing it.

## Preexisting AddPrimitive Boundary Is Separate

addPrimitive at exactly64objects satisfies the current precondition; its scalar kind guard has no prospective count limit. Handler `🎮️commands/➕️add-primitive/🦀️.rs:22–39` builds a primitive, takes the new object, then emits CreateObject at index=projection.objects.len(). Engine add_primitive pushes the new object without a64check. No prospective lowpoly_snapshot_admitted check occurs in this handler/work completion. New model diff validation checks paint extent, not retained object count.

Thus source permits64→65objects and can produce the same next-command envelope failure independently of enlarged snapshot input. This is a preexisting structural closure defect, not a regression unique to new JSON transport. Similar expansion operations need separate review; this audit does not claim their live behavior was executed.

## Decision Boundary

Do not blindly promote64/8/4096 into ModelSchema merely to make bounded implementation assumptions look authoritative. The canonical texture invariant was explicit model intent; these envelope caps were not. Root owns the clean valid-state execution design.

A sound follow-up must preserve a supported route for every accepted valid document: bounded/paged traversal and honest per-operation resource admission, or an explicitly authored product-domain limit with all ingress/transitions/fixtures updated together. Import replacement and recovery must not depend on an already inadmissible source shape. A local prospective refusal can prevent new stranding while the broader decision is made, but it must be deliberate public policy with localized reason and atomic behavior, not hidden blanket schema narrowing.

Future mounted tests should load each neutral target through the actual JSON action, verify follow-up delete/export/replace or explicit pre-load typed refusal, and check64→65transition with before/after state/effects. Do not test only direct handlers or empty snapshots and call the entire valid-model contract covered.

Lowpoly still has no natural_file_codec override; typed editor assembly only mounts paired natural actions forSome(codec). Static committed IO currently has empty import/exportFormats and no natural Open/Save pair. There is no hidden working native-file alternative established by this review.
