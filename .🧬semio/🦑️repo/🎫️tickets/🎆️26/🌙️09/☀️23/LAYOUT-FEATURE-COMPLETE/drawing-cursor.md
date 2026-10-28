# Borrowed Drawing Scene Cursor

The child codec now supplies a bounded source-borrowing lowering port. It does not decode or clone a complete Drawing snapshot, VectorScene, text value or image payload. The real Layout/Drawing composition and the framework testkit both implement the new required native method; the real TypeScript composition implements the corresponding required lowerStep contract.

## API And Consumer Guidance

Native `semio_framework_io_ports` exports VectorSceneCursor, VectorSceneGrant, VectorSceneReceipt, VectorSceneStep, VectorSceneEvent, VectorStyleRef and VectorImageBytes. `EmbeddedChildCodecV1::lower_step(content, cursor, grant)` delegates to the actual bound child owner. Layout model `background_drawing_step(snapshot,cursor,grant)` loans that seam from the retained background child. The older whole-scene lower method still serves existing synchronous consumers; this port does not silently call it.

Events are Canvas, Style, BeginLayer, BeginGroup, EndGroup, BeginPath, Segment, EndPath, Text, Image, EndLayer and Done. Each Segment carries only one fixed-size command. All names/text/image data borrow the retained source. Image byte views support intrinsic binary backing or native JSON array backing, with individual checked octet access. Layer visibility is emitted even for hidden layers so the consumer decides visibility policy while retaining source order.

The cursor owns only numeric group ancestry. A turn revisits those source records within maximum_source_items and maximum_depth, and records actual visits. Growing ancestry requires independently authored maximum_copy_bytes, maximum_capacity_bytes and maximum_release_bytes; replacing the backing uses a separate new allocation, copies the numeric ancestry and releases the old block. The actual receipt remains observable through last_receipt even on refusal. Consumers must keep the source immutable across loans. Source indexing is proportional to active depth, rather than document size, and is charged on each reborrow.

The active `current_leaf(content,grant)` method reborrows a path style, full text descriptor or image descriptor without advancing. Text/Image leaves remain addressable until the next advance, so a consumer can perform incremental style search or UTF8 text pagination while retaining only numeric offsets. Paths remain addressable through their segment turns. A separate style cursor can replay Canvas/Style records without retaining style names or an entire style table.

On completion, refusal or cancellation, quote `next_release_bytes()`, obtain original caller release authority and call `close_one(maximum_bytes)`. It refuses an insufficient release grant without clearing ancestry. `terminal_is_empty()` proves no cursor-owned backing remains. The original retained document remains with its parent owner and is never retired by this borrowed view. TypeScript tracks explicit Float64Array ancestry custody; these tests do not assert JavaScript GC deallocation timing.

Root PDF integration should replace whole `build_unshaped_export_background` planning with this cursor and a Layout-owned transform/style/geometry adapter. That adapter must fund any transform/primitive backing separately, use current_leaf for long text/style lookup, borrow image bytes, and close its own backing plus this cursor. Root owns `plan_pdf_one`; that PDF file was not modified here. STEP consumer integration is owned by the interchange agent. Neither integration is claimed complete by this report.

## Files

- Framework `🧰️framework/🔨️modules/🚪️io/🔌️ports/🧵️lower`: language-neutral event/grant schema, native cursor, canonical TypeScript cursor, three portable fixtures, TypeScript test and test-only independent oracle. Existing ports roots and slot mock tests mount the required method.
- Layout schema root: background_drawing_step and native shared cursor-law mount; `🧬️schema/🧪️tests/🧵️drawing/🦀️.rs` consumes the same portable fixtures.
- Layout `🔨️modules/🧩️drawing-composition`: real native/TypeScript binding methods and a native child borrowing/retirement law.
- Canonical Drawing native JSON boundary: omitted styles/layers now receive the schema-declared empty defaults; explicit null remains refused. The empty portable scene exposed this defect.
- Ticket `drawing-cursor-check.mjs`: retained verification input, compiles actual source into owned temporary generated modules and removes them.

## Executed Evidence

The actual TypeScript cursor, canonical Drawing native JSON boundary and real bound composition execute successfully: strict diagnostics 0 over 225 actual source files; three shared scenes, 18 cancellation/completion cuts, three malformed scenes and 4097 wide path segments. Tests compare event order, text/image values, source image identity, repeated active-leaf views, input immutability, underfunded source/copy/capacity/release refusals, exact retained ancestry receipts and terminal emptiness. Flat wide geometry retains zero ancestry backing.

The test-owned external interface wraps Ajv against the authoritative Drawing source schema and the new neutral event/grant schema, lodash collection projection and Three affine group geometry. Actual emitted descriptors pass the independent protocol schema; the independent source oracle confirms text/image/style outputs and the transformed point.

Seven changed Rust files parse with web-tree-sitter without grammar errors; maximum native path 132 characters. Native shared laws and real composition borrowing laws are authored, but unrun. There was no native compilation, allocation observer execution or fresh repository runner execution.

Canonical mutation regression rerun after this port: 373 actual sources, full strict diagnostics 0; 96 portable semantic cases, 51 goldens, all 58 operations, 40 word cases and two drawing semantic-owner cases pass.


## Planner Continuation

The Layout-owned two-pass adapter is now mounted and independently executed; see `drawing-planning.md`. The lower cursor additionally exposes current_style/currentStyle, and the parent model exposes background_drawing_leaf/background_drawing_style. Latest lower-port evidence remains three scenes,18cuts,three refusals,4097segments,strict225-source0.
