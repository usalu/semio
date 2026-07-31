# layout technology - DSL + OpText notes

Design mirrors draw_rs's 🔖Dsl/🔖OpText style (hand-rolled tokenizer, tuple/list/brace grammar,
field-label markers decoupled from self-delimiting value grammars).

- New `mod dsl` in layout/rs/lib.rs (after `mod operations`), containing:
  - 🔖Dsl: lexer (LayoutTok/lex_layout_dsl), parser (LayoutDslParser + kv/opt helpers), value grammars
    (tuples for bounds/margins/columns/color, id-lists, style-run/override lists), entity grammars
    (paragraphStyle/characterStyle/story/link/layer/frame/parentPage/page/spread), impl DocumentDsl for
    LayoutDocument (EXTENSION="layout").
  - 🔖OpText: parse_page_patch/print_page_patch, parse_frame_patch/print_frame_patch (sparse attr-loop),
    impl OpText for LayoutOperation covering all 7 variants (Pages/Stories/Links wrap CollectionOperation
    with 4 sub-cases each -> pagesAdd/pagesRemove/pagesMove/pagesPatch etc; AddFrame/RemoveFrame/
    PatchFrame; SetCamera).
  - #[cfg(test)] mod tests at the end of `mod dsl` (layout/rs/lib.rs has no unifying `//#region 🧪Tests`
    like draw/rs/lib.rs does -- each mod owns its own local tests submodule, so this follows that
    existing convention instead of inventing one).

- Frame enum (rect/text/image) is self-delimiting by its own kind token, no wrapper "frame" keyword
  needed inside the `{ layer... rect|text|image... }` block (matches draw's DrawLayerNode pattern).
  Op-text embedding still uses a "frame:"/"page:"/"story:"/"link:" field-label marker (decoupled from
  the value's own grammar) for readability, mirroring draw's "layer:" marker on AddLayer.

- Page/TextStory/ImageLink get a `_fields` (no leading keyword) + full (`page`/`story`/`link` keyword
  wrapper) split so the exact same `_fields` grammar is reused standalone in op-text (pagesAdd/
  storiesAdd/linksAdd) without double-consuming the leading keyword.

- characterStyles: Vec<serde_json::Value> (untyped) -> encoded as `characterStyle json="<escaped>"`
  opaque JSON blob, since there's no fixed shape to build grammar around.

- FrameBase/RectFrame/TextFrame/ImageFrame/FrameKind structs in layout/rs/lib.rs are dead code (verified
  via grep, unused anywhere) -- ignored, only the `Frame` enum matters.

- layout/example/sample.layout.json deleted, replaced with layout/example/sample.layout (handcrafted
  text, same parsed content -- JSON's extra unknown fields like ParagraphStyle.spaceBefore/spaceAfter or
  a frame's "styleRuns"/"override" that don't exist on the actual Rust structs were already silently
  dropped by serde before this migration, so they're correctly absent from the new fixture too).

- layout/plugin/rs/lib.rs: LAYOUT_SAMPLE_JSON -> LAYOUT_SAMPLE_TEXT (include_str! of .layout), 
  default_document() now uses LayoutDocument::parse_dsl. Added layout_sample_document_json() JSON bridge
  (mirrors draw plugin's semio_draw_example_json()) since framework's App::example hardcodes
  serde_json::from_str on document_json -- framework/plugin/rs/lib.rs is off-limits so this bridge stays.

- layout/rs/lib.rs export mod's 3 tests (png/pdf/package-zip) switched from
  include_str!(".../sample.layout.json") + parse_layout_document to the DSL fixture + parse_dsl;
  export_package_zip still needs a JSON string (it literally writes document.json into the zip), so it's
  fed `serde_json::to_string(&doc)` from the parsed DSL doc instead of raw file bytes.

- layout/manifest/sample.manifest.json "fixture" field updated to "sample.layout" (static metadata, not
  read by any Rust code -- checked via grep).
