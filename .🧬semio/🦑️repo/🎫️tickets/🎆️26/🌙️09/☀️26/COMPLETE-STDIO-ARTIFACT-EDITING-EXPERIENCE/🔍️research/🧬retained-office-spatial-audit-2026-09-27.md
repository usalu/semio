# Retained Office And Spatial Audit — 2026-09-27

Read-only audit of the current retained native-edit paths for DOCX base, Semio mesh, and Semio BREP. This report distinguishes source inspection from the small amount of runtime evidence reported by the coordinator. It did not start Cargo, Nx, browser, or link checks.

## Exact Source Locators

The shorthand file labels below resolve to these repository-relative paths:

- Contract editing: "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs"
- Contract details: "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs"
- DOCX editor: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs"
- DOCX preparation: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs"
- DOCX tests: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
- Mesh editor: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs"
- Mesh diff: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🔺️diff/🦀️.rs"
- Mesh snapshot: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/📸️snapshot/🦀️.rs"
- Mesh tests: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
- BREP editor: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs"
- BREP move-vertex diff: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🧬️mutations/📍move-vertex/🔺️diff/🦀️.rs"
- BREP snapshot: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🦀️.rs"
- BREP tests: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

## Findings

### P1 — Custom native work bypasses the raw-command envelope

The shared contract declares a raw-command cap at "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs:773-783". Its default extent encodes the command and rejects an encoded value above that cap. The three custom cursors replace that extent without restoring the equivalent admission:

- DOCX accepts when only "text.len()" is within the cap; the unbounded "revision" remains part of the command at DOCX editor:94-99.
- Mesh accepts arbitrary-length "mesh_id" and "primitive_id" at Mesh editor:96-105.
- BREP accepts an arbitrary-length "vertex_id" at BREP editor:113-122.

The typed action path moves the command directly into "ArtifactRetainedCommandPayload::try_new" without encoding or measuring it at Contract editing:1150-1182. The existing raw cap is only applied to a wire page at Contract editing:1103-1118. An action argument with a large identifier or DOCX revision can therefore be admitted although the declared bounded-first-step raw limit says otherwise.

Put a single encoded-size admission in the typed-job builder before payload construction, or make every custom extent call the shared encoding check. Add registered action tests with each variable-size field over the cap and assert rejection before a job or Store edit is created.

### P1 — Semio native work is mounted, but its Store preparation remains the aggregate fallback

Mesh and BREP opt into the bounded native factory at Mesh editor:179-185 and BREP editor:176-182. Their bounded-native declarations provide "work" but no "preparation_route" at Mesh editor:261-280 and BREP editor:247-261. The default route is "None" at Contract editing:797-806, so the editor-support macro supplies the generic "bounded_config_store_one_item_preparation_factory" at Contract details:1918-1931.

Thus their lookup cursors are cooperative, but the forward/inverse/diff/post Store handoff is still the generic aggregate route. The current owner checkpoint identifies this route as deriving and applying sparse diffs through aggregate vectors. It has no retained snapshot owner comparable to DOCX's "DocxPreparation", so a large mesh or BREP cannot presently claim bounded Store publication, cancellation cleanup, or exact post-snapshot copying.

Add a native preparation route for each subset, with a cursor that owns the selected item, its inverse, the post snapshot, and retirement. Test a large snapshot through registered "set-vertex" with fixed item/byte grants, cancellation at each phase, then publication, undo, and redo.

### P1 — DOCX post-copy can clone thousands of structural owners in one 4 KiB / one-item turn

"DocxPreparation::advance" calls "DocxPostCopy::advance" once for each granted preparation item at DOCX preparation:114-168. The post-copy structural envelope limits a body, paragraph, or table to 4,096 elements at DOCX preparation:347-354, but it does not turn those elements into cursors.

For the target paragraph, "copy_target_paragraph_structure" validates up to 4,096 runs and then clones every run plus its property vector in one call at DOCX preparation:607-628. For a non-target body block, "copy_body" validates the complete block and clones it in one call; "ensure_block_bounded" merely limits aggregate text and nodes at DOCX preparation:630-675. A bounded text budget does not bound the number or heap size of the individual String and Vec allocations performed by those derived clones.

A valid document containing 4,096 short runs or a deeply structured table can therefore make one progress turn do thousands of allocations despite the advertised 4 KiB/one-item grant. Cancellation cannot intervene until that clone ends.

Replace complete structural clones with owned cursors: page body blocks, target paragraph runs, and table/XML nodes should be copied through one structural owner per grant, while every string/byte payload stays in a retained byte/text owner. Add a 4,096-run target and a non-target table fixture that asserts every advance consumes no more than its granted item/byte budget and can close from every phase.

### P1 — RetainedTextCopy::take_partial can construct an invalid String

"RetainedTextCopy::advance" copies arbitrary byte counts at Contract editing:820-839, but its public "take_partial" uses "String::from_utf8_unchecked" at Contract editing:854-858. After reservation, advancing "🧾" by one byte and then calling "take_partial" violates the unsafe function's UTF-8 precondition. The resulting invalid String is undefined behavior.

No direct Stdio call site of "RetainedTextCopy::take_partial()" was found in this audit, so this is latent rather than evidence of the DOCX close path executing it. It remains a public unsafe-in-effect API.

Return bytes for partial text, or retain only a preceding UTF-8 boundary before constructing a String. Add interruption tests at every byte of a multi-byte scalar; ASCII-only paging tests cannot prove this invariant.

### P2 — Mesh and BREP target resolution accepts the first duplicate identifier

The mesh cursor stops as soon as it sees the first matching mesh and the first matching primitive at Mesh editor:114-146; BREP stops at the first matching vertex at BREP editor:124-144. Neither cursor scans for a second matching target before emitting. The direct mesh and BREP lookup helpers are also first-match operations at Mesh diff:482-488 and BREP move-vertex diff:9-23.

The snapshot shapes expose plain vectors (Mesh snapshot:35-62,105-117; BREP snapshot:260-280) and their JSON decoders perform only "pack::from_json_str" (Mesh snapshot:637-639; BREP snapshot:1317-1319). An ambiguous snapshot can therefore reach native work, which silently chooses one target instead of refusing the operation before any mutation is emitted.

Validate uniqueness at snapshot admission or have the retained lookup scan to completion and reject duplicates. Add registered-operation tests for duplicate mesh IDs, duplicate primitive IDs within a mesh, and duplicate BREP vertex IDs; each should fault with the snapshot and history unchanged.

## Confirmed Source Properties

- The factory/host assembly is present. The shared macro registers the native factory before the shared snapshot factory and dispatches a matching native tool to "build_bounded_native_edit_tool_job" at Contract details:1880-1908.
- "NativeEditPreparationRoute" selects its recognized factory in both preflight and begin, never falling back after a recognized native mutation is refused at Contract editing:967-1016. DOCX recognizes exactly "SetRunText" at DOCX preparation:16-18.
- DOCX source correctly rechecks lane, operation, generation, base revision, actor, target shape, replacement size, and original text size before it owns the request at DOCX preparation:24-70. It pages inverse and post-copy owners, then creates the exact one-forward/one-inverse Store edit under the live authority and hands it to the sealer at DOCX preparation:141-203. Its cancel/close path also retires retained owners at DOCX preparation:218-245. These are source findings only; the structural-cloning issue above limits their boundedness claim.
- Stale and missing targets fault in the retained cursors: DOCX checks the addressed page/run before work at DOCX editor:94-99 and again in preparation at DOCX preparation:42-51; mesh and BREP fault on a missing selected item at Mesh editor:114-135 and BREP editor:124-137.
- The retained native no-op branches emit no mutation for equal values in DOCX at DOCX editor:121-128, mesh at Mesh editor:136-140, and BREP at BREP editor:138-143. The "2" work-item capacity is the forward-plus-inverse Store row count, not the number of replay steps.
- The BREP binary decoder no longer forms a potentially overflowing "4 + length + 24"; it first subtracts the fixed 28-byte framing length and compares the declared length at BREP editor:58-77. The focused regression is at BREP tests:53-58.

## Evidence Limits And Required Runtime Proof

The coordinator reported one completed native DOCX run: exactly two "post_copy" tests passed and 120 tests were skipped. Those tests establish paged OPC sibling copying and cancellation cleanup in isolation. They do not execute the registered DOCX action, its factory selection, or Store sealing.

The registered DOCX route run is pending in "🗑️generated/docx-registered-route-native-1.log" at the time of this audit. No native runtime result was available here for either Semio registered "set-vertex" test, the BREP overflow regression, no-op history behavior, duplicate targets, raw-cap rejection, or large-snapshot Store publication. The registered test sources are useful intended-law coverage—DOCX tests:103-138, Mesh tests:38-71, BREP tests:98-121—but this audit does not treat unexecuted sources as runtime proof.

The repository-goals MCP resource could not be read because its server closed the handshake. No ticket lifecycle action was taken because this audit belongs to the already-active ticket.
