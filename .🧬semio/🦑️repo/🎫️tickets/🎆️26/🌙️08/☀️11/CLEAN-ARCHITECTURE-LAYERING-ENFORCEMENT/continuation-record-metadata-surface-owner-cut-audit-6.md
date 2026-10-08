# Record Metadata and Surface Canonical Owner Cut

Read-only source design audit. Parent reports closed Value5 all180 pass and Record7 actual92 pass/7 fail with copy demand over3; this report does not independently join those terminals.

## Defining metadata authority

Writer order Vec<usize> contains only field indices constructed at encoding90–94. Intrinsic path Vec<usize> contains traversal indices pushed155/popped153. Intrinsic frames contain Copy traversal state, not domain payload. These are metadata owned by RetainedRecordWriter, separate from original RecordValue/source. A defining writer cursor can clear their logical lengths under one item grant with Advanced (no copied/released bytes), retaining the original backing until a subsequent exact whole allocation release. Copy metadata clear has constant destructor work; avoid generic domain Vec scalar-copy requirements here.

Perform that transition directly in the writer cursor: for metadata positions, nonempty clears and leaves position unchanged; reserved-empty queries capacity*sizeof(element) as release demand; admitted exact release drops only backing and advances; empty zero-capacity advances0. No extra Box/Deferred constructor is needed for these three original metadata Vecs. Update next_work0, next_birth0, next_close demand for the selected metadata state; release category must reflect actual backing. Reject undergrant before clear/drop/position movement; zero-items never clear. Keep scalar u32 Vector generic demand4 unchanged.

Proof should compare the original pointers/capacities before/after clear and denied one-below release, actual allocator zero/whole grants, partial cancellation at each metadata field, and terminal writer Box separate release. Original table64 1MB and copy3 laws must pass unchanged, plus all198 finite-family/5 writer-position receipts. No generic Collection weakening is justified by metadata semantics.

## Active Surface cut and API relation

Current node graph255/258/261/264/267/270/273/276 calls store scene_field_json_text for preview_off, lod, controls, clusters, status, computing, capabilities, host_snapshot JSON. Binary node graph474 calls store decode_wire_value then decode_pack_value;475 calls store dsl_value_to_json. These active runtime import sites are the smallest payload interpretation cut, while DAG host and interaction owners remain separate.

Current store scene_field_json_text6890 recognizes canonical pack-base64 prefix, decodes pack value, then serializes its JSON projection; ordinary strings pass through. decode_pack_value6806 uses General Record one-field Value bridge spec, General Pack document decode, and extracts DslValue. decode_wire_value6823 uses containerless same bridge. These need General Pack's defining value bridge owner, not a facade depending back on OS Store. General JSON package already depends only on Value and exports its owned JSON grammar; retain that direction rather than making its syntax core depend on full binary Pack/Record. Compose the envelope interpretation at General Pack or a properly owned Pack presentation facet; JSON projection itself belongs to General JSON/Value.

External relation is explicit: Surface consumes first-party decoded DslValue/owned JSON/error/control contracts; independent Serde JSON is test oracle. NodeGraphError::Pack must use canonical Pack error/refusal and preserve both wire-first and pack-fallback diagnostics. Preserve eight optional field values, canonical pack prefix/base64 acceptance, raw JSON string semantics, numeric variants, graph scene hostDocument content, cancellation and input/owned limits. Keep current graph layout/hit-test and Kernel interaction APIs until their actual defining-owner extraction; no whole Surface OS dependency removal claim follows from this payload slice.

Schema contract must be in General defining owner modules outside fixture collections. Existing packed/plain scene corpus and independent decode/JSON expectations should qualify the new owner before replacing every active Surface call, with no legacy Store forwarding shim.
