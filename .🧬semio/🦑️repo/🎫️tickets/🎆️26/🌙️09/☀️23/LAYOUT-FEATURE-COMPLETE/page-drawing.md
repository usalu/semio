# Page Drawing Ownership, Canvas And Relational Proof

Page Drawing is a canonical typed child, with a distinct layout.page-drawing slot and independent child/target artifact identities. Page Canvas preserves the original viewport coordinates and margins; Document/Content preserves existing ink-fit behavior. This report covers the graph owner's changes, not the parent-owned transport/UI or export integration report.

## Domain Changes

Layout schema root Rust/TypeScript, snapshot child visitor/retirement, exact equality, JSON Schema/GraphQL/proto, native JSON Page codec and graph validator include optional Page.backgroundDrawing. Children validate through their actual Drawing owner. Page children additionally require finite positive Canvas dimensions. Document and Page child IDs and target artifact IDs cannot alias. Whole-document uniqueness uses one metadata set traversal; local assignment guards only inspect related child identity metadata. validatePageDrawingAssignment / validate_page_drawing_assignment is the reusable mutation/CreatePage guard.

page_drawing_child_handle / pageDrawingChildHandle creates independently scoped handles. retarget_page_drawing_child / retargetPageDrawingChild derives both identities from exact UTF-8 length-prefixed page/source identity tuples using the existing content_id/contentId owner; it does not serialize the copied payload. Independent OpenSSL SHA256 witnesses include Unicode and NUL boundaries. Existing content_id uses the first eight digest bytes with an owned prefix; no new hash protocol was invented.

SetPageBackgroundDrawing is operation59. Its local semantic evaluator emits sparse PagePatch background_drawing (missing preserve, null clear, child replace), meaningful inverse only when the emitted diff changes state, and exact child restoration. PagePatch JSON/GraphQL/proto, central apply/absorb/inverse and TS evaluator agree. No-op, invalid slot/dialect/viewport and aliased ownership yield empty inverse. Native laws cover actual central diff/apply/inverse; native execution remains unavailable.

DrawingPlanCursor now takes source Document or Page(index), placement Content or Canvas, and target bounds. Canvas uses full viewport-to-target affine, including unequal axes/rotation. It still measures all text in the first traversal, preserving once-shaped label replay. Borrowed Drawing lower-step/leaf/style accessors are source qualified. The native export JSON cursor serializes original selected child leaves in bounded chunks rather than cold-lowering a whole VectorScene. Existing unrelated JSON fragment caps remain.

The actual Drawing owner has scalar Canvas edit methods under snapshot/🖼️canvas. Neutral EmbeddedChildCodecV1 exposes set_canvas_size / withCanvasSize; Layout uses the page slot and exact scalar owner instead of lowering/rebuilding a scene. Width/height edits retain original unrelated layer/style/byte backing in TypeScript. Native methods change only admitted scalar fields.

## Actual Relational Storage

Drawing SQLite now supports multiple independently owned document roots in one database. project_sqlite_component_owned returns the allocated root ID; reconstruction indexes all roots once, scopes styles/layers by document, resolves geometry transitively from layer roots, rejects cross-component style references, shared geometry/cycles and orphan rows, then returns each typed component. The standalone API still requires its one actual standalone root. No blob/JSON/namespace fallback and no new runtime dependencies were introduced.

Layout layout_drawing_component rows map the exact Drawing root primary key to exactly one document or page foreign key and preserve complete child/target identity. Typed reconstruction checks owner XOR/uniqueness and moves each child to the right owner. The first-party supported SQL profile rejects generated autoindex schema, so domain XOR/uniqueness checks live in the typed admission owner; the table keeps real primary/foreign keys. Native projection/reconstruction forwards original allocation_stage to controlled Drawing decode/encode, retains cancellation prefixes and uses original child retirement.

## Files

- Layout 🧬️schema roots, 📸️snapshot root, 🟰️equality, ✅️validation Rust/TS; root JSON/GraphQL/proto; text snapshot 🪪️native-json TS.
- Layout 🧬️mutations/🖊️set-page-background-drawing native leaf, content/enum mounts and TS types; semantic evaluator; shared semantic fixture/tests; graph native laws.
- Layout 🔺️diff row-patches Rust/TS and schema companions; central TS page applier.
- Layout SQLite snapshot Rust/TS/DDL and indexed rows; actual Drawing SQLite projection/reconstruction Rust/TS and controlled reconstruction prefix.
- Framework neutral io/ports Rust/TS and testkit; Layout drawing-composition Rust/TS plus owned native component laws; Drawing snapshot 🖼️canvas schema/Rust/TS and mounts.
- Scene 🧵️drawing Rust/TS, geometry Rust/TS, existing tests and new 📄️page-canvas fixture/tests/oracles/run entry.
- Export 🔣️json child cursor and cursor mounts; export validation group12; native JSON publication law. Native Page initializer sites add None.
- Ticket page-drawing-check.mjs, page-sqlite-check.mjs and this report; page-drawing-live.md records live integration seam.

## Executed Evidence

Tests were authored before implementation. Three aliased-child witnesses initially failed semantic publication, then passed after the local/shared ownership guard.

Final actual execution on current source:

- Page placement/identity: 370-source strict closure, zero diagnostics; 3 portable cases, 6 grant runs, 24 cancellation cuts, 5 refusals, 3 independent identity cases and 2 Canvas edit laws. Third-party Three matrix and Node/OpenSSL hash oracles.
- Relational Page Drawing: 377-source strict closure, zero diagnostics; 2 snapshots / 5 independent Drawing roots and 6 malformed graph refusals. Actual node:sqlite SQLite C engine roundtrip, foreign-key checks and independent recursive SQL ownership/style/geometry queries.
- Semantic mutation suite: 384-source strict closure, zero diagnostics; 123 portable cases, 51 golden cases, all59 variants, 40 IEEE words and 2 Drawing laws, 68 generated modules. Independent RFC6902 remains behind the owned test oracle.
- Durable combined Page run.ts entry: 381-source strict closure, zero diagnostics.
- Rust syntax scan: 214 files checked; 213 parser-clean. The sole parser refusal is existing async-closure syntax in Drawing SQLite tests row89, unsupported by the installed tree-sitter Rust grammar; no new Page source grammar error. Maximum scanned path196 characters.

Temporary compiler outputs are removed by each runner. Native compile, native laws, native SQLite execution, binary/text operation59 roundtrip and GUI acceptance are UNVERIFIED because the cached repo runner cannot execute current native sources. No raw cargo/build refactor was attempted. Parent owns public verification registration/transport/UI and live integration; page-drawing-live.md identifies remaining live document-only lowering and lossy cold collectors without duplicating a composition owner.
