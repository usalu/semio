# Composed Child Full-Document Persistence Audit

## Scope and conclusion

This is a read-only audit of the actual Pack, text, protocol, and member-factory paths for Animate Presentation, Architect Program, and Imperative Procedure. No source or tests changed; no Cargo command ran.

The bare parent codec is correct: an ArtifactChild persists its typed child handle only because an owned child is a separate envelope and VCS history. The P0 is not to put child payload back into ArtifactSnapshot::encode_pack. The P0 is that every public API described as a full document serializes and replaces only the parent envelope, while these three apps retain child content only in process-local state and declare no usable child-member factory.

A separate ReadChildren / LoadChildren pair exists, so the framework has a low-level way to persist child envelopes. It is neither an atomic full-document archive nor usable for these applications as currently declared.

## Actual document payloads

| Path | Actual payload | Children included |
| --- | --- | --- |
| PluginApp::document_pack and plugin_document_pack | print_document_pack(self.store.envelope()) | No |
| PluginApp::document_text and plugin_document_text | print_document_text(self.store.envelope()) | No |
| AppCommand::ReadDocument / LoadDocument | parent Pack plus SPR | No |
| default media Document export/import | document_pack, then load_document_pack | No |
| ReadChildren / LoadChildren | ChildPackEntry[] of child full envelopes | Separately |

Evidence:

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23307-23338 prints/parses/resets only self.store.envelope() for document text and Pack.
- The exported helpers at the same file:28466-28507 call those methods. The docstring at :28487-28489 calls the parent Pack a “full persistent document”; that is false for a composing document.
- :29938-29983 routes ReadDocument / LoadDocument independently from ReadChildren / LoadChildren.
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:27-42 explicitly says the parent Document pair is insufficient because composed children have their own envelopes.
- The default Document media export/import at 🔌️plugin/🦀️.rs:10996-11023 also uses only the parent methods.
- SpaceMember exposes envelope_pack_bytes but no text-envelope equivalent (🏪️store/🦀️.rs:18337-18342); there is no complete child text archive today.

The generic test named a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames does not exercise a parent reload. It obtains child_packs, creates a fresh default parent, and loads only the entries (🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:3091-3115). It proves member-factory opening against a compatible default parent, not complete document persistence.

## Child identity is global, not a local alias

The equality gate is intentional and has shared-contract evidence:

- The kernel distinguishes CHILD, one owner and a separate child envelope, from LINK, an independently-lived pinable reference (🏪️store/🦀️.rs:2695-2712).
- ArtifactChild projects child_id and target.artifact_id separately into ChildRefFields (:2819-2845).
- ChildRestoreProjection rejects unequal values (:3009-3029); admits_member requires both to equal the opening ArtifactRef (:2966-2976).
- open_member_store validates owner.child_id, history.doc_id, and expected.artifact_id as the same identity (:3220-3249).
- The composition design explicitly states that one id is used for the child ArtifactRef.artifact_id, ArtifactChild.child_id, and OwnerRef.child_id (.🧬️semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b2-store-composition-report.md:141-145).
- The 15-row neutral contract lists parent child/target-id disagreement as invalid (.🧬️semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️root-member-dialect-authority.md:69-73; native consumer at 🏪️store/🧪️tests/🔬️unit/🦀️.rs:7012-7059).

Do not relax the equality gate. Repair the affected handle constructors so target.artifact_id equals child_id.

## P0 — Presentation

Presentation correctly declares presentation and animation as child slots, yet it creates no child member stores.

- presentation_child_handle makes a content-hash child_id but a fixed target id at ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🦀️.rs:163-177.
- animation_child_handle repeats the error at :179-195.
- PRESENTATION_SCRATCH is the only content owner (:198-215). presentation_working_scene_for_handle returns a default source and no tiles when the cache misses (:217-224), and all normal reads route through it (:226-240).
- presentation_snapshot_with_tiles only seeds the cache while storing handles (:243-248).
- The editor and viewer registrations use VcsArtifactApp with the default NoMembers parameter (:383-392); the editor has no child_restore_projection override (editor/🦀️.rs:579-669).

After a fresh parent Pack/text load, the child content has not been loaded at all. If a host separately submits entries, the loaded parent projection rejects both Presentation handles before M::open because child_id differs from target.artifact_id. If that were bypassed, NoMembers cannot open the stdio dialect.

Reachable outcome: non-default source/tile edits render from the scratch cache; a fresh LoadDocument retains only the handles and renders a default empty deck.

Required change:

1. Set each Presentation child target id to its emitted child id; remove the conflicting fixed target constants.
2. Add a concrete space_members! enum for exact stdio Presentation and Animation dialects and use it at editor and viewer VcsArtifactApp construction.
3. Add child_restore_projection in both roles, delegating to store::ChildRestoreProjection::from_snapshot.
4. Replace the scratch map as authority. Resolve the exact child store via ArtifactView.children; missing required content must be a fault, not a default deck.
5. Create/update child stores through owned composition operations, then replace parent handles in that same operation. The empty animation still needs one actual canonical child entry.

## P0 — Architect Program

Program has the same two defects.

- benchmarks_child_from_records hashes the child id but uses fixed architect-program-benchmarks as target id (✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs:79-110).
- knowledge_child_from_records does the same with architect-program-knowledge (:149-175).
- ProgramBenchmarksWorkingTable and ProgramKnowledgeWorkingTable are local owners only; their public readers silently return empty vectors when the reconstructed handle has no owner (:105-110, :168-175).
- empty_plugin installs the invalid handles (:334-339).
- The legacy document codec uses EditorApp<ArchitectPlayApp> with default NoMembers (:557-563). No Program source has space_members! or a child_restore_projection override; the viewer also has none (viewer/🦀️.rs:42-82).

The catalog reads the registers through program_knowledge and program_benchmarks (editor/🗂️catalog/🦀️.rs:173-182 and :264-272). Thus fresh parent-only load turns a durable non-empty knowledge/benchmark register into empty runtime data, and subsequent normal mutations can report target-missing.

Required change:

1. Make target.artifact_id equal each content-addressed scene id.
2. Add ProgramMembers for the exact stdio Table dialect, used by editor and viewer VcsArtifactApp construction.
3. Add both role projection hooks.
4. Write the Table converter results into Table child envelopes. ProgramDiff carries only parent handles; the child envelopes own record rows/history.
5. Resolve catalog, mutations, inference, and renders through ArtifactView.children and reject missing required children rather than returning empty data.

## P0 — Procedure

Procedure constructs valid child identities but still has no persistent member stores.

- Its Flow and Text builders use target.artifact_id = child_id (✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs:181-209).
- ProcedureFlowWorkingData and ProcedureTextWorkingData are local-only. Missing local owners map to default Path/seed (:212-263).
- Every mutation derives a whole replacement Flow handle from procedure_working_scene (:272-279; for example reorder-steps diff :24-27).
- Editor/viewer main and script rendering use that same working scene (editor main:65-69; viewer main:27-33; viewer script:24-29).
- Its document codec uses default EditorApp<ImperativePlayApp> (:337-345), and neither role supplies child projection authority.
- The current child-owner test proves the gap: local owners disappear through JSON handle roundtrip (procedure/🧪️tests/🔬️unit/🦀️.rs:35-52).

Reachable outcome: create/edit steps, export and reopen through the normal Pack/text route. The parent has handles but no Flow/Text payload. Renders are empty, and the next edit derives a replacement Flow from empty Path, overwriting prior work.

Required change:

1. Add ProcedureMembers for exact stdio Flow and Text dialects at editor/viewer construction.
2. Add both projection hooks.
3. Materialize converted Flow/Text snapshots as actual child store genesis/update payloads, never just local owners.
4. Feed render, execution, and mutation preparation from ArtifactView.children; missing child content must block the operation.
5. Keep parent codecs as handle-only. They are correct once the complete composition archive exists.

## P0 — document replacement is not an atomic composition replacement

load_document_text and load_document_pack reset the parent but retain self.children (🔌️plugin/🦀️.rs:23311-23318 and :23325-23338). LoadChildren then opens entries one at a time (:29956-29974).

This can retain stale child entries, fail on a duplicate after parent replacement, publish entries one through N before N+1 fails, and later save a mix of stale/new children. It also never invokes ChildRestoreProjection::admit_complete, which already validates exact unordered child-set equality (🏪️store/🦀️.rs:2978-2997).

## Reusable implementation plan

1. Keep ArtifactPackFiles and ArtifactTextFiles explicitly parent-envelope types. Add first-class DocumentCompositionPack with parent ArtifactPackFiles plus ChildPackEntry[], and DocumentCompositionText with parent ArtifactTextFiles plus ChildTextEntry[]. ChildTextEntry must contain slot, child id, full dialect, and child ArtifactTextFiles. Extend SpaceMember and its generated enum with text-envelope printing/opening; opaque binary child Pack next to parent text is a hybrid archive, not complete text export.

2. Add PluginApp/runtime read/load methods for each composition archive. Keep ReadDocument/LoadDocument as low-level parent-only frames if useful, but do not expose them as a full composed-document export/import. Default media Document must use the new archive or reject a composing document.

3. Implement one VcsArtifactApp candidate-batch replacement shared by Pack/text:
   - parse the candidate parent without resetting live state;
   - derive A::child_restore_projection from its snapshot;
   - parse/bound every entry’s full dialect, form exact child fields, and call admit_complete before member publication;
   - open every M member against the candidate parent/owner stamp in an uncommitted fixed set;
   - atomically publish parent, child registry, graph, and one ChildContentView generation only when the full set succeeds;
   - on parse/factory/projection/cancellation/publication failure, retire candidate members using fixed abort retirement while the old parent/graph/children remain unchanged.

   CompositionCoordinator already gives the intended atomic child-before-parent transaction model (🏪️store/🦀️.rs:20044-20127), and GroupReceipt returns concrete created members (:19513-19545). Factor a batch prepare/commit from open_child’s exact admission/cancellation path (🔌️plugin/🦀️.rs:17931-18057); do not loop over open_child.

4. Preserve explicit bounds. ChildRestoreProjection allows at most 64 refs and 256 bytes per identity/slot field (🏪️store/🦀️.rs:2900-2954). The fixed child registry is 1,024 entries and also bounds slot/id at 256 bytes (🔌️plugin/🦀️.rs:7324-7329, :7532-7545). The manifest must impose the stricter 64-entry composition limit and an explicit aggregate byte limit before retaining decoded strings/Pack buffers. The window fair-partition cursor has the same requirement: cloned ids need the same validated fixed metadata budget rather than an inferred unbounded policy.

5. Use ArtifactView::with_children and its children view, already present at 🔌️plugin/🦀️.rs:7212-7276, at every content-reading command/render boundary. ArtifactChild local_owner is serialization-skipped by design (🏪️store/🦀️.rs:2725-2727) and is not a persistence mechanism.

## Executable tests

1. Framework Pack archive: edited parent plus edited child -> complete archive -> fresh app -> exact parent, child snapshot/history, owner stamp, graph, ChildContentView, and ReadChildren.
2. Framework text archive: same assertion using parent and child text DSL/op files, without binary child-Pack fallback.
3. Atomic replacement: start A plus child A; load B with a bad second child. Assert A parent/map/graph/root unchanged and candidates drain. Then valid B has no stale A entry.
4. Manifest law: omission, extra entry, duplicate, singular duplicate, cross-slot duplicate id, 64/65 entries, 256/257 byte Unicode ids, aggregate-byte limit, cancellation, and parent generation change during staging. Reuse the neutral member-dialect vectors.
5. Presentation restart: actual tile command on non-default source; complete Pack/text archive; clear scratch/local state; fresh editor and viewer see identical tiles/source and child id == target id. Include empty Animation.
6. Program restart: create knowledge/benchmark records through commands; fresh editor/viewer retain catalog rows and accept a follow-up mutation without target-missing; both child ids equal targets.
7. Procedure restart: nested steps plus non-empty seed; fresh editor/viewer retain main/script rows and reorder/add/remove correctly, with Flow/Text snapshots matching converters.
8. Parent-only contract: prove document_pack/document_text preserve only parent handles and name this behavior parent-only, preventing a caller from treating it as full persistence.
9. Native plus TypeScript/Ajv vectors: archive manifest, global child identity, missing-child fault, and exact-set admission must accept/reject identically.

## Validation status

No build or test was run for this audit. Existing green Pack/text/parity tests establish only that parent fields serialize child handles. They do not show that a fresh app has child envelopes, member factories, parent-aware restore authority, or atomic replacement.

