# PDF 1.7 Native Semantic Fields

Handcrafted provider inventory, extracted from the actual typed source for review. No persistence schema or provider is generated from this inventory.

```rust
pub struct ObjRef {
    pub num: u32,
    pub gen: u16,
}

pub struct PdfDictEntry {
    pub key: String,
    pub value: PdfObject,
}

pub struct PdfPredictor {
    pub predictor: u32,
    pub colors: u32,
    pub bits_per_component: u32,
    pub columns: u32,
}

pub struct PdfCcittParameters {
    pub k: i32,
    pub columns: u32,
    pub rows: u32,
    pub black_is_1: bool,
    pub encoded_byte_align: bool,
    pub end_of_line: bool,
    pub end_of_block: bool,
    pub damaged_rows_before_error: u32,
}

pub enum PdfStreamFilter {
    Flate { predictor: Option<PdfPredictor> },
    Lzw { predictor: Option<PdfPredictor>, early_change: bool },
    AsciiHex,
    Ascii85,
    RunLength,
    Dct { color_transform: Option<u32> },
    Jpx,
    Ccitt { parameters: PdfCcittParameters },
    Jbig2 { globals: Option<Vec<u8>> },
    Crypt { name: Option<String> },
}

pub struct PdfDecimal {
    pub negative: bool,
    pub coefficient: String,
    pub scale: u32,
}

pub enum PdfObject {
    Null,
    Bool(bool),
    Int(i64),
    Real(PdfDecimal),
    Str(Vec<u8>),
    Name(String),
    Array(Vec<PdfObject>),
    Dict(Vec<PdfDictEntry>),
    Ref(ObjRef),
    Stream {
        dict: Vec<PdfDictEntry>,
        data: Vec<u8>,
        filters: Vec<PdfStreamFilter>,
    },
}

pub struct PdfIndirectObject {
    pub id: ObjRef,
    pub value: PdfObject,
}

pub enum PdfTextString {
    Text { text: String },
    Codes { bytes: Vec<u8> },
}

pub enum PdfTextArrayItem {
    Text { text: String },
    Codes { bytes: Vec<u8> },
    Adjust { amount: f64 },
}

pub enum PdfPropertyList {
    Named { name: String },
    Inline { entries: Vec<PdfDictEntry> },
}

pub struct PdfInlineImage {
    pub width: u32,
    pub height: u32,
    pub bits_per_component: u32,
    pub color_space: Option<PdfColorSpace>,
    pub image_mask: bool,
    pub decode: Vec<f64>,
    pub interpolate: bool,
    pub filters: Vec<PdfStreamFilter>,
    pub data: Vec<u8>,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfLineCap {
    Butt,
    Round,
    Square,
}

pub enum PdfLineJoin {
    Miter,
    Round,
    Bevel,
}

pub enum PdfOp {
    // 🎛️ General graphics state (Table 57)
    SetLineWidth { width: f64 },
    SetLineCap { cap: PdfLineCap },
    SetLineJoin { join: PdfLineJoin },
    SetMiterLimit { limit: f64 },
    SetDash { array: Vec<f64>, phase: f64 },
    SetRenderingIntent { intent: String },
    SetFlatness { flatness: f64 },
    SetExtGState { name: String },
    // 🧷 Special graphics state
    Save,
    Restore,
    Transform { matrix: PdfMatrix },
    // ✏️ Path construction (Table 59)
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64 },
    CurveToInitial { x2: f64, y2: f64, x3: f64, y3: f64 },
    CurveToFinal { x1: f64, y1: f64, x3: f64, y3: f64 },
    ClosePath,
    Rectangle { x: f64, y: f64, width: f64, height: f64 },
    // 🎨 Path painting (Table 60)
    Stroke,
    CloseStroke,
    Fill,
    FillEvenOdd,
    FillStroke,
    FillStrokeEvenOdd,
    CloseFillStroke,
    CloseFillStrokeEvenOdd,
    EndPath,
    // ✂️ Clipping (Table 61)
    Clip,
    ClipEvenOdd,
    // 🔤 Text objects, state, positioning, showing (Tables 107, 105, 108, 109)
    BeginText,
    EndText,
    SetCharSpacing { spacing: f64 },
    SetWordSpacing { spacing: f64 },
    SetHorizontalScale { scale: f64 },
    SetLeading { leading: f64 },
    SetFont { name: String, size: f64 },
    SetTextRenderingMode { mode: u32 },
    SetTextRise { rise: f64 },
    MoveText { tx: f64, ty: f64 },
    MoveTextSetLeading { tx: f64, ty: f64 },
    SetTextMatrix { matrix: PdfMatrix },
    NextLine,
    ShowText { text: PdfTextString },
    ShowTextArray { items: Vec<PdfTextArrayItem> },
    NextLineShowText { text: PdfTextString },
    NextLineShowTextSpaced { word_spacing: f64, char_spacing: f64, text: PdfTextString },
    // 🔠 Type 3 glyph metrics (Table 113)
    SetGlyphWidth { wx: f64, wy: f64 },
    SetGlyphWidthAndBox { wx: f64, wy: f64, llx: f64, lly: f64, urx: f64, ury: f64 },
    // 🌈 Colour (Table 74)
    SetStrokeColorSpace { name: String },
    SetFillColorSpace { name: String },
    SetStrokeColor { components: Vec<f64> },
    SetStrokeColorN { components: Vec<f64>, pattern: Option<String> },
    SetFillColor { components: Vec<f64> },
    SetFillColorN { components: Vec<f64>, pattern: Option<String> },
    SetStrokeGray { gray: f64 },
    SetFillGray { gray: f64 },
    SetStrokeRgb { r: f64, g: f64, b: f64 },
    SetFillRgb { r: f64, g: f64, b: f64 },
    SetStrokeCmyk { c: f64, m: f64, y: f64, k: f64 },
    SetFillCmyk { c: f64, m: f64, y: f64, k: f64 },
    // 🖼️ Shading, XObjects, inline images (Tables 77, 87, 92)
    PaintShading { name: String },
    PaintXObject { name: String },
    InlineImage { image: PdfInlineImage },
    // 🏷️ Marked content (Table 320)
    MarkedContentPoint { tag: String },
    MarkedContentPointWithProperties { tag: String, properties: PdfPropertyList },
    BeginMarkedContent { tag: String },
    BeginMarkedContentWithProperties { tag: String, properties: PdfPropertyList },
    EndMarkedContent,
    // 🧯 Compatibility (Table 32)
    BeginCompatibility,
    EndCompatibility,
    // 🧳 An operator this codec does not know, retained with its operands so nothing is dropped.
    Unknown { operator: String, operands: Vec<PdfObject> },
}

pub enum PdfColorSpace {
    DeviceGray,
    DeviceRgb,
    DeviceCmyk,
    CalGray { white_point: [f64; 3], black_point: Option<[f64; 3]>, gamma: Option<f64> },
    CalRgb { white_point: [f64; 3], black_point: Option<[f64; 3]>, gamma: Option<[f64; 3]>, matrix: Option<[f64; 9]> },
    Lab { white_point: [f64; 3], black_point: Option<[f64; 3]>, range: Option<[f64; 4]> },
    IccBased { components: u32, profile: Vec<u8>, alternate: Option<Box<PdfColorSpace>>, range: Option<Vec<f64>> },
    Indexed { base: Box<PdfColorSpace>, hival: u32, lookup: Vec<u8> },
    Separation { name: String, alternate: Box<PdfColorSpace>, tint_transform: PdfFunction },
    DeviceN { names: Vec<String>, alternate: Box<PdfColorSpace>, tint_transform: PdfFunction, attributes: Option<Vec<PdfDictEntry>> },
    Pattern { base: Option<Box<PdfColorSpace>> },
    Named { name: String },
}

pub enum PdfFunction {
    Sampled { domain: Vec<f64>, range: Vec<f64>, size: Vec<u32>, bits_per_sample: u32, order: Option<u32>, encode: Option<Vec<f64>>, decode: Option<Vec<f64>>, samples: Vec<u8> },
    Exponential { domain: Vec<f64>, range: Option<Vec<f64>>, c0: Vec<f64>, c1: Vec<f64>, n: f64 },
    Stitching { domain: Vec<f64>, range: Option<Vec<f64>>, functions: Vec<PdfFunction>, bounds: Vec<f64>, encode: Vec<f64> },
    PostScript { domain: Vec<f64>, range: Vec<f64>, code: String },
    Array { functions: Vec<PdfFunction> },
}

pub enum PdfShadingKind {
    FunctionBased { domain: Option<[f64; 4]>, matrix: Option<PdfMatrix>, function: PdfFunction },
    Axial { coords: [f64; 4], domain: Option<[f64; 2]>, function: PdfFunction, extend: [bool; 2] },
    Radial { coords: [f64; 6], domain: Option<[f64; 2]>, function: PdfFunction, extend: [bool; 2] },
    Mesh { shading_type: u32, bits_per_coordinate: u32, bits_per_component: u32, bits_per_flag: Option<u32>, vertices_per_row: Option<u32>, decode: Vec<f64>, function: Option<PdfFunction>, data: Vec<u8> },
}

pub struct PdfShading {
    pub id: String,
    pub color_space: PdfColorSpace,
    pub kind: PdfShadingKind,
    pub background: Option<Vec<f64>>,
    pub bbox: Option<PdfRect>,
    pub anti_alias: bool,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfPatternKind {
    Tiling { paint_type: u32, tiling_type: u32, bbox: PdfRect, x_step: f64, y_step: f64, content: Vec<PdfOp> },
    Shading { shading: String, ext_g_state: Option<String> },
}

pub struct PdfPattern {
    pub id: String,
    pub matrix: PdfMatrix,
    pub kind: PdfPatternKind,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfSoftMask {
    None,
    Alpha { group: String, transfer: Option<PdfFunction> },
    Luminosity { group: String, backdrop: Option<Vec<f64>>, transfer: Option<PdfFunction> },
}

pub struct PdfExtGState {
    pub id: String,
    pub line_width: Option<f64>,
    pub line_cap: Option<PdfLineCap>,
    pub line_join: Option<PdfLineJoin>,
    pub miter_limit: Option<f64>,
    pub dash: Option<(Vec<f64>, f64)>,
    pub rendering_intent: Option<String>,
    pub overprint_stroke: Option<bool>,
    pub overprint_fill: Option<bool>,
    pub overprint_mode: Option<u32>,
    pub font: Option<(String, f64)>,
    pub blend_mode: Option<Vec<String>>,
    pub soft_mask: Option<PdfSoftMask>,
    pub stroke_alpha: Option<f64>,
    pub fill_alpha: Option<f64>,
    pub alpha_is_shape: Option<bool>,
    pub stroke_adjust: Option<bool>,
    pub flatness: Option<f64>,
    pub smoothness: Option<f64>,
    pub text_knockout: Option<bool>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfNamedColorSpace {
    pub name: String,
    pub color_space: PdfColorSpace,
}

pub struct PdfNamedProperties {
    pub name: String,
    pub entries: Vec<PdfDictEntry>,
}

pub enum PdfBaseEncoding {
    Standard,
    WinAnsi,
    MacRoman,
    MacExpert,
}

pub struct PdfEncodingDifference {
    pub code: u32,
    pub glyph: String,
}

pub struct PdfSimpleEncoding {
    pub base: Option<PdfBaseEncoding>,
    pub differences: Vec<PdfEncodingDifference>,
}

pub struct PdfFontDescriptor {
    pub font_name: String,
    pub flags: u32,
    pub font_bbox: PdfRect,
    pub italic_angle: f64,
    pub ascent: f64,
    pub descent: f64,
    pub cap_height: f64,
    pub stem_v: f64,
    pub stem_h: Option<f64>,
    pub x_height: Option<f64>,
    pub leading: Option<f64>,
    pub avg_width: Option<f64>,
    pub max_width: Option<f64>,
    pub missing_width: Option<f64>,
    pub font_family: Option<String>,
    pub font_stretch: Option<String>,
    pub font_weight: Option<f64>,
    pub char_set: Option<String>,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfFontProgram {
    Type1 { data: Vec<u8>, length1: u32, length2: u32, length3: u32 },
    TrueType { data: Vec<u8> },
    Cff { data: Vec<u8> },
    CidCff { data: Vec<u8> },
    OpenType { data: Vec<u8> },
}

pub enum PdfToUnicodeMapping {
    Char { code: u32, text: String },
    Range { low: u32, high: u32, text: String },
}

pub struct PdfToUnicode {
    pub byte_width: u32,
    pub mappings: Vec<PdfToUnicodeMapping>,
}

pub struct PdfCodespaceRange {
    pub byte_width: u32,
    pub low: u32,
    pub high: u32,
}

pub enum PdfCidMapping {
    Char { code: u32, cid: u32 },
    Range { low: u32, high: u32, cid: u32 },
}

pub struct PdfEmbeddedCMap {
    pub name: String,
    pub vertical: bool,
    pub codespace: Vec<PdfCodespaceRange>,
    pub mappings: Vec<PdfCidMapping>,
    pub use_cmap: Option<String>,
}

pub enum PdfCMap {
    Predefined { name: String },
    Embedded { cmap: PdfEmbeddedCMap },
}

pub struct PdfCidSystemInfo {
    pub registry: String,
    pub ordering: String,
    pub supplement: u32,
}

pub struct PdfCidWidthRun {
    pub start_cid: u32,
    pub widths: Vec<f64>,
}

pub struct PdfCidVerticalRun {
    pub start_cid: u32,
    pub metrics: Vec<[f64; 3]>,
}

pub enum PdfCidToGid {
    Identity,
    Map { data: Vec<u8> },
}

pub struct PdfCidFont {
    pub true_type: bool,
    pub base_font: String,
    pub system_info: PdfCidSystemInfo,
    pub descriptor: PdfFontDescriptor,
    pub default_width: f64,
    pub widths: Vec<PdfCidWidthRun>,
    pub default_vertical: Option<[f64; 2]>,
    pub vertical_metrics: Vec<PdfCidVerticalRun>,
    pub cid_to_gid: Option<PdfCidToGid>,
    pub program: Option<PdfFontProgram>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfCharProc {
    pub name: String,
    pub content: Vec<PdfOp>,
}

pub enum PdfFontKind {
    Type1 { base_font: String, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, descriptor: Option<PdfFontDescriptor>, program: Option<PdfFontProgram> },
    TrueType { base_font: String, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, descriptor: Option<PdfFontDescriptor>, program: Option<PdfFontProgram> },
    Type3 { font_matrix: PdfMatrix, font_bbox: PdfRect, encoding: PdfSimpleEncoding, first_char: u32, widths: Vec<f64>, char_procs: Vec<PdfCharProc>, descriptor: Option<PdfFontDescriptor> },
    Type0 { base_font: String, cmap: PdfCMap, descendant: PdfCidFont },
}

pub struct PdfFont {
    pub id: String,
    pub kind: PdfFontKind,
    pub to_unicode: Option<PdfToUnicode>,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfImageCodec {
    Raw,
    Dct { color_transform: Option<u32> },
    Jpx,
    Ccitt { parameters: PdfCcittParameters },
    Jbig2 { globals: Option<Vec<u8>> },
}

pub enum PdfImageMask {
    Stencil { image: String },
    ColorKey { ranges: Vec<u32> },
}

pub struct PdfImage {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub color_space: Option<PdfColorSpace>,
    pub bits_per_component: u32,
    pub image_mask: bool,
    pub decode: Vec<f64>,
    pub interpolate: bool,
    pub codec: PdfImageCodec,
    pub data: Vec<u8>,
    pub soft_mask: Option<String>,
    pub soft_mask_in_data: Option<u32>,
    pub mask: Option<PdfImageMask>,
    pub matte: Option<Vec<f64>>,
    pub intent: Option<String>,
    pub optional_content: Option<String>,
    pub struct_parent: Option<u32>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfTransparencyGroup {
    pub color_space: Option<PdfColorSpace>,
    pub isolated: bool,
    pub knockout: bool,
}

pub struct PdfFormXObject {
    pub id: String,
    pub bbox: PdfRect,
    pub matrix: PdfMatrix,
    pub content: Vec<PdfOp>,
    pub group: Option<PdfTransparencyGroup>,
    pub optional_content: Option<String>,
    pub struct_parent: Option<u32>,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfDestinationFit {
    Xyz { left: Option<f64>, top: Option<f64>, zoom: Option<f64> },
    Fit,
    FitHorizontal { top: Option<f64> },
    FitVertical { left: Option<f64> },
    FitRectangle { rect: PdfRect },
    FitBoundingBox,
    FitBoundingBoxHorizontal { top: Option<f64> },
    FitBoundingBoxVertical { left: Option<f64> },
}

pub enum PdfDestination {
    Page { page: u32, fit: PdfDestinationFit },
    RemotePage { page: u32, fit: PdfDestinationFit },
    Named { name: String },
}

pub enum PdfFileSpecification {
    Path { path: String },
    Embedded { file: String },
}

pub enum PdfActionKind {
    GoTo { destination: PdfDestination },
    GoToRemote { file: PdfFileSpecification, destination: PdfDestination, new_window: Option<bool> },
    GoToEmbedded { destination: PdfDestination, new_window: Option<bool> },
    Launch { file: PdfFileSpecification, new_window: Option<bool> },
    Thread { file: Option<PdfFileSpecification>, thread: u32 },
    Uri { uri: String, is_map: bool },
    Sound { sound: String, volume: Option<f64>, synchronous: bool, repeat: bool, mix: bool },
    Movie { annotation: Option<String>, operation: Option<String> },
    Hide { annotations: Vec<String>, hide: bool },
    Named { name: String },
    SubmitForm { url: String, fields: Vec<String>, flags: u32 },
    ResetForm { fields: Vec<String>, flags: u32 },
    ImportData { file: PdfFileSpecification },
    JavaScript { script: String },
    SetOptionalContentState { states: Vec<PdfDictEntry>, preserve_radio_buttons: bool },
    Rendition { entries: Vec<PdfDictEntry> },
    Transition { entries: Vec<PdfDictEntry> },
    GoTo3dView { entries: Vec<PdfDictEntry> },
    Unknown { subtype: String, entries: Vec<PdfDictEntry> },
}

pub struct PdfAction {
    pub kind: PdfActionKind,
    pub next: Vec<PdfAction>,
}

pub struct PdfOutlineItem {
    pub title: String,
    pub destination: Option<PdfDestination>,
    pub action: Option<PdfAction>,
    pub color: Option<[f64; 3]>,
    pub italic: bool,
    pub bold: bool,
    pub open: bool,
    pub children: Vec<PdfOutlineItem>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfNamedDestination {
    pub name: String,
    pub destination: PdfDestination,
}

pub enum PdfPageLabelStyle {
    Decimal,
    RomanUpper,
    RomanLower,
    LettersUpper,
    LettersLower,
}

pub struct PdfPageLabelRange {
    pub start_index: u32,
    pub style: Option<PdfPageLabelStyle>,
    pub prefix: Option<String>,
    pub start: u32,
}

pub struct PdfAppearanceState {
    pub state: String,
    pub form: String,
}

pub enum PdfAppearanceEntry {
    Single { form: String },
    States { states: Vec<PdfAppearanceState> },
}

pub struct PdfAppearance {
    pub normal: PdfAppearanceEntry,
    pub rollover: Option<PdfAppearanceEntry>,
    pub down: Option<PdfAppearanceEntry>,
}

pub struct PdfBorderStyle {
    pub width: f64,
    pub style: Option<String>,
    pub dash: Option<Vec<f64>>,
    pub radii: Option<[f64; 2]>,
}

pub struct PdfMarkupAnnotation {
    pub title: Option<String>,
    pub popup: Option<usize>,
    pub opacity: Option<f64>,
    pub rich_contents: Option<String>,
    pub creation_date: Option<PdfDate>,
    pub in_reply_to: Option<usize>,
    pub subject: Option<String>,
    pub reply_type: Option<String>,
    pub intent: Option<String>,
}

pub enum PdfAnnotationKind {
    Text { open: bool, icon: Option<String>, state: Option<String>, state_model: Option<String> },
    Link { action: Option<PdfAction>, destination: Option<PdfDestination>, highlight: Option<String>, quad_points: Vec<f64> },
    FreeText { default_appearance: String, quadding: u32, callout: Option<Vec<f64>>, line_ending: Option<String>, rich_text: Option<String> },
    Line { points: [f64; 4], line_endings: Option<[String; 2]>, interior_color: Option<Vec<f64>>, leader_length: Option<f64>, caption: bool },
    Square { interior_color: Option<Vec<f64>>, rect_differences: Option<PdfRect> },
    Circle { interior_color: Option<Vec<f64>>, rect_differences: Option<PdfRect> },
    Polygon { vertices: Vec<f64>, interior_color: Option<Vec<f64>> },
    PolyLine { vertices: Vec<f64>, line_endings: Option<[String; 2]>, interior_color: Option<Vec<f64>> },
    Highlight { quad_points: Vec<f64> },
    Underline { quad_points: Vec<f64> },
    Squiggly { quad_points: Vec<f64> },
    StrikeOut { quad_points: Vec<f64> },
    Stamp { icon: Option<String> },
    Caret { rect_differences: Option<PdfRect>, symbol: Option<String> },
    Ink { paths: Vec<Vec<f64>> },
    Popup { parent: Option<usize>, open: bool },
    FileAttachment { file: PdfFileSpecification, icon: Option<String> },
    Sound { sound: Vec<PdfDictEntry>, icon: Option<String> },
    Movie { title: Option<String>, movie: Vec<PdfDictEntry>, activation: Option<Vec<PdfDictEntry>> },
    Widget { field: Option<String>, highlight: Option<String>, characteristics: Vec<PdfDictEntry>, action: Option<PdfAction>, additional_actions: Vec<PdfDictEntry> },
    Screen { title: Option<String>, characteristics: Vec<PdfDictEntry>, action: Option<PdfAction>, additional_actions: Vec<PdfDictEntry> },
    PrinterMark { mark_style: Option<String>, colorants: Vec<PdfDictEntry> },
    TrapNet { entries: Vec<PdfDictEntry> },
    Watermark { fixed_print: Option<Vec<PdfDictEntry>> },
    ThreeD { entries: Vec<PdfDictEntry> },
    Redact { quad_points: Vec<f64>, interior_color: Option<Vec<f64>>, overlay_text: Option<String>, repeat: bool, default_appearance: Option<String>, quadding: u32 },
    Unknown { subtype: String, entries: Vec<PdfDictEntry> },
}

pub struct PdfAnnotation {
    pub rect: PdfRect,
    pub kind: PdfAnnotationKind,
    pub contents: Option<String>,
    pub name: Option<String>,
    pub modified: Option<String>,
    pub flags: u32,
    pub border: Option<PdfBorderStyle>,
    pub color: Vec<f64>,
    pub appearance: Option<PdfAppearance>,
    pub appearance_state: Option<String>,
    pub markup: Option<PdfMarkupAnnotation>,
    pub optional_content: Option<String>,
    pub struct_parent: Option<u32>,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfFormFieldKind {
    Button { value: Option<String>, default_value: Option<String>, options: Vec<String> },
    Text { value: Option<String>, default_value: Option<String>, max_length: Option<u32>, rich_value: Option<String> },
    Choice { values: Vec<String>, default_values: Vec<String>, options: Vec<(String, String)>, top_index: Option<u32> },
    Signature { value: Option<Vec<PdfDictEntry>> },
    Container,
}

pub struct PdfFormField {
    pub name: String,
    pub kind: PdfFormFieldKind,
    pub flags: u32,
    pub alternate_name: Option<String>,
    pub mapping_name: Option<String>,
    pub default_appearance: Option<String>,
    pub quadding: Option<u32>,
    pub widgets: Vec<[u32; 2]>,
    pub children: Vec<PdfFormField>,
    pub additional_actions: Vec<PdfDictEntry>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfAcroForm {
    pub fields: Vec<PdfFormField>,
    pub need_appearances: bool,
    pub signature_flags: u32,
    pub default_appearance: Option<String>,
    pub quadding: Option<u32>,
    pub default_fonts: Vec<String>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfOptionalContentGroup {
    pub id: String,
    pub name: String,
    pub intent: Vec<String>,
    pub usage: Vec<PdfDictEntry>,
}

pub struct PdfOptionalContent {
    pub groups: Vec<PdfOptionalContentGroup>,
    pub name: Option<String>,
    pub base_state_off: bool,
    pub on: Vec<String>,
    pub off: Vec<String>,
    pub order: Vec<PdfObject>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub offset_minutes: Option<i32>,
}

pub struct PdfInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<PdfDate>,
    pub modification_date: Option<PdfDate>,
    pub trapped: Option<String>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfEmbeddedFile {
    pub id: String,
    pub file_name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub data: Vec<u8>,
    pub creation_date: Option<PdfDate>,
    pub modification_date: Option<PdfDate>,
    pub relationship: Option<String>,
    pub listed: bool,
}

pub struct PdfOutputIntent {
    pub subtype: String,
    pub condition_identifier: String,
    pub condition: Option<String>,
    pub registry_name: Option<String>,
    pub info: Option<String>,
    pub profile: Option<Vec<u8>>,
}

pub enum PdfPageLayout {
    SinglePage,
    OneColumn,
    TwoColumnLeft,
    TwoColumnRight,
    TwoPageLeft,
    TwoPageRight,
}

pub enum PdfPageMode {
    UseNone,
    UseOutlines,
    UseThumbs,
    FullScreen,
    UseOc,
    UseAttachments,
}

pub struct PdfViewerPreferences {
    pub hide_toolbar: bool,
    pub hide_menubar: bool,
    pub hide_window_ui: bool,
    pub fit_window: bool,
    pub center_window: bool,
    pub display_doc_title: bool,
    pub non_full_screen_page_mode: Option<PdfPageMode>,
    pub direction: Option<String>,
    pub view_area: Option<String>,
    pub view_clip: Option<String>,
    pub print_area: Option<String>,
    pub print_clip: Option<String>,
    pub print_scaling: Option<String>,
    pub duplex: Option<String>,
    pub pick_tray_by_pdf_size: bool,
    pub print_page_range: Vec<u32>,
    pub num_copies: Option<u32>,
    pub extra: Vec<PdfDictEntry>,
}

pub enum PdfOpenAction {
    Destination { destination: PdfDestination },
    Action { action: PdfAction },
}

pub enum PdfEncryptionAlgorithm {
    Rc4_40,
    Rc4_128,
    Aes128,
    Aes256,
}

pub struct PdfEncryption {
    pub algorithm: PdfEncryptionAlgorithm,
    pub permissions: i32,
    pub user_password: String,
    pub owner_password: Option<String>,
    pub encrypt_metadata: bool,
}

pub struct PdfMarkInfo {
    pub marked: bool,
    pub user_properties: bool,
    pub suspects: bool,
}

pub struct PdfPage {
    pub media_box: PdfRect,
    pub crop_box: Option<PdfRect>,
    pub bleed_box: Option<PdfRect>,
    pub trim_box: Option<PdfRect>,
    pub art_box: Option<PdfRect>,
    pub rotate: i32,
    pub user_unit: Option<f64>,
    pub content: Vec<PdfOp>,
    pub annotations: Vec<PdfAnnotation>,
    pub group: Option<PdfTransparencyGroup>,
    pub thumbnail: Option<String>,
    pub struct_parents: Option<u32>,
    pub transition: Option<Vec<PdfDictEntry>>,
    pub duration: Option<f64>,
    pub metadata: Option<String>,
    pub additional_actions: Vec<PdfDictEntry>,
    pub extra: Vec<PdfDictEntry>,
}

pub struct PdfSnapshot {
    pub schema: String,
    pub declared_version: String,
    pub pages: Vec<PdfPage>,
    pub fonts: Vec<PdfFont>,
    pub images: Vec<PdfImage>,
    pub forms: Vec<PdfFormXObject>,
    pub ext_g_states: Vec<PdfExtGState>,
    pub shadings: Vec<PdfShading>,
    pub patterns: Vec<PdfPattern>,
    pub color_spaces: Vec<PdfNamedColorSpace>,
    pub properties: Vec<PdfNamedProperties>,
    pub outlines: Vec<PdfOutlineItem>,
    pub named_destinations: Vec<PdfNamedDestination>,
    pub page_labels: Vec<PdfPageLabelRange>,
    pub embedded_files: Vec<PdfEmbeddedFile>,
    pub output_intents: Vec<PdfOutputIntent>,
    pub acro_form: Option<PdfAcroForm>,
    pub optional_content: Option<PdfOptionalContent>,
    pub page_layout: Option<PdfPageLayout>,
    pub page_mode: Option<PdfPageMode>,
    pub viewer_preferences: Option<PdfViewerPreferences>,
    pub open_action: Option<PdfOpenAction>,
    pub language: Option<String>,
    pub mark_info: Option<PdfMarkInfo>,
    pub metadata: Option<String>,
    pub document_id: Option<[Vec<u8>; 2]>,
    pub encryption: Option<PdfEncryption>,
    pub info: PdfInfo,
    pub catalog_extra: Vec<PdfDictEntry>,
    pub objects: Vec<PdfIndirectObject>,
    pub trailer: Vec<PdfDictEntry>,
}
```
