# 📓️ W3-STDIO-CASES-2 — pdf 1.7 encoder, docx base judge, semio drawing (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W3-STDIO-CASES-2, 2026-09-30.

- **Brief:** fix at the root the red stdio cases `📓️w2-w-document-report.md` §4 routed here: pdf 1.7 base and its six
  conformance subsets (§4.1), docx base (§4.2), semio drawing (§4.3). Coordinator additions: align the docx
  whole-package `set-snapshot` scenarios with the office pattern (`mutate-set-snapshot`/`inverse-set-snapshot`).
- **Contract:** `📋️design.md` §11 (evidence rule), `📌️important/📝️.md` (fleet rules).

## 1. Outcome

_Case table filled in §5 from the runs._

## 2. pdf 1.7 — which side was wrong, and the fix

### 2.1 Diagnosis

`PdfSnapshot` carries the document twice: the typed lanes (pages, info, catalog lanes, resources …) that typed
mutations edit, and the retained COS carrier (`objects`, `trailer`) that the COS-level mutations — base
`insert/remove/set-object`, `set/remove-dict-entry`, `set/remove-trailer-entry` and all 76 graph-editing leaves of the
six conformance subsets — edit. The writer (`encode_pdf_with`) had one rule: if the carrier still lifts to the typed
lanes, write it verbatim; otherwise **regenerate every typed-lane-owned object from the typed lanes**. Three defects
followed, all in the product:

1. **A COS edit was overwritten by a stale typed lane.** The graph edit changed `objects`, the typed lanes were not
   told, so the lift differed, the writer regenerated from the stale lanes, and the edit vanished (#145 OpenAction,
   the catalog's `/PageMode`/`/Outlines`, #3015; in the subsets MarkInfo, StructTreeRoot, Lang, ViewerPreferences,
   AcroForm fields, OutputIntents, TrimBox, DPartRoot).
2. **A typed edit re-stated the whole catalog.** Any moved lane regenerated everything: `/Names /Dests` inlined,
   `/OpenAction /Type /Action` added — two differences on every page/info row.
3. **Trailer entries were dropped.** `decode_pdf` kept only Root/Info/ID; `encode_pdf` wrote `extra: []`, and
   `/ID` came from the typed lane only.

The subject side was wrong in all three; the lopdf oracle is an in-place editor and was right. A fourth red —
`inverse-remove-page`/`-append-page-content`/`-set-page-content` — was the **oracle** being lossy (its undo rebuilt a
page from its `Tj` text alone; the thesis sets type with `TJ`).

### 2.2 Fix (product)

- **Graph edits carry the lanes they move** — `io::carry_graph_edit(base, &mut next)`: lifts the graph before and
  after the edit and gives every lane whose reading moved the edited graph's reading; untouched lanes (and a typed
  edit pending in another lane) are kept. Wired into:
  - `diff::diff_graph_edit(base, graph)` — the 7 base COS leaves wrap their sparse builder diff with it;
  - `conformance_support::graph_edit_diff(base, next)` — all 76 graph-editing conformance leaves (the 3
    `set-info-*` leaves edit the typed `info` lane and stay as they were).
- **Incremental reconciliation in the writer** — `io::reconcile` (region `🔖️Encode`):
  - a graph that spells every typed lane is written as it stands (trailer entries included);
  - a moved lane is **grafted**: only the objects/entries it owns are re-stated —
    pages: matched by value, a moved page patched in place (only the moved fields' entries, a new content stream
    plus resources bound on a page-own copy when the content needs a binding), new pages lowered fresh, the page tree
    re-stated flat under its root only when the page sequence changed (inherited attributes materialized);
    info: re-stated in place; catalog lanes (outlines, named destinations → `/Names /Dests`, page labels, output
    intents, layout/mode/preferences/open action/lang/mark info/metadata, extra entries): only their own entries,
    lowered against the graph's pages and resources (`lower_catalog_standalone`);
  - the grafted graph is read back and grafted again until it spells the typed lanes (index shifts from a structural
    page edit, resource discovery order — the latter kept by binding every typed resource on the page-tree root);
    a round that reads back exactly the lanes it grafted has reached the graft's fixed point (a destination to a page
    index the document no longer has — a regeneration lowers it identically) and is written;
  - only a lane no graft expresses (declared version, embedded files, AcroForm, optional content, a changed resource
    value) regenerates the typed objects wholesale, as before, now with the trailer entries carried;
  - displaced values are collected at the end: objects only they reached leave the file, unrelated orphans stay.
- **Trailer** — `decode_pdf` keeps every trailer entry except the writer's bookkeeping
  (`writer::WRITER_TRAILER_KEYS`, shared with the writer's own filter); every write passes the rest as `extra`.
- **Oracle undo** — the lopdf reference captures a page's operators verbatim (`page_ops`, the wire's generic
  `unknown` `PdfOp`) and re-encodes exactly those; the `regenerates_page_content`/`without_content_operators`
  carve-out is deleted, all three inverses are held to the full projection.

### 2.3 Pinned by

`🚪️io/🧪️tests/🔬️unit/🦀️.rs`, on the committed thesis asset:
- `a_direct_graph_edit_moves_its_typed_lanes_and_survives_the_write` — OpenAction object, catalog `/PageMode`,
  a custom trailer key and a removed `/ID` survive; lanes carried; a lane the edit never reached stays;
- `a_typed_page_and_info_edit_rewrites_only_what_those_lanes_own` — exactly the 4 edited pages + `/Info` change,
  one new content stream, the displaced one collected, catalog byte-identical;
- `a_structural_page_edit_restates_the_page_tree_and_keeps_the_catalog` — move + remove + insert: lanes preserved,
  catalog/OpenAction/Outlines objects untouched, the output is its own fixed point;
- `a_page_removal_keeps_the_catalog_when_a_destination_outruns_the_pages` — the fixed-point rule.

## 3. docx base — which side was wrong

Both reds were the **judge** (jszip probe, mirrored in the Rust supplement's projection), not the writer:

1. **docProps digests.** The snapshot is XML-authoritative: every XML part (docProps included) is a logical
   `XmlDocument`, written by the xml artifact's deterministic writer (LF after the declaration, long start tags
   wrapped). The original carries CRLF and no wrapping; XML-equal, byte-different. Not writer non-determinism —
   the comparison digested raw bytes of XML parts, the exact over-claim the profile already rejects for
   `word/document.xml`. Fix: an XML-bearing part is digested over its parsed logical content (names, attributes by
   name, text, order), any other part over its bytes; content changes still move it (set-part/remove-part rows).
2. **Bold flag.** The row sets `bold:false`; the subject writes `<w:b w:val="0"/>` (explicit switch-off, which the
   model keeps distinct from absent), python-docx drops the toggle. The judge read the PRESENCE of `w:b` as bold.
   ECMA-376 Part 1 §17.17.4: `w:b`/`w:i` are ST_OnOff, `w:val` `0`/`false`/`off` is off; `w:u w:val="none"` is no
   underline. Fixed in the probe and in `run_from_xnode`.

Also: the subject's inverse rows now apply the production inverse (`inverse_docx_mutation`, new bridge) instead of a
whole-document `SetSnapshot(base)` restore; the whole-package scenarios are `mutate-set-snapshot`/
`inverse-set-snapshot` in base, strict and transitional (strict/transitional undo the stamp with the production
inverse via `inverse_docx_strict_mutation`/`inverse_docx_transitional_mutation`).

## 4. semio drawing — which implementation deviated

1. **The census (every mutate row + identity):** the Rust adapter's scene-graph census looked for node kind
   `group-nodes` (the VERB's spelling, since 09-05) while the snapshot wire tag and the schema's `DrawNode` union say
   `group`; `unwrap_or(0)` then tallied every group as a path. The Python census matched the contract. Fix: the Rust
   census keys every node by the snapshot's own tag.
2. **`inverse-unflatten-node`:** production `UnflattenNode::inverse` was a bare `Flatten`, exact only when the
   current node is the flattening of `original`; the grammar admits any node. The Python implementation captured
   the overwritten node. Fix in the vocabulary: the inverse captures the node it overwrites
   (`🎈unflatten-node/↩️inverse`), pinned by `the_undo_restores_a_node_that_was_not_the_flattening`.
   The Python implementation was not touched.

## 5. Verification

_Filled from the runs below._

## 6. Notes for others

_Filled at the end._

## Session 2 — 2026-10-01 (S2-STDIO-A, WP-2 of `📓️resume-evidence.md` §5)

Status: **IN PROGRESS** (started 22:45, usage cut ~23:00, resumed 02:45 under the coordinator's CARGO HOLD, fleet rule 26).
This section is rewritten at every milestone. Scratch: `🗑️generated/s2-stdio-a/`.

### S2.1 Baseline (22:50)

cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, `--under ✏️s/🔌️plugins/🗄️stdio`:
- `schema mutation-inputs`: 1660/1661 inputs of 989 leaves, **1 finding** (kit `set-snapshot`, `labelMissing` at `/snapshot/schema`).
- `schema mutation-payloads`: 1997/1998 rows clean, 989/989 leaves witnessed, **2 findings**, both the wav fixture `🔊️resamples` (WP-3).
- No predecessor S2-STDIO-A edits on disk. Owned files changed after the 14:31 auto-commit are peer sweeps: 18:54 `Cargo.toml`
  `semio-framework-value`, 19:41 the `FromValue` migration, 20:23/20:26 semio editor/kit.

### S2.2 Source work done

| item | change | evidence |
|---|---|---|
| kit `set-snapshot` label | `🧿️semio/…/🧰️kit/🧬️schema/📸️snapshot/🔣️.json` `schema` gets the `x-semio-ui` every other semio subset snapshot carries (`Snapshot schema` / `Snapshot-Schema`, widget `text`) | kit inputs 27/27 of 16 leaves, **0**; payloads 16/16, **0** |
| json `JsonValue` refs | 8 untyped `{"description": "any JSON value"}` nodes in 7 leaves now `$ref` `…/json/rfc8259/base/snapshot.json#/$defs/JsonValue`, each with an en/de `x-semio-ui.description`. The leaves are base `✏️set-member`, `📥️insert-array-element` and `🔢️set-scalar`, plus i-json `➕upsert-member`, `📥insert-array-element`, `📸️set-snapshot` (`JsonSnapshot.value`) and `🌳set-top-level` (`JsonIJsonRoot` array items and `JsonMember.value`). The shared snapshot schema labels `schema`, `lexeme`, `items` and `members` in en/de, and so does `JsonIJsonRoot`. The i-json `set-snapshot` keeps its local `JsonSnapshot`: Rust defaults `value` (`#[value(default)]`) while the base snapshot schema requires it. | json inputs 32/32 of 15 leaves, **0**; payloads 29/29 rows, 15/15 witnessed, **0** (the tagged wire witnesses now validate against the real union) |

### S2.3 Source work done under the hold (02:45–)

| item | change |
|---|---|
| `deserialize_double_option` | `📇️registry/🧬️contract/🦀️.rs` region `🪆️DoubleOption` now owns the one `pub fn deserialize_double_option<T: kernel::FromValue>` (os-kernel re-exports `FromValue`/`DslValue`/`ValueError`). The local copies are deleted and replaced by `use semio_s_artifact_stdio_contract::deserialize_double_option;` in five places: html `🧬️mutations/🦀️.rs` (leaf `🔖set-attribute` sees it through `use super::*`), semio `🏛️model/🧬️mutations/🦀️.rs` (`🧭set-spatial-node`, `🎛️set-element`), and the docx, pptx and xml `🔺️diff/🦀️.rs`. The bcf (`💬️bcf/…/🖊️markup/🧬️mutations/🦀️.rs`, WP-3) and glTF (`🧊️gltf/…/🔺️diff/🦀️.rs`, peer) copies are NOT mine. Each is a one-line swap, because both crates already depend on the contract crate. |
| dead `no-mutation` / `set-snapshot` dialect | `🔮️oracles/📃️document/🦀️.rs` loses 135 lines. In `ooxml`: the `no-mutation` early return, the `set-snapshot {conformanceClass}` arm of `apply_conformance_mutation`, and both arms of `conformance_inverse_spec`. In `pdf_conformance`: the `no-mutation` early return, the `no-mutation`/`set-snapshot {conformance}` arms of `apply_in_place` and `conformance_inverse_spec`, the now-unreachable `stamp` fn and the `conformant_title` profile field, which is also removed from the six pdf 1.7 subset `PROFILE`s. The evidence that these paths were dead: every subset oracle gates on its own `KINDS`, and none lists them; no feature row anywhere in stdio says `conformanceClass` or `stamped`; and the six pdf subsets have no `set-snapshot` leaf. The ooxml whole-package stamp lives on in `stamp_conformance_class` (each subset's `oracle_stamp`). The `markup_or_default` fallbacks stay, because `conformance_arrange` inserts removal targets without markup. `⚖️law/🧪️tests/🔬️unit/🦀️.rs` drops its retired `no-mutation` assertion. The gif/avi/dxf generator `no-mutation-no-op` recipes are WP-3's. |
| manifest rows | `patch-snapshot` (`PatchSnapshot`, outcomes `applied`/`rejected`) is added to csv `✳️any` and json `🧱️base`. `set-snapshot` (`SetSnapshot`, outcomes `applied`/`no-op`/`rejected`, the subset's own oracle requirement, the shape the other 11 semio subsets already carry) is added to semio table, object, text, mesh, graph, drawing, brep and kit. These clear the `test-only-mutation` contract breaches of the last breach file (`⚡️cache/breaches/testing.json`, 10-01 03:41). |
| inline test modules | csv and json `🩹️patch-snapshot`: the inline `mod tests` and the `📝️text` `#[test]` move to `🩹️patch-snapshot/🧪️tests/🧪️compact-patch/🦀️.rs`, mounted `#[cfg(test)] #[path] mod tests;`. Both tests are unchanged; the hex test calls `text::parse`. |
| fixture digests | `🧪️w3-stdio-refresh-fixture-digests.py` refreshes 41 stale `sha256`/`bytes` lines: xml base 1 (the 09-28 recipe moved `set-declaration` to `encoding="utf-8"`), semio base 4 and model 36 (the 09-30 camelCase sweep `mesh_id`→`meshId`, −180 bytes per file). An independent re-hash of all 1684 fixture files of the 54 owned catalogs now finds **0 mismatches**. |
| **xml compare pipeline (root cause)** | The `xml-compare` probe report embeds both full projections of the 92 KB `word/document.xml`. The orchestrator (`spawnCapturedSync`) reads probe stdout through a pipe, and the probe ended with `process.exit(await main())`. A bun reproduction shows that `process.exit()` after `process.stdout.write` to a pipe truncates at exactly **65536 bytes** (files are unaffected), so `JSON.parse` fails and the stage reads "probe report is not an object". Stage 0 (`xml-import`, small report) passed, and the only green row of 14 was `minified-identity-round-trip` (the 747-byte part). Fix: `process.exitCode = await main(…)`, which in the reproduction keeps 0.2/2/8 MB intact with the exit code preserved. Applied to all 19 owned scripts: the xml, svg base and semio mesh probes, the docx probe (whose awaited write callback also measured intact), and the 15 owned generators (svg 1, pdf 9, docx 1, xml 1, semio 3). There is no hang risk: none of the 19 holds a non-`unref`'d timer, server, worker or async child. Re-measured on the live tree, the mesh probe (`manifold-3d`) answers `mesh-validity`/`mesh-compare` and exits in 1 s, and a `brepjs` init exits in 1 s. **22 scripts outside my scope still carry `process.exit(await main`**: the las and gif generators, and the generator + probe pairs of bcf, step, png, jpg, avi, dxf, tiff, obj, gltf and bmp. They stay latent until a report exceeds 64 KiB. They belong to WP-3 or peers, and the generic fix belongs in the test module's capture wrapper (§S2.6). |

Lints after these edits: json and kit both 0/0 (§S2.2). The other edits do not touch leaf schemas or witnesses.

### S2.4a Verification run after the hold lifted (03:01)

| command | result |
|---|---|
| `schema mutation-inputs --under ✏️s/🔌️plugins/🗄️stdio` | **1661/1661 inputs of 989 leaves, 0 findings** (baseline 1) |
| `schema mutation-payloads --under ✏️s/🔌️plugins/🗄️stdio` | 1997/1998 rows, 989/989 witnessed, 2 findings, both wav `🔊️resamples` (WP-3): **0 in WP-2 scope** |
| fixed probe, piped: `bun …/📰️xml/…/🔬️probes/📜️script.ts xml-compare --input <92 873-byte 🧪️ooxml-readme-document> --input <same> \| wc -c` | **1 983 102 bytes** arrive through the pipe (the old exit cut at 65 536), parse as `status ok`, `equal true`, `diffCount 0`, exit 0 |
| gated `cargo check --keep-going -p …-stdio-{contract,html,docx,pptx,xml,semio,csv,json} --target wasm32-wasip2` (02:59–03:01) | **BLOCKED by a peer**: `semio-framework-os-kernel` fails with 27 errors from the in-progress `RecordSpecProducer` dsl refactor (`🗣️dsl/🦀️.rs`, `🗣️dsl/🪟️viewport`, `🏪️store/🦀️.rs`, `🎒️pack/🌱️value`, `🚪️io/🧬️schema/🔗️reference`). No error is in a WP-2 file. Retry when the kernel is green (`check-wasm-1.txt`). |
| same, retry 03:13–03:17 (`check-wasm-2.txt`) | kernel now compiles; **BLOCKED** one crate later: `semio-framework-plugin` has 4 errors (`🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs:145,222`, `🔌️plugin/🦀️.rs:11555`: `store::mounted_pack_rt::RecordSpecProducer` no longer exists). This is the same unfinished peer sweep, which INFRA is completing workspace-wide (status 03:25). Still unchanged at 03:34. |

### S2.4 Verification pending the CARGO HOLD (rule 26)

These are the next commands, run one at a time behind the rustc gate. Results land in §S2.5.

1. `cargo check -p semio-s-artifact-stdio-{contract,html,docx,pptx,xml,semio,csv,json} --target wasm32-wasip2 --message-format=short`.
2. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-stdio-a cargo test -p …-{pdf,docx,xlsx,pptx,zip,semio,json,csv,html,xml} --lib` (incl. `semio_payload_law_*`).
3. The oracle crate `cargo test --manifest-path ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.toml --features oracles --lib`.
4. `zsh T/🧪️w3-stdio-run-cases.sh` (redirected to `🗑️generated/s2-stdio-a/`) for: `📰️mutate-xml-1-0`, `🧾️mutate-pdf-1-7-vt`, `🗄️mutate-pdf-1-7-a`, `🖨️mutate-pdf-1-7-x`, `📐️mutate-pdf-1-7-e`, docx base/strict/transitional, semio drawing, `🔀️mutate-json-rfc8259(-i-json)`, `📊️mutate-csv-rfc4180`, `🎨️mutate-svg-1-1`.
5. `contract exhaustive --owner 🗄️stdio`.

The 02:29 blocker of the pdf subsets (`Cannot find module '../../../🧬️schema/✅️validation/🟦️.ts'` from `📇️catalog/📣️publication/🟦️.ts`) is gone: the module imports cleanly (bun, 02:49).

### S2.6 Notes for others (hand-offs)

1. **Coordinator / test module owner.** Make the capture generic: `CAPTURE_WRAPPER_SOURCE` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`)
   spawns the probe with `stdio: [.., "pipe", "pipe"]`. Handing the child the stdout/stderr FILE descriptors directly (`openSync(spec.stdoutPath, "w")`)
   removes the 64 KiB bun truncation for every probe at once, including the 22 scripts outside WP-2. Not edited: framework file, not mine.
2. **WP-3 (STDIO-B).**
   - The 22 scripts listed in §S2.3 still need the one-line `process.exitCode` fix (or item 1): the las and gif generators, plus the bcf, step, png, jpg, avi, dxf, tiff, obj, gltf and bmp generator + probe pairs.
   - bcf `💬️bcf/…/🖊️markup/🧬️mutations/🦀️.rs` still carries its own `deserialize_double_option`. Replace it with `use semio_s_artifact_stdio_contract::deserialize_double_option;` (the crate already depends on the contract).
   - The gif/avi/dxf generator `no-mutation-no-op` recipes (`📓️resume-evidence.md` §2.4) are still in place.
3. **glTF peer.** `🧊️gltf/…/♾️any/🧬️schema/🔺️diff/🦀️.rs` keeps a local `deserialize_double_option`; the same one-line swap applies.
4. **Central `schema generate` (WP-5).** The json base snapshot and kit snapshot schemas gained `x-semio-ui`, and 7 json leaf schemas changed, so their catalog rows read stale until it runs.
5. **Not converted, with the reason recorded:** the semio cases still run `no-mutation-baseline-*` scenarios (`noMutation` sentinel →
   identity `set-snapshot`) in image, audio, video, animation, document, cad, model, flow, value and presentation. They are live
   identity probes, not dead branches, and each case already has `identity-round-trip`. Removing them is the text lane's recipe step 2,
   applied to 10 cases (features + Rust + Python adapters). That is follow-up work outside this WP's checklist.
6. **Peer layout breaches in owned trees** (`⚡️cache/breaches/testing.json`, 10-01 03:41): inline test modules and `.test.ts` naming in
   `📇️registry/🧬️contract/✏️editing/{🧪️tests,🩹️patch}`, zip/mesh/brep `✏️editor/📬️preparation`, pdf `📸️snapshot/🪶️sqlite`, and `📖️pdf/🦀️.rs`.
   These come from the editing-patch and sqlite peers and are left to their owners.
