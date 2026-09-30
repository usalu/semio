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
