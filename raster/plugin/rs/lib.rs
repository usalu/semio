//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM component.

//#region 🔖Domain
/// 🖼️ Raster's own document/VCS domain model (layer tree, operations, diffs) kept app-owned while
/// `RasterHost`/`RasterSession` (the generic paint-canvas hosting mechanism) live in
/// `framework_surface_paint`.
pub(crate) mod domain {
    use std::collections::HashMap;

    //#region 🔖DocumentVcs
    use vcs::{DocumentDsl, OpText, Operation, OperationDiff};

    pub const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";

    fn default_one() -> f64 {
        1.0
    }

    fn default_true() -> bool {
        true
    }
    
    //#region 🔖Projection
    /// 🎞️ Non-destructive raster document: a nested layer tree (pixel/group/adjustment) over a pannable
    /// camera, plus embedded image assets. This is the authoritative projection shared by the wasm
    /// compositor bridge and the `raster-plugin` `DocumentApp`. Ephemeral tool/brush/selection state
    /// lives in the plugin's app struct, never here.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterCamera {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default = "default_one")]
        pub zoom: f64,
    }
    
    impl Default for RasterCamera {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, zoom: 1.0 }
        }
    }
    
    fn one_f32() -> f32 {
        1.0
    }
    
    fn default_blend() -> String {
        "normal".into()
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterTransform {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default = "default_one")]
        pub scale_x: f64,
        #[serde(default = "default_one")]
        pub scale_y: f64,
        #[serde(default)]
        pub rotation: f64,
    }
    
    impl Default for RasterTransform {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
        }
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterLayerMask {
        #[serde(default = "default_true")]
        pub enabled: bool,
        #[serde(default = "default_true")]
        pub linked: bool,
        #[serde(default)]
        pub invert: bool,
        pub width: Option<u32>,
        pub height: Option<u32>,
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum RasterLayerNode {
        #[serde(rename = "pixel", rename_all = "camelCase")]
        Pixel {
            id: String,
            name: String,
            #[serde(default = "default_true")]
            visible: bool,
            #[serde(default = "one_f32")]
            opacity: f32,
            #[serde(default = "default_blend")]
            blend_mode: String,
            #[serde(default)]
            transform: RasterTransform,
            mask: Option<RasterLayerMask>,
            width: Option<u32>,
            height: Option<u32>,
            image_key: Option<String>,
        },
        #[serde(rename = "group", rename_all = "camelCase")]
        Group {
            id: String,
            name: String,
            #[serde(default = "default_true")]
            visible: bool,
            #[serde(default = "one_f32")]
            opacity: f32,
            #[serde(default = "default_blend")]
            blend_mode: String,
            #[serde(default)]
            transform: RasterTransform,
            mask: Option<RasterLayerMask>,
            children: Vec<RasterLayerNode>,
        },
        #[serde(rename = "adjustment", rename_all = "camelCase")]
        Adjustment {
            id: String,
            name: String,
            #[serde(default = "default_true")]
            visible: bool,
            #[serde(default = "one_f32")]
            opacity: f32,
            #[serde(default = "default_blend")]
            blend_mode: String,
            #[serde(default)]
            transform: RasterTransform,
            adjustment_kind: String,
            #[serde(default)]
            params: serde_json::Map<String, serde_json::Value>,
        },
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterImageAsset {
        pub mime: String,
        pub data: String,
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterProjection {
        pub schema: String,
        pub id: String,
        #[serde(default)]
        pub title: Option<String>,
        #[serde(default)]
        pub camera: RasterCamera,
        #[serde(default)]
        pub layers: Vec<RasterLayerNode>,
        #[serde(default)]
        pub assets: HashMap<String, RasterImageAsset>,
    }
    
    pub fn empty_raster_projection() -> RasterProjection {
        RasterProjection {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "raster".into(),
            title: Some("Untitled".into()),
            camera: RasterCamera::default(),
            layers: Vec::new(),
            assets: HashMap::new(),
        }
    }
    //#endregion 🔖Projection
    
    //#region 🔖Tree
    pub fn layer_node_id(layer: &RasterLayerNode) -> &str {
        match layer {
            RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
        }
    }
    
    pub fn layer_name(layer: &RasterLayerNode) -> &str {
        match layer {
            RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. } => name,
        }
    }
    
    pub fn layer_visible(layer: &RasterLayerNode) -> bool {
        match layer {
            RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. } => *visible,
        }
    }
    
    pub fn layer_opacity(layer: &RasterLayerNode) -> f32 {
        match layer {
            RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. } => *opacity,
        }
    }
    
    pub fn find_layer<'a>(layers: &'a [RasterLayerNode], target_id: &str) -> Option<&'a RasterLayerNode> {
        for layer in layers {
            if layer_node_id(layer) == target_id {
                return Some(layer);
            }
            if let RasterLayerNode::Group { children, .. } = layer {
                if let Some(found) = find_layer(children, target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    
    /// 🧭 Finds a layer's parent-group id (`None` at the root) and its index among its siblings.
    pub fn locate_layer(layers: &[RasterLayerNode], target_id: &str) -> Option<(Option<String>, usize)> {
        fn walk(layers: &[RasterLayerNode], parent: Option<&str>, target_id: &str) -> Option<(Option<String>, usize)> {
            for (index, layer) in layers.iter().enumerate() {
                if layer_node_id(layer) == target_id {
                    return Some((parent.map(str::to_string), index));
                }
                if let RasterLayerNode::Group { id, children, .. } = layer {
                    if let Some(found) = walk(children, Some(id), target_id) {
                        return Some(found);
                    }
                }
            }
            None
        }
        walk(layers, None, target_id)
    }
    
    pub fn flatten_raster_layers(layers: &[RasterLayerNode]) -> Vec<&RasterLayerNode> {
        let mut out = Vec::new();
        fn visit<'a>(layers: &'a [RasterLayerNode], out: &mut Vec<&'a RasterLayerNode>) {
            for layer in layers {
                out.push(layer);
                if let RasterLayerNode::Group { children, .. } = layer {
                    visit(children, out);
                }
            }
        }
        visit(layers, &mut out);
        out
    }
    
    fn remove_layer_from_tree(layers: &mut Vec<RasterLayerNode>, target_id: &str) -> Option<RasterLayerNode> {
        if let Some(index) = layers.iter().position(|layer| layer_node_id(layer) == target_id) {
            return Some(layers.remove(index));
        }
        for layer in layers.iter_mut() {
            if let RasterLayerNode::Group { children, .. } = layer {
                if let Some(removed) = remove_layer_from_tree(children, target_id) {
                    return Some(removed);
                }
            }
        }
        None
    }
    
    fn insert_layer(layers: &mut Vec<RasterLayerNode>, parent_id: Option<&str>, index: usize, layer: RasterLayerNode) {
        match parent_id {
            None => {
                let at = index.min(layers.len());
                layers.insert(at, layer);
            }
            Some(parent_id) => {
                for node in layers.iter_mut() {
                    if let RasterLayerNode::Group { id, children, .. } = node {
                        if id == parent_id {
                            let at = index.min(children.len());
                            children.insert(at, layer);
                            return;
                        }
                        insert_layer(children, Some(parent_id), index, layer.clone());
                    }
                }
            }
        }
    }
    
    fn apply_layer_patch(node: &mut RasterLayerNode, patch: &RasterLayerPatch) -> RasterLayerPatch {
        let mut inverse = RasterLayerPatch::default();
        match node {
            RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, width, height, .. } => {
                if let Some(value) = &patch.name {
                    inverse.name = Some(name.clone());
                    *name = value.clone();
                }
                if let Some(value) = patch.visible {
                    inverse.visible = Some(*visible);
                    *visible = value;
                }
                if let Some(value) = patch.opacity {
                    inverse.opacity = Some(*opacity);
                    *opacity = value;
                }
                if let Some(value) = &patch.blend_mode {
                    inverse.blend_mode = Some(blend_mode.clone());
                    *blend_mode = value.clone();
                }
                if let Some(value) = patch.transform_x {
                    inverse.transform_x = Some(transform.x);
                    transform.x = value;
                }
                if let Some(value) = patch.transform_y {
                    inverse.transform_y = Some(transform.y);
                    transform.y = value;
                }
                if let Some(value) = patch.width {
                    inverse.width = Some(width.unwrap_or(512));
                    *width = Some(value);
                }
                if let Some(value) = patch.height {
                    inverse.height = Some(height.unwrap_or(512));
                    *height = Some(value);
                }
            }
            RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, .. } => {
                if let Some(value) = &patch.name {
                    inverse.name = Some(name.clone());
                    *name = value.clone();
                }
                if let Some(value) = patch.visible {
                    inverse.visible = Some(*visible);
                    *visible = value;
                }
                if let Some(value) = patch.opacity {
                    inverse.opacity = Some(*opacity);
                    *opacity = value;
                }
                if let Some(value) = &patch.blend_mode {
                    inverse.blend_mode = Some(blend_mode.clone());
                    *blend_mode = value.clone();
                }
                if let Some(value) = patch.transform_x {
                    inverse.transform_x = Some(transform.x);
                    transform.x = value;
                }
                if let Some(value) = patch.transform_y {
                    inverse.transform_y = Some(transform.y);
                    transform.y = value;
                }
            }
            RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, adjustment_kind, .. } => {
                if let Some(value) = &patch.name {
                    inverse.name = Some(name.clone());
                    *name = value.clone();
                }
                if let Some(value) = patch.visible {
                    inverse.visible = Some(*visible);
                    *visible = value;
                }
                if let Some(value) = patch.opacity {
                    inverse.opacity = Some(*opacity);
                    *opacity = value;
                }
                if let Some(value) = &patch.blend_mode {
                    inverse.blend_mode = Some(blend_mode.clone());
                    *blend_mode = value.clone();
                }
                if let Some(value) = &patch.adjustment_kind {
                    inverse.adjustment_kind = Some(adjustment_kind.clone());
                    *adjustment_kind = value.clone();
                }
            }
        }
        inverse
    }
    
    fn patch_layer_in_tree(layers: &mut [RasterLayerNode], target_id: &str, patch: &RasterLayerPatch) -> Option<RasterLayerPatch> {
        for layer in layers.iter_mut() {
            if layer_node_id(layer) == target_id {
                return Some(apply_layer_patch(layer, patch));
            }
            if let RasterLayerNode::Group { children, .. } = layer {
                if let Some(inverse) = patch_layer_in_tree(children, target_id, patch) {
                    return Some(inverse);
                }
            }
        }
        None
    }
    //#endregion 🔖Tree
    
    //#region 🔖Operations
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterLayerPatch {
        pub name: Option<String>,
        pub visible: Option<bool>,
        pub opacity: Option<f32>,
        pub blend_mode: Option<String>,
        pub transform_x: Option<f64>,
        pub transform_y: Option<f64>,
        pub width: Option<u32>,
        pub height: Option<u32>,
        pub adjustment_kind: Option<String>,
    }
    
    /// 🧩 One atomic tree mutation — the building block of {@link RasterDiff}, kept ordered so a diff can
    /// coalesce several edits (e.g. a multi-layer patch) while still inverting each mechanically.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "step", rename_all = "camelCase")]
    pub enum RasterStep {
        AddLayer { parent_id: Option<String>, index: usize, layer: RasterLayerNode },
        RemoveLayer { layer_id: String },
        PatchLayer { layer_id: String, patch: RasterLayerPatch },
        MoveLayer { layer_id: String, parent_id: Option<String>, index: usize },
    }
    
    fn apply_step(layers: &mut Vec<RasterLayerNode>, step: &RasterStep) {
        match step {
            RasterStep::AddLayer { parent_id, index, layer } => insert_layer(layers, parent_id.as_deref(), *index, layer.clone()),
            RasterStep::RemoveLayer { layer_id } => {
                remove_layer_from_tree(layers, layer_id);
            }
            RasterStep::PatchLayer { layer_id, patch } => {
                patch_layer_in_tree(layers, layer_id, patch);
            }
            RasterStep::MoveLayer { layer_id, parent_id, index } => {
                if let Some(node) = remove_layer_from_tree(layers, layer_id) {
                    insert_layer(layers, parent_id.as_deref(), *index, node);
                }
            }
        }
    }
    
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "operation", rename_all = "camelCase")]
    pub enum RasterOperation {
        AddLayer { parent_id: Option<String>, index: usize, layer: RasterLayerNode },
        RemoveLayer { layer_id: String },
        PatchLayer { layer_id: String, patch: RasterLayerPatch },
        MoveLayer { layer_id: String, parent_id: Option<String>, index: usize },
        SetCamera { camera: RasterCamera },
        ReplaceDocument { document: RasterProjection },
    }
    
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RasterDiff {
        pub steps: Vec<RasterStep>,
        pub camera: Option<RasterCamera>,
        pub replace: Option<Box<RasterProjection>>,
    }
    
    impl OperationDiff<RasterProjection> for RasterDiff {
        fn apply(&self, projection: &RasterProjection) -> RasterProjection {
            let mut next = self.replace.as_ref().map(|document| (**document).clone()).unwrap_or_else(|| projection.clone());
            for step in &self.steps {
                apply_step(&mut next.layers, step);
            }
            if let Some(camera) = &self.camera {
                next.camera = camera.clone();
            }
            next
        }
    
        fn absorb(&mut self, other: Self) {
            if let Some(replace) = other.replace {
                self.replace = Some(replace);
                self.steps.clear();
            }
            self.steps.extend(other.steps);
            if other.camera.is_some() {
                self.camera = other.camera;
            }
        }
    }
    
    fn step_diff(step: RasterStep) -> RasterDiff {
        RasterDiff { steps: vec![step], ..Default::default() }
    }
    
    impl Operation<RasterProjection> for RasterOperation {
        type Diff = RasterDiff;
    
        fn diff(&self, _projection: &RasterProjection) -> RasterDiff {
            match self {
                RasterOperation::AddLayer { parent_id, index, layer } => {
                    step_diff(RasterStep::AddLayer { parent_id: parent_id.clone(), index: *index, layer: layer.clone() })
                }
                RasterOperation::RemoveLayer { layer_id } => step_diff(RasterStep::RemoveLayer { layer_id: layer_id.clone() }),
                RasterOperation::PatchLayer { layer_id, patch } => {
                    step_diff(RasterStep::PatchLayer { layer_id: layer_id.clone(), patch: patch.clone() })
                }
                RasterOperation::MoveLayer { layer_id, parent_id, index } => {
                    step_diff(RasterStep::MoveLayer { layer_id: layer_id.clone(), parent_id: parent_id.clone(), index: *index })
                }
                RasterOperation::SetCamera { camera } => RasterDiff { camera: Some(camera.clone()), ..Default::default() },
                RasterOperation::ReplaceDocument { document } => RasterDiff { replace: Some(Box::new(document.clone())), ..Default::default() },
            }
        }
    
        fn backwards(&self, projection: &RasterProjection) -> Vec<Self> {
            match self {
                RasterOperation::AddLayer { layer, .. } => vec![RasterOperation::RemoveLayer { layer_id: layer_node_id(layer).to_string() }],
                RasterOperation::RemoveLayer { layer_id } => match (locate_layer(&projection.layers, layer_id), find_layer(&projection.layers, layer_id)) {
                    (Some((parent_id, index)), Some(layer)) => vec![RasterOperation::AddLayer { parent_id, index, layer: layer.clone() }],
                    _ => Vec::new(),
                },
                RasterOperation::PatchLayer { layer_id, patch } => {
                    let mut probe = projection.layers.clone();
                    match patch_layer_in_tree(&mut probe, layer_id, patch) {
                        Some(inverse) => vec![RasterOperation::PatchLayer { layer_id: layer_id.clone(), patch: inverse }],
                        None => Vec::new(),
                    }
                }
                RasterOperation::MoveLayer { layer_id, .. } => match locate_layer(&projection.layers, layer_id) {
                    Some((parent_id, index)) => vec![RasterOperation::MoveLayer { layer_id: layer_id.clone(), parent_id, index }],
                    None => Vec::new(),
                },
                RasterOperation::SetCamera { .. } => vec![RasterOperation::SetCamera { camera: projection.camera.clone() }],
                RasterOperation::ReplaceDocument { .. } => vec![RasterOperation::ReplaceDocument { document: projection.clone() }],
            }
        }
    }
    
    pub type RasterEnvelope = vcs::DocumentVcsEnvelope<RasterProjection, RasterOperation>;
    pub type RasterStore = vcs::DocumentVcsStore<RasterProjection, RasterOperation>;
    //#endregion 🔖Operations

    //#region 🔖Dsl
    /// 📜 Hand-rolled lexer, parser and printer shared by `RasterProjection`'s `.raster` DSL and by
    /// `RasterOperation`'s compact single-line op encoding (`AddLayer`/`ReplaceDocument` reprint the same
    /// layer/document grammar on one line). Whitespace (including newlines) is never significant to the
    /// parser — `print_dsl` inserts newlines/indentation purely for readability, `print_op` renders the
    /// identical grammar with spaces only. See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
    mod raster_text {
        use super::{RasterCamera, RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterLayerPatch, RasterProjection, RasterTransform};
        use std::collections::HashMap;

        //#region Lexer
        #[derive(Clone, Debug, PartialEq)]
        enum Tok {
            Word(String),
            Str(String),
            LBrace,
            RBrace,
            LBracket,
            RBracket,
            Eof,
        }

        #[derive(Clone, Debug)]
        struct Lexed {
            tok: Tok,
            span: vcs::TextSpan,
        }

        /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`[`/`]`/`"`, so
        /// `=` and `,` are ordinary word characters — `key=value` collapses into one token (split later
        /// by {@link Parser::parse_kv_map}), and only a quoted value or `[`/`{` forces a token boundary
        /// right after `key=`.
        fn lex(input: &str) -> Result<Vec<Lexed>, vcs::TextError> {
            let chars: Vec<char> = input.chars().collect();
            let mut out = Vec::new();
            let mut i = 0usize;
            let mut line = 1u32;
            let mut col = 1u32;
            while i < chars.len() {
                match chars[i] {
                    ' ' | '\t' | '\r' => {
                        i += 1;
                        col += 1;
                    }
                    '\n' => {
                        i += 1;
                        line += 1;
                        col = 1;
                    }
                    '{' => {
                        out.push(Lexed { tok: Tok::LBrace, span: vcs::TextSpan::at(line, col) });
                        i += 1;
                        col += 1;
                    }
                    '}' => {
                        out.push(Lexed { tok: Tok::RBrace, span: vcs::TextSpan::at(line, col) });
                        i += 1;
                        col += 1;
                    }
                    '[' => {
                        out.push(Lexed { tok: Tok::LBracket, span: vcs::TextSpan::at(line, col) });
                        i += 1;
                        col += 1;
                    }
                    ']' => {
                        out.push(Lexed { tok: Tok::RBracket, span: vcs::TextSpan::at(line, col) });
                        i += 1;
                        col += 1;
                    }
                    '"' => {
                        let (start_line, start_col) = (line, col);
                        i += 1;
                        col += 1;
                        let mut s = String::new();
                        let mut closed = false;
                        while i < chars.len() {
                            let ch = chars[i];
                            if ch == '\\' && i + 1 < chars.len() {
                                match chars[i + 1] {
                                    'n' => s.push('\n'),
                                    '"' => s.push('"'),
                                    '\\' => s.push('\\'),
                                    other => {
                                        s.push('\\');
                                        s.push(other);
                                    }
                                }
                                i += 2;
                                col += 2;
                            } else if ch == '"' {
                                i += 1;
                                col += 1;
                                closed = true;
                                break;
                            } else if ch == '\n' {
                                s.push(ch);
                                i += 1;
                                line += 1;
                                col = 1;
                            } else {
                                s.push(ch);
                                i += 1;
                                col += 1;
                            }
                        }
                        if !closed {
                            return Err(vcs::TextError::new("unterminated string literal", vcs::TextSpan::at(start_line, start_col)));
                        }
                        out.push(Lexed { tok: Tok::Str(s), span: vcs::TextSpan::at(start_line, start_col) });
                    }
                    _ => {
                        let (start_line, start_col, start) = (line, col, i);
                        while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '[' | ']' | '"') {
                            i += 1;
                            col += 1;
                        }
                        let word: String = chars[start..i].iter().collect();
                        out.push(Lexed { tok: Tok::Word(word), span: vcs::TextSpan::at(start_line, start_col) });
                    }
                }
            }
            out.push(Lexed { tok: Tok::Eof, span: vcs::TextSpan::at(line, col) });
            Ok(out)
        }
        //#endregion Lexer

        //#region Parser
        #[derive(Clone, Debug)]
        enum FieldValue {
            Str(String),
            Word(String),
        }

        struct Parser {
            toks: Vec<Lexed>,
            pos: usize,
        }

        impl Parser {
            fn peek(&self) -> &Tok {
                &self.toks[self.pos].tok
            }

            fn span(&self) -> vcs::TextSpan {
                self.toks[self.pos].span
            }

            fn bump(&mut self) -> Tok {
                let tok = self.toks[self.pos].tok.clone();
                if self.pos + 1 < self.toks.len() {
                    self.pos += 1;
                }
                tok
            }

            fn at_lbrace(&self) -> bool {
                matches!(self.peek(), Tok::LBrace)
            }

            fn at_rbrace(&self) -> bool {
                matches!(self.peek(), Tok::RBrace)
            }

            fn at_rbracket(&self) -> bool {
                matches!(self.peek(), Tok::RBracket)
            }

            fn at_word(&self, word: &str) -> bool {
                matches!(self.peek(), Tok::Word(w) if w == word)
            }

            fn expect_word(&mut self) -> Result<String, vcs::TextError> {
                let span = self.span();
                match self.bump() {
                    Tok::Word(w) => Ok(w),
                    other => Err(vcs::TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
                }
            }

            fn expect_keyword(&mut self, keyword: &str) -> Result<(), vcs::TextError> {
                let span = self.span();
                let word = self.expect_word()?;
                if word != keyword {
                    return Err(vcs::TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
                }
                Ok(())
            }

            fn expect_lbrace(&mut self) -> Result<(), vcs::TextError> {
                let span = self.span();
                match self.bump() {
                    Tok::LBrace => Ok(()),
                    other => Err(vcs::TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
                }
            }

            fn expect_rbrace(&mut self) -> Result<(), vcs::TextError> {
                let span = self.span();
                match self.bump() {
                    Tok::RBrace => Ok(()),
                    other => Err(vcs::TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
                }
            }

            fn expect_rbracket(&mut self) -> Result<(), vcs::TextError> {
                let span = self.span();
                match self.bump() {
                    Tok::RBracket => Ok(()),
                    other => Err(vcs::TextError::expected(format!("expected ']', found {other:?}"), span, "]")),
                }
            }

            fn expect_str(&mut self) -> Result<String, vcs::TextError> {
                let span = self.span();
                match self.bump() {
                    Tok::Str(s) => Ok(s),
                    other => Err(vcs::TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
                }
            }

            /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one —
            /// the generic header-field reader every construct (document/camera/layer/mask/asset) is
            /// built on.
            fn parse_kv_map(&mut self) -> Result<HashMap<String, (FieldValue, vcs::TextSpan)>, vcs::TextError> {
                let mut map = HashMap::new();
                loop {
                    let word = match self.peek() {
                        Tok::Word(w) if w.contains('=') => w.clone(),
                        _ => break,
                    };
                    let span = self.span();
                    self.bump();
                    let (key, rest) = word.split_once('=').expect("word already checked to contain '='");
                    let value = if rest.is_empty() {
                        FieldValue::Str(self.expect_str()?)
                    } else {
                        FieldValue::Word(rest.to_string())
                    };
                    map.insert(key.to_string(), (value, span));
                }
                Ok(map)
            }
        }

        type FieldMap = HashMap<String, (FieldValue, vcs::TextSpan)>;

        fn kv_str(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
            match map.get(key) {
                Some((FieldValue::Str(s), _)) => Ok(s.clone()),
                Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
                None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
            }
        }

        fn kv_opt_str(map: &FieldMap, key: &str) -> Option<String> {
            match map.get(key) {
                Some((FieldValue::Str(s), _)) => Some(s.clone()),
                _ => None,
            }
        }

        fn kv_word(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
            match map.get(key) {
                Some((FieldValue::Word(w), _)) => Ok(w.clone()),
                Some((FieldValue::Str(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
                None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
            }
        }

        fn kv_num(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<f64, vcs::TextError> {
            let word = kv_word(map, key, span)?;
            word.parse::<f64>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a number"), span, "number"))
        }

        fn kv_num32(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<f32, vcs::TextError> {
            let word = kv_word(map, key, span)?;
            word.parse::<f32>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a number"), span, "number"))
        }

        fn kv_opt_num32(map: &FieldMap, key: &str) -> Option<f32> {
            match map.get(key) {
                Some((FieldValue::Word(w), _)) => w.parse::<f32>().ok(),
                _ => None,
            }
        }

        fn kv_bool(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<bool, vcs::TextError> {
            match kv_word(map, key, span)?.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(vcs::TextError::expected(format!("field '{key}' must be 'true' or 'false'"), span, "true|false")),
            }
        }

        fn kv_opt_bool(map: &FieldMap, key: &str) -> Option<bool> {
            match map.get(key) {
                Some((FieldValue::Word(w), _)) if w == "true" => Some(true),
                Some((FieldValue::Word(w), _)) if w == "false" => Some(false),
                _ => None,
            }
        }

        fn kv_opt_num(map: &FieldMap, key: &str) -> Option<f64> {
            match map.get(key) {
                Some((FieldValue::Word(w), _)) => w.parse::<f64>().ok(),
                _ => None,
            }
        }

        fn kv_opt_u32(map: &FieldMap, key: &str) -> Option<u32> {
            match map.get(key) {
                Some((FieldValue::Word(w), _)) => w.parse::<u32>().ok(),
                _ => None,
            }
        }

        fn kv_usize(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<usize, vcs::TextError> {
            let word = kv_word(map, key, span)?;
            word.parse::<usize>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be an unsigned integer"), span, "uint"))
        }

        //#region Value
        /// 🧬 Recursive value grammar backing `RasterLayerNode::Adjustment`'s free-form `params` map —
        /// number/bool/null/string plus `[ ]` arrays and `{ key=value }` objects, so any `serde_json::Value`
        /// an adjustment kind ever needs round-trips without a JSON-in-a-string escape hatch.
        fn parse_scalar_word(word: &str, span: vcs::TextSpan) -> Result<serde_json::Value, vcs::TextError> {
            match word {
                "true" => Ok(serde_json::Value::Bool(true)),
                "false" => Ok(serde_json::Value::Bool(false)),
                "null" => Ok(serde_json::Value::Null),
                _ => word
                    .parse::<f64>()
                    .map(serde_json::Value::from)
                    .map_err(|_| vcs::TextError::expected(format!("invalid value '{word}'"), span, "number|true|false|null")),
            }
        }

        fn parse_value(p: &mut Parser) -> Result<serde_json::Value, vcs::TextError> {
            let span = p.span();
            match p.peek().clone() {
                Tok::Str(s) => {
                    p.bump();
                    Ok(serde_json::Value::String(s))
                }
                Tok::LBracket => {
                    p.bump();
                    let mut items = Vec::new();
                    while !p.at_rbracket() {
                        items.push(parse_value(p)?);
                    }
                    p.expect_rbracket()?;
                    Ok(serde_json::Value::Array(items))
                }
                Tok::LBrace => {
                    p.bump();
                    let mut map = serde_json::Map::new();
                    while !p.at_rbrace() {
                        let entry_span = p.span();
                        let word = p.expect_word()?;
                        let (key, value) = parse_kv_value(&word, entry_span, p)?;
                        map.insert(key, value);
                    }
                    p.expect_rbrace()?;
                    Ok(serde_json::Value::Object(map))
                }
                Tok::Word(w) => {
                    p.bump();
                    parse_scalar_word(&w, span)
                }
                other => Err(vcs::TextError::expected(format!("expected a value, found {other:?}"), span, "value")),
            }
        }

        /// 🔑 Splits a `key=value`/`key=` token and reads its value — the value is either the inline
        /// scalar bareword already in `rest`, or (when `rest` is empty) whatever follows: a quoted
        /// string, a `[ ]` array, or a `{ }` object, via {@link parse_value}.
        fn parse_kv_value(word: &str, entry_span: vcs::TextSpan, p: &mut Parser) -> Result<(String, serde_json::Value), vcs::TextError> {
            let Some((key, rest)) = word.split_once('=') else {
                return Err(vcs::TextError::expected("expected 'key=value' in params", entry_span, "key=value"));
            };
            let value = if rest.is_empty() { parse_value(p)? } else { parse_scalar_word(rest, entry_span)? };
            Ok((key.to_string(), value))
        }

        fn parse_params(p: &mut Parser) -> Result<serde_json::Map<String, serde_json::Value>, vcs::TextError> {
            p.expect_lbrace()?;
            let mut map = serde_json::Map::new();
            while !p.at_rbrace() {
                let entry_span = p.span();
                let word = p.expect_word()?;
                let (key, value) = parse_kv_value(&word, entry_span, p)?;
                map.insert(key, value);
            }
            p.expect_rbrace()?;
            Ok(map)
        }

        fn print_value(value: &serde_json::Value) -> String {
            match value {
                serde_json::Value::Null => "null".to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::String(s) => quote(s),
                serde_json::Value::Array(items) => format!("[{}]", items.iter().map(print_value).collect::<Vec<_>>().join(" ")),
                serde_json::Value::Object(map) => {
                    format!("{{ {} }}", map.iter().map(|(k, v)| format!("{k}={}", print_value(v))).collect::<Vec<_>>().join(" "))
                }
            }
        }
        //#endregion Value

        fn parse_mask(p: &mut Parser) -> Result<RasterLayerMask, vcs::TextError> {
            p.expect_keyword("mask")?;
            p.expect_lbrace()?;
            let span = p.span();
            let map = p.parse_kv_map()?;
            p.expect_rbrace()?;
            Ok(RasterLayerMask {
                enabled: kv_bool(&map, "enabled", span)?,
                linked: kv_bool(&map, "linked", span)?,
                invert: kv_bool(&map, "invert", span)?,
                width: kv_opt_u32(&map, "width"),
                height: kv_opt_u32(&map, "height"),
            })
        }

        /// 📥 Parses one layer node (`pixel`/`group`/`adjustment`), including its optional trailing
        /// `mask { ... }` and (for `group`) `{ children }` / (for `adjustment`) `params { ... }`.
        fn parse_layer(p: &mut Parser) -> Result<RasterLayerNode, vcs::TextError> {
            let span = p.span();
            let kind = p.expect_word()?;
            let map = p.parse_kv_map()?;
            let id = kv_str(&map, "id", span)?;
            let name = kv_str(&map, "name", span)?;
            let visible = kv_bool(&map, "visible", span)?;
            let opacity = kv_num32(&map, "opacity", span)?;
            let blend_mode = kv_str(&map, "blend", span)?;
            let transform = RasterTransform {
                x: kv_num(&map, "x", span)?,
                y: kv_num(&map, "y", span)?,
                scale_x: kv_num(&map, "scaleX", span)?,
                scale_y: kv_num(&map, "scaleY", span)?,
                rotation: kv_num(&map, "rotation", span)?,
            };
            match kind.as_str() {
                "pixel" => {
                    let width = kv_opt_u32(&map, "width");
                    let height = kv_opt_u32(&map, "height");
                    let image_key = kv_opt_str(&map, "image");
                    let mask = if p.at_word("mask") { Some(parse_mask(p)?) } else { None };
                    Ok(RasterLayerNode::Pixel { id, name, visible, opacity, blend_mode, transform, mask, width, height, image_key })
                }
                "group" => {
                    let mask = if p.at_word("mask") { Some(parse_mask(p)?) } else { None };
                    let children = if p.at_lbrace() {
                        p.bump();
                        let mut children = Vec::new();
                        while !p.at_rbrace() {
                            children.push(parse_layer(p)?);
                        }
                        p.expect_rbrace()?;
                        children
                    } else {
                        Vec::new()
                    };
                    Ok(RasterLayerNode::Group { id, name, visible, opacity, blend_mode, transform, mask, children })
                }
                "adjustment" => {
                    let adjustment_kind = kv_str(&map, "kind", span)?;
                    let params = if p.at_word("params") {
                        p.bump();
                        parse_params(p)?
                    } else {
                        serde_json::Map::new()
                    };
                    Ok(RasterLayerNode::Adjustment { id, name, visible, opacity, blend_mode, transform, adjustment_kind, params })
                }
                other => Err(vcs::TextError::expected(format!("unknown layer kind '{other}'"), span, "pixel|group|adjustment")),
            }
        }

        /// 📥 Parses a full `.raster` document: `raster`/`camera` (required, any order-independent
        /// fields), then `assets`/`layers` (each optional, in any order).
        pub(super) fn parse_document(text: &str) -> Result<RasterProjection, vcs::TextError> {
            let toks = lex(text)?;
            let mut p = Parser { toks, pos: 0 };

            let doc_span = p.span();
            p.expect_keyword("raster")?;
            let doc_map = p.parse_kv_map()?;
            let id = kv_str(&doc_map, "id", doc_span)?;
            let schema = kv_str(&doc_map, "schema", doc_span)?;
            let title = kv_opt_str(&doc_map, "title");

            let camera_span = p.span();
            p.expect_keyword("camera")?;
            let camera_map = p.parse_kv_map()?;
            let camera = RasterCamera {
                x: kv_num(&camera_map, "x", camera_span)?,
                y: kv_num(&camera_map, "y", camera_span)?,
                zoom: kv_num(&camera_map, "zoom", camera_span)?,
            };

            let mut assets = HashMap::new();
            let mut layers = Vec::new();
            loop {
                let keyword = match p.peek() {
                    Tok::Word(w) => w.clone(),
                    _ => break,
                };
                match keyword.as_str() {
                    "assets" => {
                        p.bump();
                        p.expect_lbrace()?;
                        while !p.at_rbrace() {
                            let key = p.expect_word()?;
                            p.expect_lbrace()?;
                            let entry_span = p.span();
                            let entry_map = p.parse_kv_map()?;
                            p.expect_rbrace()?;
                            assets.insert(
                                key,
                                RasterImageAsset {
                                    mime: kv_str(&entry_map, "mime", entry_span)?,
                                    data: kv_str(&entry_map, "data", entry_span)?,
                                },
                            );
                        }
                        p.expect_rbrace()?;
                    }
                    "layers" => {
                        p.bump();
                        p.expect_lbrace()?;
                        while !p.at_rbrace() {
                            layers.push(parse_layer(&mut p)?);
                        }
                        p.expect_rbrace()?;
                    }
                    other => return Err(vcs::TextError::expected(format!("unknown document section '{other}'"), p.span(), "assets|layers")),
                }
            }

            Ok(RasterProjection { schema, id, title, camera, layers, assets })
        }

        /// ⚡ Parses one op-log line. Every variant but `replace-document ...` (which embeds a whole
        /// compact document — handled as a direct string slice before tokenizing, since it is itself a
        /// nested instance of this same grammar) shares the `Parser` used by {@link parse_document}.
        pub(super) fn parse_operation(line: &str) -> Result<super::RasterOperation, vcs::TextError> {
            use super::RasterOperation;
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("replace-document ") {
                return Ok(RasterOperation::ReplaceDocument { document: parse_document(rest)? });
            }

            let toks = lex(line)?;
            let mut p = Parser { toks, pos: 0 };
            let span = p.span();
            let keyword = p.expect_word()?;
            match keyword.as_str() {
                "add-layer" => {
                    let map = p.parse_kv_map()?;
                    let parent_id = kv_opt_str(&map, "parent");
                    let index = kv_usize(&map, "index", span)?;
                    let layer = parse_layer(&mut p)?;
                    Ok(RasterOperation::AddLayer { parent_id, index, layer })
                }
                "remove-layer" => {
                    let map = p.parse_kv_map()?;
                    Ok(RasterOperation::RemoveLayer { layer_id: kv_str(&map, "id", span)? })
                }
                "patch-layer" => {
                    let map = p.parse_kv_map()?;
                    let layer_id = kv_str(&map, "id", span)?;
                    let patch = RasterLayerPatch {
                        name: kv_opt_str(&map, "name"),
                        visible: kv_opt_bool(&map, "visible"),
                        opacity: kv_opt_num32(&map, "opacity"),
                        blend_mode: kv_opt_str(&map, "blend"),
                        transform_x: kv_opt_num(&map, "x"),
                        transform_y: kv_opt_num(&map, "y"),
                        width: kv_opt_u32(&map, "width"),
                        height: kv_opt_u32(&map, "height"),
                        adjustment_kind: kv_opt_str(&map, "kind"),
                    };
                    Ok(RasterOperation::PatchLayer { layer_id, patch })
                }
                "move-layer" => {
                    let map = p.parse_kv_map()?;
                    Ok(RasterOperation::MoveLayer {
                        layer_id: kv_str(&map, "id", span)?,
                        parent_id: kv_opt_str(&map, "parent"),
                        index: kv_usize(&map, "index", span)?,
                    })
                }
                "set-camera" => {
                    let map = p.parse_kv_map()?;
                    Ok(RasterOperation::SetCamera {
                        camera: RasterCamera {
                            x: kv_num(&map, "x", span)?,
                            y: kv_num(&map, "y", span)?,
                            zoom: kv_num(&map, "zoom", span)?,
                        },
                    })
                }
                other => Err(vcs::TextError::expected(format!("unknown operation '{other}'"), span, "operation keyword")),
            }
        }
        //#endregion Parser

        //#region Printer
        fn quote(value: &str) -> String {
            let mut out = String::with_capacity(value.len() + 2);
            out.push('"');
            for ch in value.chars() {
                match ch {
                    '\\' => out.push_str("\\\\"),
                    '"' => out.push_str("\\\""),
                    '\n' => out.push_str("\\n"),
                    _ => out.push(ch),
                }
            }
            out.push('"');
            out
        }

        fn fmt_num(value: f64) -> String {
            value.to_string()
        }

        fn fmt_num32(value: f32) -> String {
            value.to_string()
        }

        fn indent_str(depth: usize) -> String {
            "  ".repeat(depth)
        }

        /// 🧱 Wraps `items` (each already rendered, without its own leading indentation) in `{ }`, one
        /// per line indented at `depth + 1` when `pretty`, or space-joined on one line otherwise.
        fn wrap_body(items: &[String], depth: usize, pretty: bool) -> String {
            if pretty {
                let inner_pad = indent_str(depth + 1);
                let outer_pad = indent_str(depth);
                let body: String = items.iter().map(|item| format!("{inner_pad}{item}\n")).collect();
                format!("{{\n{body}{outer_pad}}}")
            } else {
                format!("{{ {} }}", items.join(" "))
            }
        }

        fn common_header(id: &str, name: &str, visible: bool, opacity: f32, blend_mode: &str, transform: &RasterTransform) -> String {
            format!(
                "id={} name={} visible={visible} opacity={} blend={} x={} y={} scaleX={} scaleY={} rotation={}",
                quote(id),
                quote(name),
                fmt_num32(opacity),
                quote(blend_mode),
                fmt_num(transform.x),
                fmt_num(transform.y),
                fmt_num(transform.scale_x),
                fmt_num(transform.scale_y),
                fmt_num(transform.rotation),
            )
        }

        fn print_mask(mask: &RasterLayerMask) -> String {
            let mut body = format!("enabled={} linked={} invert={}", mask.enabled, mask.linked, mask.invert);
            if let Some(width) = mask.width {
                body.push_str(&format!(" width={width}"));
            }
            if let Some(height) = mask.height {
                body.push_str(&format!(" height={height}"));
            }
            format!("mask {{ {body} }}")
        }

        fn print_params(params: &serde_json::Map<String, serde_json::Value>, depth: usize, pretty: bool) -> String {
            let items: Vec<String> = params.iter().map(|(key, value)| format!("{key}={}", print_value(value))).collect();
            format!("params {}", wrap_body(&items, depth, pretty))
        }

        fn print_layer(layer: &RasterLayerNode, depth: usize, pretty: bool) -> String {
            match layer {
                RasterLayerNode::Pixel { id, name, visible, opacity, blend_mode, transform, mask, width, height, image_key } => {
                    let mut header = format!("pixel {}", common_header(id, name, *visible, *opacity, blend_mode, transform));
                    if let Some(width) = width {
                        header.push_str(&format!(" width={width}"));
                    }
                    if let Some(height) = height {
                        header.push_str(&format!(" height={height}"));
                    }
                    if let Some(image_key) = image_key {
                        header.push_str(&format!(" image={}", quote(image_key)));
                    }
                    match mask {
                        Some(mask) => format!("{header} {}", print_mask(mask)),
                        None => header,
                    }
                }
                RasterLayerNode::Group { id, name, visible, opacity, blend_mode, transform, mask, children } => {
                    let mut header = format!("group {}", common_header(id, name, *visible, *opacity, blend_mode, transform));
                    if let Some(mask) = mask {
                        header = format!("{header} {}", print_mask(mask));
                    }
                    if children.is_empty() {
                        header
                    } else {
                        let items: Vec<String> = children.iter().map(|child| print_layer(child, depth + 1, pretty)).collect();
                        format!("{header} {}", wrap_body(&items, depth, pretty))
                    }
                }
                RasterLayerNode::Adjustment { id, name, visible, opacity, blend_mode, transform, adjustment_kind, params } => {
                    let header = format!("adjustment {} kind={}", common_header(id, name, *visible, *opacity, blend_mode, transform), quote(adjustment_kind));
                    if params.is_empty() {
                        header
                    } else {
                        format!("{header} {}", print_params(params, depth, pretty))
                    }
                }
            }
        }

        fn print_assets_section(assets: &HashMap<String, RasterImageAsset>, depth: usize, pretty: bool) -> String {
            let mut keys: Vec<&String> = assets.keys().collect();
            keys.sort();
            let items: Vec<String> = keys
                .into_iter()
                .map(|key| {
                    let asset = &assets[key];
                    format!("{key} {{ mime={} data={} }}", quote(&asset.mime), quote(&asset.data))
                })
                .collect();
            format!("assets {}", wrap_body(&items, depth, pretty))
        }

        fn print_layers_section(layers: &[RasterLayerNode], depth: usize, pretty: bool) -> String {
            let items: Vec<String> = layers.iter().map(|layer| print_layer(layer, depth + 1, pretty)).collect();
            format!("layers {}", wrap_body(&items, depth, pretty))
        }

        /// 📤 Renders `document` as `raster`/`camera` (always present) followed by `assets`/`layers`
        /// when non-empty, joined by newlines when `pretty` or single spaces otherwise (see
        /// {@link parse_document} for the mirrored grammar).
        pub(super) fn print_document(document: &RasterProjection, pretty: bool) -> String {
            let mut parts = Vec::new();

            let mut header = format!("raster id={} schema={}", quote(&document.id), quote(&document.schema));
            if let Some(title) = &document.title {
                header.push_str(&format!(" title={}", quote(title)));
            }
            parts.push(header);

            parts.push(format!("camera x={} y={} zoom={}", fmt_num(document.camera.x), fmt_num(document.camera.y), fmt_num(document.camera.zoom)));

            if !document.assets.is_empty() {
                parts.push(print_assets_section(&document.assets, 0, pretty));
            }
            if !document.layers.is_empty() {
                parts.push(print_layers_section(&document.layers, 0, pretty));
            }

            parts.join(if pretty { "\n" } else { " " })
        }

        /// ⚡ Renders one `RasterOperation` as a single line — `AddLayer`/`ReplaceDocument` reuse the
        /// compact (space-joined) form of {@link print_layer}/{@link print_document}.
        pub(super) fn print_operation(operation: &super::RasterOperation) -> String {
            use super::RasterOperation;
            match operation {
                RasterOperation::AddLayer { parent_id, index, layer } => {
                    let mut header = format!("add-layer index={index}");
                    if let Some(parent) = parent_id {
                        header.push_str(&format!(" parent={}", quote(parent)));
                    }
                    format!("{header} {}", print_layer(layer, 0, false))
                }
                RasterOperation::RemoveLayer { layer_id } => format!("remove-layer id={}", quote(layer_id)),
                RasterOperation::PatchLayer { layer_id, patch } => {
                    let mut line = format!("patch-layer id={}", quote(layer_id));
                    if let Some(name) = &patch.name {
                        line.push_str(&format!(" name={}", quote(name)));
                    }
                    if let Some(visible) = patch.visible {
                        line.push_str(&format!(" visible={visible}"));
                    }
                    if let Some(opacity) = patch.opacity {
                        line.push_str(&format!(" opacity={}", fmt_num32(opacity)));
                    }
                    if let Some(blend_mode) = &patch.blend_mode {
                        line.push_str(&format!(" blend={}", quote(blend_mode)));
                    }
                    if let Some(x) = patch.transform_x {
                        line.push_str(&format!(" x={}", fmt_num(x)));
                    }
                    if let Some(y) = patch.transform_y {
                        line.push_str(&format!(" y={}", fmt_num(y)));
                    }
                    if let Some(width) = patch.width {
                        line.push_str(&format!(" width={width}"));
                    }
                    if let Some(height) = patch.height {
                        line.push_str(&format!(" height={height}"));
                    }
                    if let Some(adjustment_kind) = &patch.adjustment_kind {
                        line.push_str(&format!(" kind={}", quote(adjustment_kind)));
                    }
                    line
                }
                RasterOperation::MoveLayer { layer_id, parent_id, index } => {
                    let mut line = format!("move-layer id={} index={index}", quote(layer_id));
                    if let Some(parent) = parent_id {
                        line.push_str(&format!(" parent={}", quote(parent)));
                    }
                    line
                }
                RasterOperation::SetCamera { camera } => format!("set-camera x={} y={} zoom={}", fmt_num(camera.x), fmt_num(camera.y), fmt_num(camera.zoom)),
                RasterOperation::ReplaceDocument { document } => format!("replace-document {}", print_document(document, false)),
            }
        }
        //#endregion Printer
    }

    impl DocumentDsl for RasterProjection {
        const EXTENSION: &'static str = "raster";

        fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
            raster_text::parse_document(text)
        }

        fn print_dsl(&self) -> String {
            raster_text::print_document(self, true)
        }
    }
    //#endregion 🔖Dsl

    //#region 🔖OpText
    impl OpText for RasterOperation {
        fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
            raster_text::parse_operation(line)
        }

        fn print_op(&self) -> String {
            raster_text::print_operation(self)
        }
    }
    //#endregion 🔖OpText

    //#region 🔖WasmDocumentVcs
    #[cfg(target_arch = "wasm32")]
    use std::cell::RefCell;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::prelude::*;
    
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub struct RasterDocumentVcs {
        store: RefCell<RasterStore>,
    }
    
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    impl RasterDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<RasterDocumentVcs, JsValue> {
            let envelope: RasterEnvelope =
                serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self {
                store: RefCell::new(RasterStore::new(envelope)),
            })
        }
    
        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    
        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    
        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }
    
        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
    //#endregion 🔖WasmDocumentVcs
    
    //#region 🧪DocumentVcsTests
    #[cfg(test)]
    mod raster_vcs_tests {
        use super::*;
        use vcs::{apply_operation, create_document_vcs_envelope, DocumentVcsCommand};
    
        fn pixel_layer(id: &str, name: &str) -> RasterLayerNode {
            RasterLayerNode::Pixel {
                id: id.into(),
                name: name.into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(512),
                height: Some(512),
                image_key: None,
            }
        }
    
        fn round_trip(projection: &RasterProjection, operation: &RasterOperation) -> RasterProjection {
            let forward = apply_operation(projection, operation);
            let mut restored = forward.clone();
            for back in operation.backwards(projection) {
                restored = apply_operation(&restored, &back);
            }
            assert_eq!(&restored, projection, "backwards() must restore the pre-operation projection");
            forward
        }
    
        #[test]
        fn add_remove_patch_layer_round_trip() {
            let projection = empty_raster_projection();
            let added = round_trip(&projection, &RasterOperation::AddLayer { parent_id: None, index: 0, layer: pixel_layer("l1", "Base") });
            assert_eq!(added.layers.len(), 1);
            let patched = round_trip(
                &added,
                &RasterOperation::PatchLayer { layer_id: "l1".into(), patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() } },
            );
            assert_eq!(layer_name(&patched.layers[0]), "Renamed");
            assert!(!layer_visible(&patched.layers[0]));
            let removed = round_trip(&patched, &RasterOperation::RemoveLayer { layer_id: "l1".into() });
            assert!(removed.layers.is_empty());
        }
    
        #[test]
        fn move_layer_into_group_round_trip() {
            let mut projection = empty_raster_projection();
            projection.layers.push(RasterLayerNode::Group {
                id: "g1".into(),
                name: "Group".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                children: Vec::new(),
            });
            projection.layers.push(pixel_layer("l1", "Base"));
            let moved = round_trip(&projection, &RasterOperation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g1".into()), index: 0 });
            let RasterLayerNode::Group { children, .. } = &moved.layers[0] else { panic!("expected group") };
            assert_eq!(children.len(), 1);
            assert_eq!(layer_node_id(&children[0]), "l1");
        }
    
        #[test]
        fn set_camera_and_replace_round_trip() {
            let projection = empty_raster_projection();
            let next = round_trip(&projection, &RasterOperation::SetCamera { camera: RasterCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
            assert_eq!(next.camera.zoom, 2.0);
            let mut replacement = empty_raster_projection();
            replacement.layers.push(pixel_layer("l9", "Replaced"));
            let replaced = round_trip(&projection, &RasterOperation::ReplaceDocument { document: replacement.clone() });
            assert_eq!(replaced, replacement);
        }
    
        #[test]
        fn store_applies_layer_add() {
            let mut store = RasterStore::new(create_document_vcs_envelope(
                RASTER_DOCUMENT_SCHEMA,
                "raster",
                empty_raster_projection(),
                None,
            ));
            store
                .dispatch(DocumentVcsCommand::Apply {
                    operations: vec![RasterOperation::AddLayer { parent_id: None, index: 0, layer: pixel_layer("l1", "Base") }],
                    description: None,
                })
                .expect("apply");
            assert_eq!(store.projection().expect("projection").layers.len(), 1);
        }
    }
    //#endregion 🧪DocumentVcsTests
    //#endregion 🔖DocumentVcs
}
//#endregion 🔖Domain


use semio_framework_plugin::{SurfaceKind,
    build_paint_2d_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor,
    AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp, DocumentView, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder,
    Paint2dScene, ResourceKindSpec, UtilityCategory, UtilityDefinition, WindowMeasure, is_de_locale, localized_label_map, resolve_labels,
    selection_ids, tree_item_with_action,
    UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode, ViewState,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    create_default_layout, SET_ACTIVE_UTILITY_ACTION_ID,
};
use crate::domain::{
    empty_raster_projection, find_layer, flatten_raster_layers, layer_name, layer_node_id, layer_visible,
    RasterCamera, RasterImageAsset, RasterLayerNode, RasterLayerPatch, RasterOperation,
    RasterProjection as RasterDocument, RasterTransform,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖Constants
const RASTER_PLAY_APP_ID: &str = "raster-play";
const RASTER_PLAY_CONTROLLER_ID: &str = "raster-play";
const RASTER_PLAY_SURFACE_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_SURFACE_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_BODY_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_BODY_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
const RASTER_PLAY_BODY_CATALOGUE: &str = "raster.play.catalogue";
const RASTER_PLAY_BODY_MASKS: &str = "raster.play.masks";
const RASTER_PLAY_BODY_PROPERTIES: &str = "raster.play.properties";
const RASTER_PLAY_WINDOW_COMPOSITE: &str = "raster-composite";
const RASTER_PLAY_WINDOW_NAVIGATOR: &str = "raster-navigator";
const RASTER_PLAY_MASKS_TAB_ID: &str = "raster.panel.masks";
const RASTER_DOCUMENT_SCHEMA: &str = "raster.document";
const RASTER_TREE_PREFIX: &str = "raster-play-layers";
/// 🧰 Fallback utility when the host has not yet asserted a session active utility for the composite window.
const RASTER_DEFAULT_UTILITY: &str = "selectMarquee";

/// 📄 The `semio` example document, handcrafted in the `.raster` DSL (see `🔖Dsl`) instead of JSON —
/// {@link semio_example_document}/{@link semio_example_json} are the only ways it should be consumed.
const SEMIO_RASTER_EXAMPLE_TEXT: &str = include_str!("../../example/semio.raster");

static RASTER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
/// 🎛️ Ephemeral view state (selection, hover, utility/brush settings, navigator viewport) held in the
/// app struct — never in the document — so it stays out of undo history and off the operation channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterPlayRuntime {
    selected_ids: Vec<String>,
    hovered_id: Option<String>,
    brush_size: f32,
    brush_opacity: f32,
    composite_viewport: Option<RasterViewportSize>,
}

impl RasterPlayRuntime {
    fn new() -> Self {
        Self {
            selected_ids: Vec::new(),
            hovered_id: None,
            brush_size: 24.0,
            brush_opacity: 1.0,
            composite_viewport: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RasterViewportSize {
    width: f64,
    height: f64,
}

fn create_raster_id(prefix: &str) -> String {
    let next = RASTER_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

fn create_pixel_layer(name: &str, width: u32, height: u32) -> RasterLayerNode {
    RasterLayerNode::Pixel {
        id: create_raster_id("layer"),
        name: name.into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        width: Some(width),
        height: Some(height),
        image_key: None,
    }
}

fn create_group_layer() -> RasterLayerNode {
    RasterLayerNode::Group {
        id: create_raster_id("group"),
        name: "Group".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        mask: None,
        children: Vec::new(),
    }
}

fn create_adjustment_layer() -> RasterLayerNode {
    RasterLayerNode::Adjustment {
        id: create_raster_id("adjust"),
        name: "Adjustment".into(),
        visible: true,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: RasterTransform::default(),
        adjustment_kind: "brightnessContrast".into(),
        params: serde_json::Map::new(),
    }
}

fn create_layer_of_kind(kind: &str) -> RasterLayerNode {
    match kind {
        "group" => create_group_layer(),
        "adjustment" => create_adjustment_layer(),
        _ => create_pixel_layer("Layer", 512, 512),
    }
}

fn empty_raster_document() -> RasterDocument {
    let mut document = empty_raster_projection();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

/// 📄 The `semio` example, parsed once from {@link SEMIO_RASTER_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
fn semio_example_document() -> RasterDocument {
    <RasterDocument as vcs::DocumentDsl>::parse_dsl(SEMIO_RASTER_EXAMPLE_TEXT).unwrap_or_else(|_| empty_raster_document())
}

/// 📄 JSON re-serialization of {@link semio_example_document}, for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`) — out of scope to change,
/// since it is defined in `framework/plugin`.
fn semio_example_json() -> String {
    serde_json::to_string(&semio_example_document()).expect("serialize semio example document")
}

fn layer_row_id(layer: &RasterLayerNode) -> String {
    let segment = match layer {
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
        RasterLayerNode::Pixel { .. } => "layer",
    };
    format!("{RASTER_TREE_PREFIX}.{segment}.{}", layer_node_id(layer))
}

fn layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    row_id
        .strip_prefix(&format!("{RASTER_TREE_PREFIX}."))
        .and_then(|rest| rest.split('.').nth(1))
        .map(str::to_string)
}

fn mask_row_id(target_id: &str) -> String {
    format!("{RASTER_TREE_PREFIX}.mask.{target_id}")
}

/// 📄 Duplicates a layer subtree with freshly minted ids (a new document node, not an operation inverse).
fn clone_layer(layer: &RasterLayerNode) -> RasterLayerNode {
    match layer {
        RasterLayerNode::Pixel { name, visible, opacity, blend_mode, transform, mask, width, height, image_key, .. } => {
            RasterLayerNode::Pixel {
                id: create_raster_id("layer"),
                name: format!("{name} copy"),
                visible: *visible,
                opacity: *opacity,
                blend_mode: blend_mode.clone(),
                transform: transform.clone(),
                mask: mask.clone(),
                width: *width,
                height: *height,
                image_key: image_key.clone(),
            }
        }
        RasterLayerNode::Group { name, visible, opacity, blend_mode, transform, mask, children, .. } => RasterLayerNode::Group {
            id: create_raster_id("group"),
            name: format!("{name} copy"),
            visible: *visible,
            opacity: *opacity,
            blend_mode: blend_mode.clone(),
            transform: transform.clone(),
            mask: mask.clone(),
            children: children.iter().map(clone_layer).collect(),
        },
        RasterLayerNode::Adjustment { name, visible, opacity, blend_mode, transform, adjustment_kind, params, .. } => {
            RasterLayerNode::Adjustment {
                id: create_raster_id("adjust"),
                name: format!("{name} copy"),
                visible: *visible,
                opacity: *opacity,
                blend_mode: blend_mode.clone(),
                transform: transform.clone(),
                adjustment_kind: adjustment_kind.clone(),
                params: params.clone(),
            }
        }
    }
}

/// 🩹 Builds a sparse {@link RasterLayerPatch} for a `patchLayer`/`patchLayers` field write.
fn layer_patch_for_field(field: &str, value: &Value, prior: &RasterLayerNode) -> Option<RasterLayerPatch> {
    let mut patch = RasterLayerPatch::default();
    let opacity_of = crate::domain::layer_opacity(prior) as f64;
    match field {
        "name" => patch.name = Some(value.as_str().unwrap_or("").into()),
        "visible" => patch.visible = Some(value.as_bool().unwrap_or_else(|| !layer_visible(prior))),
        "opacity" => patch.opacity = Some(value.as_f64().unwrap_or(opacity_of) as f32),
        "blendMode" => patch.blend_mode = Some(value.as_str().unwrap_or("normal").into()),
        "transformX" => patch.transform_x = Some(value.as_f64().unwrap_or(0.0)),
        "transformY" => patch.transform_y = Some(value.as_f64().unwrap_or(0.0)),
        "width" => patch.width = Some(value.as_u64().unwrap_or(512) as u32),
        "height" => patch.height = Some(value.as_u64().unwrap_or(512) as u32),
        "adjustmentKind" => patch.adjustment_kind = Some(value.as_str().unwrap_or("brightnessContrast").into()),
        _ => return None,
    }
    Some(patch)
}
//#endregion 🔖Document

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the raster app; one field per label makes every locale combination compile-checked.
semio_framework_plugin::app_labels! {
    struct RasterPlayLabels {
        masks: &'static str = en: "Masks", de: "Masken";
        no_masks: &'static str = en: "No masks", de: "Keine Masken";
        mask_suffix: &'static str = en: "mask", de: "Maske";
        add_pixel: &'static str = en: "Add Pixel", de: "Pixel hinzufügen";
        add_group: &'static str = en: "Add Group", de: "Gruppe hinzufügen";
        layer_kinds: &'static str = en: "Layer kinds", de: "Ebenenarten";
        layer: &'static str = en: "Layer", de: "Ebene";
        catalogue_pixel: &'static str = en: "pixel — paintable bitmap layer", de: "pixel — bearbeitbare Bitmap-Ebene";
        catalogue_group: &'static str = en: "group — nested layer stack", de: "group — verschachtelter Ebenenstapel";
        catalogue_adjustment: &'static str = en: "adjustment — non-destructive filter", de: "adjustment — zerstörungsfreier Filter";
        window_composite: &'static str = en: "Composite", de: "Komposit";
        window_navigator: &'static str = en: "Navigator", de: "Navigator";
        name: &'static str = en: "Name", de: "Name";
        opacity: &'static str = en: "Opacity", de: "Deckkraft";
        mixed: &'static str = en: "Mixed", de: "Gemischt";
        schema_prefix: &'static str = en: "Schema", de: "Schema";
        brush_prefix: &'static str = en: "Brush", de: "Pinsel";
    }
}

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_raster_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the builder chain.
fn raster_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("addLayer", "Add Layer", "Ebene hinzufügen"),
        ("setDocument", "Set Document", "Dokument festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setCameraZoom", "Set Camera Zoom", "Kamerazoom festlegen"),
        ("setLayerVisible", "Set Layer Visible", "Ebenensichtbarkeit festlegen"),
        ("toggleLayerVisible", "Toggle Layer Visible", "Ebenensichtbarkeit umschalten"),
        ("dropLayerKind", "Drop Layer Kind", "Ebenenart ablegen"),
        ("deleteLayer", "Delete Layer", "Ebene löschen"),
        ("duplicateLayer", "Duplicate Layer", "Ebene duplizieren"),
        ("patchLayer", "Patch Layer", "Ebene aktualisieren"),
        ("patchLayers", "Patch Layers", "Ebenen aktualisieren"),
        ("moveLayer", "Move Layer", "Ebene verschieben"),
        ("selectAll", "Select All", "Alles auswählen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("setBrushSize", "Set Brush Size", "Pinselgröße festlegen"),
        ("setBrushOpacity", "Set Brush Opacity", "Pinseldeckkraft festlegen"),
        ("setCompositeViewport", "Set Composite Viewport", "Komposit-Ansichtsfenster festlegen"),
    ])
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_raster_app`.
fn raster_utility_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("selectMarquee", "Marquee Select", "Rahmenauswahl"),
        ("paintBrush", "Brush", "Pinsel"),
        ("paintEraser", "Eraser", "Radiergummi"),
    ])
}
//#endregion 🔖CommandLabels
//#endregion 🔖Terminology

//#region 🔖Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args,
    }
}

fn selection_from_view(view_state: &ViewState) -> Vec<String> {
    view_state
        .selection_json
        .as_ref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| {
            value.as_array().map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
        })
        .unwrap_or_default()
}

fn selection_from_runtime(runtime: &RasterPlayRuntime, view_state: &ViewState) -> Vec<String> {
    if !runtime.selected_ids.is_empty() {
        return runtime.selected_ids.clone();
    }
    selection_from_view(view_state)
}

fn layer_tree_item(layer: &RasterLayerNode) -> UiTreeItemNode {
    let nested = match layer {
        RasterLayerNode::Group { children, .. } => {
            if children.is_empty() {
                None
            } else {
                Some(children.iter().map(layer_tree_item).collect())
            }
        }
        _ => None,
    };
    let description = match layer {
        RasterLayerNode::Pixel { .. } => "pixel",
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
    };
    let icon_id = match layer {
        RasterLayerNode::Pixel { .. } => "image",
        RasterLayerNode::Group { .. } => "folder",
        RasterLayerNode::Adjustment { .. } => "sliders-horizontal",
    };
    UiTreeItemNode {
        icon_id: Some(icon_id.into()),
        default_open: Some(matches!(layer, RasterLayerNode::Group { .. })),
        draggable: Some(true),
        items: nested,
        dimmed: if layer_visible(layer) { None } else { Some(true) },
        ..tree_item_with_action(
            layer_row_id(layer),
            layer_name(layer),
            Some(description.into()),
            play_action(RASTER_PLAY_CONTROLLER_ID, "setSelection", Some(json!({ "ids": [layer_node_id(layer)] }))),
        )
    }
}

fn render_layers_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterPlayLabels) -> UiNode {
    let action_rows = vec![
        UiTreeItemNode {
            icon_id: Some("image".into()),
            ..tree_item_with_action(
                format!("{RASTER_TREE_PREFIX}.add.pixel"),
                labels.add_pixel,
                None,
                play_action(RASTER_PLAY_CONTROLLER_ID, "addLayer", Some(json!({ "kind": "pixel" }))),
            )
        },
        UiTreeItemNode {
            icon_id: Some("folder-plus".into()),
            ..tree_item_with_action(
                format!("{RASTER_TREE_PREFIX}.add.group"),
                labels.add_group,
                None,
                play_action(RASTER_PLAY_CONTROLLER_ID, "addLayer", Some(json!({ "kind": "group" }))),
            )
        },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    let selected_ids: Vec<String> = selection_from_runtime(runtime, view_state)
        .iter()
        .filter_map(|id| find_layer(&document.layers, id).map(layer_row_id))
        .collect();
    let highlighted_ids: Vec<String> = runtime
        .hovered_id
        .as_deref()
        .and_then(|id| find_layer(&document.layers, id))
        .map(|layer| vec![layer_row_id(layer)])
        .unwrap_or_default();
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)
        .section(RASTER_TREE_PREFIX, Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, [action_rows, layer_items].concat())
        .selected(selected_ids)
        .highlighted(highlighted_ids)
        .selection_change(play_action(RASTER_PLAY_CONTROLLER_ID, "setSelection", None))
        .build()
}

fn render_masks_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterPlayLabels) -> UiNode {
    let mut items = Vec::new();
    fn collect_masks(layer: &RasterLayerNode, items: &mut Vec<UiTreeItemNode>, labels: &RasterPlayLabels) {
        if let RasterLayerNode::Pixel { id, name, mask, .. }
        | RasterLayerNode::Group { id, name, mask, .. } = layer
        {
            if mask.as_ref().is_some_and(|mask| mask.enabled) {
                items.push(UiTreeItemNode {
                    icon_id: Some("scan".into()),
                    ..tree_item_with_action(
                        mask_row_id(id),
                        format!("{name} {}", labels.mask_suffix),
                        Some("mask".into()),
                        play_action(RASTER_PLAY_CONTROLLER_ID, "setSelection", Some(json!({ "ids": [id] }))),
                    )
                });
            }
        }
        if let RasterLayerNode::Group { children, .. } = layer {
            for child in children {
                collect_masks(child, items, labels);
            }
        }
    }
    for layer in &document.layers {
        collect_masks(layer, &mut items, labels);
    }
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)
        .section_or_placeholder("raster-play-masks", Some(labels.masks.into()), true, items, labels.no_masks)
        .selected(
            selection_from_runtime(runtime, view_state)
                .iter()
                .map(|id| mask_row_id(id))
                .collect(),
        )
        .build()
}

fn render_catalogue_panel(labels: &RasterPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "raster-catalogue".into(),
        label: Some(labels.layer_kinds.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![
            ui_text(labels.catalogue_pixel),
            ui_text(labels.catalogue_group),
            ui_text(labels.catalogue_adjustment),
        ],
    }])
}

fn render_properties_panel(document: &RasterDocument, runtime: &RasterPlayRuntime, view_state: &ViewState, labels: &RasterPlayLabels) -> UiNode {
    let selected = selection_from_runtime(runtime, view_state);
    let layers: Vec<&RasterLayerNode> = selected
        .iter()
        .filter_map(|id| find_layer(&document.layers, id))
        .collect();
    if layers.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("{}: {}", labels.schema_prefix, document.schema)),
            ui_text(format!("{}: {} @ {}", labels.brush_prefix, runtime.brush_size, runtime.brush_opacity)),
        ]);
    }
    let names: Vec<String> = layers.iter().map(|layer| layer_name(*layer).into()).collect();
    let opacities: Vec<f64> = layers.iter().map(|layer| crate::domain::layer_opacity(layer) as f64).collect();
    let mixed_name = ui_inspector_mixed_text(&names);
    let mixed_opacity = ui_inspector_mixed_number(&opacities);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
        id: "raster-properties.layer".into(),
        label: labels.layer.into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field(
                "raster-properties.name",
                labels.name,
                mixed_name.placeholder.unwrap_or(mixed_name.value),
            ),
            ui_inspector_readonly_field(
                "raster-properties.opacity",
                labels.opacity,
                if mixed_opacity.uniform {
                    mixed_opacity.value.to_string()
                } else {
                    labels.mixed.to_string()
                },
            ),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Render
/// 📡 Document JSON for the WASM compositor, omitting embedded assets/camera/utility/brush — mirrors premigration `rasterDocumentToSyncJson`.
fn document_sync_json(document: &RasterDocument) -> String {
    let mut value = serde_json::to_value(document).unwrap_or(Value::Null);
    if let Value::Object(ref mut map) = value {
        map.remove("assets");
        map.remove("camera");
        map.remove("brushSize");
        map.remove("brushOpacity");
    }
    value.to_string()
}

fn raster_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str, view_mode: &str) -> Paint2dScene {
    Paint2dScene {
        document_sync_json: document_sync_json(document),
        assets_json: serde_json::to_string(&document.assets).unwrap_or_else(|_| "{}".into()),
        camera_json: serde_json::to_string(&document.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into()),
        selection_json: serde_json::to_string(&runtime.selected_ids).unwrap_or_else(|_| "[]".into()),
        hovered_id: runtime.hovered_id.clone(),
        active_utility: active_utility.into(),
        brush_size: runtime.brush_size as f64,
        brush_opacity: runtime.brush_opacity as f64,
        view_mode: view_mode.into(),
        composite_viewport_json: runtime
            .composite_viewport
            .as_ref()
            .map(|viewport| serde_json::to_string(viewport).unwrap_or_else(|_| "{}".into())),
    }
}

fn render_composite_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str) -> UiNode {
    build_paint_2d_scene(RASTER_PLAY_SURFACE_COMPOSITE, RASTER_PLAY_CONTROLLER_ID, raster_scene(document, runtime, active_utility, "composite"))
}

fn render_navigator_scene(document: &RasterDocument, runtime: &RasterPlayRuntime, active_utility: &str) -> UiNode {
    build_paint_2d_scene(RASTER_PLAY_SURFACE_NAVIGATOR, RASTER_PLAY_CONTROLLER_ID, raster_scene(document, runtime, active_utility, "navigator"))
}
//#endregion 🔖Render

//#region 🔖RasterPlayApp
#[derive(Default)]
struct RasterPlayApp {
    runtime: RasterPlayRuntime,
}

impl RasterPlayApp {
    /// 🩹 Builds `PatchLayer` operations for a `patchLayer`/`patchLayers` field write across ids.
    fn patch_layer_operations(&self, document: &RasterDocument, layer_ids: &[String], field: &str, value: &Value) -> Vec<RasterOperation> {
        layer_ids
            .iter()
            .filter_map(|layer_id| {
                let prior = find_layer(&document.layers, layer_id)?;
                let patch = layer_patch_for_field(field, value, prior)?;
                Some(RasterOperation::PatchLayer { layer_id: layer_id.clone(), patch })
            })
            .collect()
    }
}

impl Default for RasterPlayRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn raster_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    play_action(RASTER_PLAY_CONTROLLER_ID, action, args)
}

fn raster_paint_utility_options(runtime: &RasterPlayRuntime, utility: &str, label: &str) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("raster-utility-options-{utility}"),
        label: label.into(),
        default_open: Some(true),
        active_utility_id: Some(utility.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Slider {
                id: format!("raster-{utility}-size"),
                label: Some("Size".into()),
                value: runtime.brush_size as f64,
                min: 1.0,
                max: 128.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: raster_action("setBrushSize", None),
            },
            WindowMeasure::Slider {
                id: format!("raster-{utility}-opacity"),
                label: Some("Opacity".into()),
                value: runtime.brush_opacity as f64,
                min: 0.0,
                max: 1.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: raster_action("setBrushOpacity", None),
            },
        ],
    }
}

fn raster_window_measures(runtime: &RasterPlayRuntime) -> Vec<WindowMeasure> {
    vec![
        raster_paint_utility_options(runtime, "paintBrush", "Brush"),
        raster_paint_utility_options(runtime, "paintEraser", "Eraser"),
    ]
}

impl DocumentApp for RasterPlayApp {
    type Projection = RasterDocument;
    type Operation = RasterOperation;

    fn app_id(&self) -> &str {
        RASTER_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        RASTER_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> RasterDocument {
        empty_raster_document()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, RasterDocument>,
        _view_state: &ViewState,
    ) -> ActionEmit<RasterOperation> {
        let document = doc.projection;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no operations.
            "setBrushSize" => {
                if let Some(size) = args.and_then(|value| value.get("value").or_else(|| value.get("brushSize"))).and_then(|value| value.as_f64()) {
                    self.runtime.brush_size = size as f32;
                }
                ActionEmit::default()
            }
            "setBrushOpacity" => {
                if let Some(opacity) = args.and_then(|value| value.get("value").or_else(|| value.get("opacity"))).and_then(|value| value.as_f64()) {
                    self.runtime.brush_opacity = (opacity as f32).clamp(0.0, 1.0);
                }
                ActionEmit::default()
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host-owned utility switch: the active utility lives in session view state (never the
                // document). There is no plugin-side paint scratch to clear — brush strokes are painted
                // host-side in the WASM canvas — so this simply acknowledges with no operations or history.
                ActionEmit::default()
            }
            "setSelection" => {
                self.runtime.selected_ids = selection_ids(args);
                ActionEmit::default()
            }
            "setHover" => {
                self.runtime.hovered_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                ActionEmit::default()
            }
            "setCompositeViewport" => {
                if let (Some(width), Some(height)) = (
                    args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()),
                    args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()),
                ) {
                    self.runtime.composite_viewport = Some(RasterViewportSize { width, height });
                }
                ActionEmit::default()
            }
            "selectAll" => {
                self.runtime.selected_ids = flatten_raster_layers(&document.layers)
                    .into_iter()
                    .map(|layer| layer_node_id(layer).to_string())
                    .collect();
                ActionEmit::default()
            }
            // 📷 Camera — a coalesced scalar operation so a pan/zoom gesture is one undo step.
            "setCamera" | "setCameraZoom" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")).and_then(|value| serde_json::from_value::<RasterCamera>(value.clone()).ok()) {
                    return ActionEmit { operations: vec![RasterOperation::SetCamera { camera }], coalesce_key: Some("camera".into()), ..Default::default() };
                }
                if let Some(zoom) = args.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()) {
                    let camera = RasterCamera { zoom, ..document.camera.clone() };
                    return ActionEmit { operations: vec![RasterOperation::SetCamera { camera }], coalesce_key: Some("camera".into()), ..Default::default() };
                }
                ActionEmit::default()
            }
            // ✏️ Operations — dispatched as VCS operations with a true inverse.
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let replacement = if example_id == "semio" {
                    semio_example_document()
                } else {
                    empty_raster_document()
                };
                self.runtime.selected_ids.clear();
                ActionEmit::operations(vec![RasterOperation::ReplaceDocument { document: replacement }])
            }
            "setDocument" => match args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<RasterDocument>(value.clone()).ok()) {
                Some(replacement) => ActionEmit::operations(vec![RasterOperation::ReplaceDocument { document: replacement }]),
                None => ActionEmit::default(),
            },
            "setLayerVisible" | "toggleLayerVisible" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                let Some(layer) = find_layer(&document.layers, target_id) else { return ActionEmit::default() };
                let visible = args
                    .and_then(|value| value.get("visible"))
                    .and_then(|value| value.as_bool())
                    .unwrap_or_else(|| !layer_visible(layer));
                ActionEmit::operations(vec![RasterOperation::PatchLayer {
                    layer_id: target_id.into(),
                    patch: RasterLayerPatch { visible: Some(visible), ..Default::default() },
                }])
            }
            "addLayer" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
                let layer = create_layer_of_kind(kind);
                self.runtime.selected_ids = vec![layer_node_id(&layer).to_string()];
                ActionEmit::operations(vec![RasterOperation::AddLayer { parent_id: None, index: document.layers.len(), layer }])
            }
            "dropLayerKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("pixel");
                let layer = create_layer_of_kind(kind);
                self.runtime.selected_ids = vec![layer_node_id(&layer).to_string()];
                ActionEmit::operations(vec![RasterOperation::AddLayer { parent_id: None, index: document.layers.len(), layer }])
            }
            "deleteLayer" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                if find_layer(&document.layers, target_id).is_none() {
                    return ActionEmit::default();
                }
                self.runtime.selected_ids.retain(|id| id != target_id);
                ActionEmit::operations(vec![RasterOperation::RemoveLayer { layer_id: target_id.into() }])
            }
            "duplicateLayer" => {
                let Some(target_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                match find_layer(&document.layers, target_id) {
                    Some(layer) => {
                        let copy = clone_layer(layer);
                        self.runtime.selected_ids = vec![layer_node_id(&copy).to_string()];
                        ActionEmit::operations(vec![RasterOperation::AddLayer { parent_id: None, index: document.layers.len(), layer: copy }])
                    }
                    None => ActionEmit::default(),
                }
            }
            "patchLayer" => {
                let layer_id = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if layer_id.is_empty() || field.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::operations(self.patch_layer_operations(document, &[layer_id.to_string()], field, &value))
            }
            "patchLayers" => {
                let layer_ids: Vec<String> = args
                    .and_then(|value| value.get("layerIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if field.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::operations(self.patch_layer_operations(document, &layer_ids, field, &value))
            }
            "moveLayer" => {
                let Some(layer_id) = args.and_then(|value| value.get("layerId")).and_then(|value| value.as_str()) else {
                    return ActionEmit::default();
                };
                if find_layer(&document.layers, layer_id).is_none() {
                    return ActionEmit::default();
                }
                let target_row_id = args.and_then(|value| value.get("targetRowId")).and_then(|value| value.as_str()).unwrap_or("raster-play-layers");
                let drop_position = args.and_then(|value| value.get("dropPosition")).and_then(|value| value.as_str()).unwrap_or("after");
                let parent_id = layer_id_from_tree_row_id(target_row_id).and_then(|id| {
                    find_layer(&document.layers, &id).and_then(|entry| matches!(entry, RasterLayerNode::Group { .. }).then_some(id))
                });
                let index = if drop_position == "before" {
                    0
                } else if let Some(parent) = &parent_id {
                    match find_layer(&document.layers, parent) {
                        Some(RasterLayerNode::Group { children, .. }) => children.len(),
                        _ => 0,
                    }
                } else {
                    document.layers.len()
                };
                ActionEmit::operations(vec![RasterOperation::MoveLayer { layer_id: layer_id.into(), parent_id, index }])
            }
            _ => ActionEmit::default(),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, RasterDocument>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let measures = raster_window_measures(&self.runtime);
        HashMap::from([(RASTER_PLAY_WINDOW_COMPOSITE.into(), measures)])
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RasterDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<RasterPlayLabels>(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(RASTER_DEFAULT_UTILITY);
        match body_key {
            RASTER_PLAY_BODY_COMPOSITE => render_composite_scene(document, &self.runtime, active_utility),
            RASTER_PLAY_BODY_NAVIGATOR => render_navigator_scene(document, &self.runtime, active_utility),
            RASTER_PLAY_BODY_LAYERS => render_layers_panel(document, &self.runtime, view_state, labels),
            RASTER_PLAY_BODY_MASKS => render_masks_panel(document, &self.runtime, view_state, labels),
            RASTER_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            RASTER_PLAY_BODY_PROPERTIES => render_properties_panel(document, &self.runtime, view_state, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<RasterPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(RASTER_PLAY_WINDOW_COMPOSITE, labels.window_composite)
            .window_kind_label(RASTER_PLAY_WINDOW_NAVIGATOR, labels.window_navigator)
            .panel_tab_label(RASTER_PLAY_MASKS_TAB_ID, labels.masks)
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .action_labels(raster_action_labels(is_de))
            .utility_labels(raster_utility_labels(is_de))
            .example_labels(HashMap::from([("semio".to_string(), "Semio".to_string())]))
    }
}
//#endregion 🔖RasterPlayApp

//#region 🔖Manifest
/// 🛠️ An internal (non-palette) action declaration — the panel/pointer/gesture-bound vocabulary
/// dispatched by the layer tree, catalogue drops, camera and inspector, never a palette command.
fn raster_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, kind) }
}

/// 🧰 One composite-window utility declaration; ids must stay host-compatible (`paint*` prefix paints,
/// `paintEraser` erases, `selectMarquee` selects) because the scene's active utility feeds `RasterHost`.
fn raster_utility(id: &str, label: &str, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

fn create_raster_app() -> App {
    App::from_builder(
        App::builder(RASTER_PLAY_APP_ID, "Raster").document(["semio", "raster"])
            .resource_kind(ResourceKindSpec {
                id: "2d.raster".into(),
                name: "2D Raster".into(),
                source_format: "raster.document".into(),
                component_kind: "raster".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "raster.document".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            .icon_id("raster")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(RASTER_PLAY_WINDOW_COMPOSITE, "Composite", RASTER_PLAY_BODY_COMPOSITE, SurfaceKind::Paint2d, "layers")
            .window_kind(RASTER_PLAY_WINDOW_NAVIGATOR, "Navigator", RASTER_PLAY_BODY_NAVIGATOR, SurfaceKind::Paint2d, "focus")
            .default_layout(create_default_layout(
                &[RASTER_PLAY_WINDOW_COMPOSITE.into(), RASTER_PLAY_WINDOW_NAVIGATOR.into()],
                "row",
                Some(&[72.0, 28.0]),
                Some(&["Composite".into(), "Navigator".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                RASTER_PLAY_BODY_LAYERS,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                RASTER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(RASTER_PLAY_MASKS_TAB_ID, "Masks", PanelGroup::Workbench, RASTER_PLAY_BODY_MASKS)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                RASTER_PLAY_BODY_PROPERTIES,
            )
            // ✏️ Palette-visible content operations.
            .operation("addLayer", "Add Layer")
            .operation("setDocument", "Set Document")
            .operation("setActiveExample", "Set Active Example")
            // 🔧 Internal content operations — layer-tree / catalogue-drop / camera / inspector bound.
            .action_with(raster_internal_action("setCamera", "Set Camera", ActionKind::Operation))
            .action_with(raster_internal_action("setCameraZoom", "Set Camera Zoom", ActionKind::Operation))
            .action_with(raster_internal_action("setLayerVisible", "Set Layer Visible", ActionKind::Operation))
            .action_with(raster_internal_action("toggleLayerVisible", "Toggle Layer Visible", ActionKind::Operation))
            .action_with(raster_internal_action("dropLayerKind", "Drop Layer Kind", ActionKind::Operation))
            .action_with(raster_internal_action("deleteLayer", "Delete Layer", ActionKind::Operation))
            .action_with(raster_internal_action("duplicateLayer", "Duplicate Layer", ActionKind::Operation))
            .action_with(raster_internal_action("patchLayer", "Patch Layer", ActionKind::Operation))
            .action_with(raster_internal_action("patchLayers", "Patch Layers", ActionKind::Operation))
            .action_with(raster_internal_action("moveLayer", "Move Layer", ActionKind::Operation))
            // 👁️ Ephemeral view state — selection, hover, live brush controls, navigator viewport.
            .view_action("selectAll", "Select All")
            .action_with(raster_internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(raster_internal_action("setHover", "Set Hover", ActionKind::View))
            .action_with(raster_internal_action("setBrushSize", "Set Brush Size", ActionKind::View))
            .action_with(raster_internal_action("setBrushOpacity", "Set Brush Opacity", ActionKind::View))
            .action_with(raster_internal_action("setCompositeViewport", "Set Composite Viewport", ActionKind::View))
            // 📝 Staged palette-form arguments for the two palette operations.
            .action_args("addLayer", vec![
                ActionArgDef::select("kind", "Layer Kind", vec![
                    ActionArgOption::new("pixel", "Pixel"),
                    ActionArgOption::new("group", "Group"),
                    ActionArgOption::new("adjustment", "Adjustment"),
                ]).required().default_value("pixel"),
            ])
            .action_args("setDocument", vec![
                ActionArgDef::text("document", "Document"),
            ])
            // 🧰 Composite-window utilities — one exclusive set, active utility host-owned (never a document operation).
            .utility(raster_utility("selectMarquee", "Marquee Select", "square-dashed", "Select", UtilityCategory::Selection))
            .utility(raster_utility("paintBrush", "Brush", "brush", "Paint", UtilityCategory::Utilities))
            .utility(raster_utility("paintEraser", "Eraser", "eraser", "Paint", UtilityCategory::Utilities))
            .window_kind_utilities(RASTER_PLAY_WINDOW_COMPOSITE, vec![
                "selectMarquee".into(), "paintBrush".into(), "paintEraser".into(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("semio", "Semio", semio_example_json())
    .program("raster", "Raster", "2d.raster")
}

fn raster_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Raster", 1024, 1024)
}

/// 📥 Rasterizes a DWG drawing's flat SVG projection into a single-layer raster document.
fn raster_document_json_from_dwg(drawing: &semio_framework_os::DwgDrawing) -> Result<Value, String> {
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let data = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height)?;
    let asset_key = create_raster_id("dwg-asset");
    let mut layer = create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let mut assets = HashMap::new();
    assets.insert(asset_key, RasterImageAsset { mime: "image/png".into(), data });
    let document = RasterDocument {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: create_raster_id("dwg-import"),
        title: Some("DWG Import".into()),
        camera: RasterCamera::default(),
        layers: vec![layer],
        assets,
    };
    serde_json::to_value(&document).map_err(|error| error.to_string())
}

fn register_raster_exports() {
    semio_framework_os::register_2d_export_handlers("2d.raster", "raster", raster_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.raster", raster_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "raster", label: "Raster", version: "0.1.0",
    setup: register_raster_exports,
    apps: [ create_raster_app => RasterPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};
    use vcs::MemoryBackbone;

    fn semio_app() -> VcsDocumentApp<RasterPlayApp> {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let document = semio_example_document();
        app.load_document(
            &serde_json::to_string(&vcs::create_document_vcs_envelope::<RasterDocument, RasterOperation>(
                RASTER_DOCUMENT_SCHEMA,
                "raster",
                document,
                None,
            ))
            .unwrap(),
        )
        .expect("load semio");
        app
    }

    #[test]
    fn renders_raster_scene() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let node = app.render(RASTER_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("raster"));
    }

    #[test]
    fn renders_navigator_scene() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_NAVIGATOR, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"navigator\""));
    }

    #[test]
    fn parses_semio_example_document() {
        let document = semio_example_document();
        assert!(!document.layers.is_empty());
    }

    #[test]
    fn imports_dwg_polyline_into_raster_document() {
        let mut drawing = semio_framework_os::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity {
            layer,
            color: semio_framework_os::DwgColor::ByLayer,
            geometry: semio_framework_os::DwgGeometry::LwPolyline {
                closed: true,
                elevation: 0.0,
                vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
                bulges: vec![0.0, 0.0, 0.0, 0.0],
            },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [10.0, 10.0, 0.0];
        let value = raster_document_json_from_dwg(&drawing).expect("dwg import");
        let document: RasterDocument = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert_eq!(asset.mime, "image/png");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn imports_empty_dwg_into_blank_raster_document() {
        let drawing = semio_framework_os::DwgDrawing::default();
        let value = raster_document_json_from_dwg(&drawing).expect("empty dwg import");
        let document: RasterDocument = serde_json::from_value(value).expect("valid raster document");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        assert_eq!(*width, Some(1));
        assert_eq!(*height, Some(1));
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = document.assets.get(asset_key).expect("asset present");
        assert!(!asset.data.is_empty());
    }

    #[test]
    fn empty_document_background_layer_has_identity_scale() {
        let document = empty_raster_document();
        let json = document_sync_json(&document);
        assert!(json.contains(r#""scaleX":1.0"#), "expected identity scale in {json}");
        assert!(json.contains(r#""scaleY":1.0"#), "expected identity scale in {json}");
        assert!(!json.contains(r#""scaleX":0.0"#), "layer must not collapse to zero size");
    }

    #[test]
    fn renders_layers_tree() {
        let mut app = semio_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Backdrop"));
    }

    #[test]
    fn raster_labels_resolve_native_english_by_default() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let layers_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(layers_json.contains("Add Pixel"));
        assert!(layers_json.contains("Add Group"));
        let masks_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_MASKS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(masks_json.contains("Masks"));
        assert!(masks_json.contains("No masks"));
        let catalogue_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(catalogue_json.contains("Layer kinds"));
        let properties_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_PROPERTIES, None, &ViewState::default()).expect("render")).unwrap();
        assert!(properties_json.contains("Schema:"));
    }

    #[test]
    fn raster_labels_resolve_german_locale() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let layers_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &view_state).expect("render")).unwrap();
        assert!(layers_json.contains("Pixel hinzufügen"));
        assert!(layers_json.contains("Gruppe hinzufügen"));
        let masks_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_MASKS, None, &view_state).expect("render")).unwrap();
        assert!(masks_json.contains("Masken"));
        assert!(masks_json.contains("Keine Masken"));
        let catalogue_json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
        assert!(catalogue_json.contains("Ebenenarten"));
    }

    #[test]
    fn composite_scene_syncs_document_and_assets() {
        let mut app = semio_app();
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"componentKind\":\"paint-2d\""));
        assert!(json.contains("\"viewMode\":\"composite\""));
        assert!(!json.contains("\"assetsJson\":\"{}\""), "semio fixture has embedded assets");
        let document = semio_example_document();
        let sync_json = document_sync_json(&document);
        assert!(!sync_json.contains("\"assets\""), "sync json must omit assets");
        assert!(!sync_json.contains("\"camera\""), "sync json must omit camera");
        assert!(sync_json.contains("\"params\""), "adjustment params must survive document→sync roundtrip for the paint host");
        let sync_value: Value = serde_json::from_str(&sync_json).expect("sync json");
        let layers = sync_value.get("layers").and_then(Value::as_array).expect("layers");
        assert!(layers.iter().any(|layer| layer.get("kind").and_then(Value::as_str) == Some("adjustment") && layer.get("params").is_some()));
        assert!(document.assets.contains_key("semio-emblem"));
    }

    #[test]
    fn semio_example_preserves_adjustment_params() {
        let document = semio_example_document();
        let RasterLayerNode::Adjustment { params, adjustment_kind, .. } = document
            .layers
            .iter()
            .find(|layer| matches!(layer, RasterLayerNode::Adjustment { id, .. } if id == "brighten"))
            .expect("brighten adjustment")
        else {
            panic!("expected adjustment");
        };
        assert_eq!(adjustment_kind, "brightnessContrast");
        assert!(params.contains_key("brightness"), "fixture brightness must roundtrip");
        assert!(params.contains_key("contrast"), "fixture contrast must roundtrip");
    }

    #[test]
    fn set_hover_highlights_layer_row_via_runtime() {
        let mut app = semio_app();
        let layer_id = layer_node_id(&app.projection().expect("projection").layers[0]).to_string();
        let row_id = layer_row_id(find_layer(&app.projection().expect("projection").layers, &layer_id).expect("layer"));
        let result = app.handle_action("setHover", Some(&json!({ "id": layer_id })), &ViewState::default(), &testkit::meta("local")).expect("hover");
        assert!(result.operations.is_empty(), "hover is a view action and emits no operations");
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains(&format!("\"id\":\"{row_id}\"")), "hovered layer row must be present");
        assert!(json.contains("\"state\":\"previewed\""), "hover stamps UiState::Previewed onto the layer row");
    }

    #[test]
    fn set_composite_viewport_feeds_navigator_scene() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        app.handle_action("setCompositeViewport", Some(&json!({ "width": 640.0, "height": 480.0 })), &ViewState::default(), &testkit::meta("local")).expect("viewport");
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_NAVIGATOR, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("compositeViewportJson"));
        assert!(json.contains(r#"\"width\":640.0"#));
        assert!(json.contains(r#"\"height\":480.0"#));
    }

    #[test]
    fn add_layer_action_appends_and_undo_removes() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let before = app.projection().expect("projection").layers.len();
        app.handle_action("addLayer", Some(&json!({ "kind": "group" })), &ViewState::default(), &testkit::meta("local")).expect("add");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.layers.len(), before + 1);
        assert!(matches!(projection.layers.last().unwrap(), RasterLayerNode::Group { .. }));
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").layers.len(), before);
    }

    #[test]
    fn patch_layer_renames_and_toggles_visibility_round_trip() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        let layer_id = layer_node_id(&app.projection().expect("projection").layers[0]).to_string();
        app.handle_action("patchLayer", Some(&json!({ "layerId": layer_id, "field": "name", "value": "Renamed" })), &ViewState::default(), &testkit::meta("local")).expect("rename");
        assert_eq!(layer_name(&app.projection().expect("projection").layers[0]), "Renamed");
        app.handle_action("toggleLayerVisible", Some(&json!({ "layerId": layer_id })), &ViewState::default(), &testkit::meta("local")).expect("toggle");
        assert!(!layer_visible(&app.projection().expect("projection").layers[0]));
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo toggle");
        assert!(layer_visible(&app.projection().expect("projection").layers[0]));
    }

    #[test]
    fn move_layer_into_group() {
        let mut app = testkit::new_app::<RasterPlayApp>();
        app.handle_action("addLayer", Some(&json!({ "kind": "group" })), &ViewState::default(), &testkit::meta("local")).expect("add group");
        let (group_id, pixel_id) = {
            let projection = app.projection().expect("projection");
            let group = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Group { .. })).unwrap();
            let pixel = projection.layers.iter().find(|layer| matches!(layer, RasterLayerNode::Pixel { .. })).unwrap();
            (layer_node_id(group).to_string(), layer_node_id(pixel).to_string())
        };
        let target_row = format!("{RASTER_TREE_PREFIX}.group.{group_id}");
        app.handle_action("moveLayer", Some(&json!({ "layerId": pixel_id, "targetRowId": target_row })), &ViewState::default(), &testkit::meta("local")).expect("move");
        let projection = app.projection().expect("projection");
        let RasterLayerNode::Group { children, .. } = projection.layers.iter().find(|layer| layer_node_id(layer) == group_id).unwrap() else {
            panic!("expected group");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(layer_node_id(&children[0]), pixel_id);
    }

    /// 🧪 The definitional merge proof: A adds a layer while B renames the background layer — disjoint
    /// tree edits on one backbone that must both survive on both instances.
    #[test]
    fn two_instances_converge_disjoint_layer_edits_via_backbone() {
        let mut instance_a = testkit::new_app::<RasterPlayApp>();
        let mut instance_b = testkit::new_app::<RasterPlayApp>();
        // Seed both from an identical base projection (a background layer with a fixed id) so B's
        // rename targets the same layer A holds — per-instance `initial_projection` mints fresh ids.
        let mut base = empty_raster_projection();
        base.layers = vec![RasterLayerNode::Pixel {
            id: "bg".into(),
            name: "Background".into(),
            visible: true,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: RasterTransform::default(),
            mask: None,
            width: Some(512),
            height: Some(512),
            image_key: None,
        }];
        let base_envelope = serde_json::to_string(&vcs::create_document_vcs_envelope::<RasterDocument, RasterOperation>(
            RASTER_DOCUMENT_SCHEMA,
            "raster",
            base,
            None,
        ))
        .unwrap();
        instance_a.load_document(&base_envelope).expect("load a");
        instance_b.load_document(&base_envelope).expect("load b");
        let background_id = "bg".to_string();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://raster-convergence", "mem://raster-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.handle_action("addLayer", Some(&json!({ "kind": "pixel" })), &ViewState::default(), &testkit::meta("actor-a")).expect("a adds layer");
        instance_b.handle_action("patchLayer", Some(&json!({ "layerId": background_id, "field": "name", "value": "Renamed By B" })), &ViewState::default(), &testkit::meta("actor-b")).expect("b renames");

        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &testkit::meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &testkit::meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert_eq!(projection_a.layers.len(), 2, "A keeps its added layer");
        assert_eq!(projection_b.layers.len(), 2, "B converges on A's added layer");
        assert_eq!(layer_name(&projection_a.layers[0]), "Renamed By B", "A converges on B's rename");
        assert_eq!(layer_name(&projection_b.layers[0]), "Renamed By B", "B keeps its rename");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<RasterPlayApp, usize>(
            "addLayer",
            Some(&json!({ "kind": "pixel" })),
            |app| app.projection().unwrap().layers.len(),
        );
    }

    #[test]
    fn set_active_utility_switch_emits_no_ops_and_reads_from_view_state() {
        let mut app = testkit::new_app_with_registry::<RasterPlayApp>(create_raster_app);
        let before = app.projection().expect("projection");
        let view = ViewState { active_utility_id: Some("paintBrush".into()), ..ViewState::default() };
        // Switching utilities is the framework View action: no document operations, nothing to sync/undo.
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "paintBrush" })), &view, &testkit::meta("local"))
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().expect("projection"), before, "utility switching does not mutate the document");
        // The composite scene reads the host-owned active utility from session view state, not the runtime.
        let json = serde_json::to_string(&app.render(RASTER_PLAY_BODY_COMPOSITE, None, &view).expect("render")).unwrap();
        assert!(json.contains("\"activeUtility\":\"paintBrush\""), "scene reflects host-owned active utility: {json}");
    }

    #[test]
    fn utility_registry_declares_utilities_scoped_to_the_composite_window() {
        let definition = create_raster_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["selectMarquee", "paintBrush", "paintEraser"]);
        // The marquee carries the Selection category; the paint utilities are Tools.
        let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
        assert_eq!(selects, ["selectMarquee"]);
        let composite = definition.window_kinds.iter().find(|window| window.id == RASTER_PLAY_WINDOW_COMPOSITE).expect("composite window");
        assert_eq!(composite.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite window kind");
        // The framework auto-injects the setActiveUtility View action once utilities are declared; no doc operation survives.
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, ActionKind::View)));
        assert!(!definition.actions.iter().any(|action| action.id == "setActiveUtility" && !matches!(action.kind, ActionKind::View)));
    }

    //#region 🔖DslAndOpText
    fn representative_raster_document() -> RasterDocument {
        let mut assets = HashMap::new();
        assets.insert(
            "asset-1".into(),
            RasterImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into() },
        );
        let mut params = serde_json::Map::new();
        params.insert("brightness".into(), serde_json::json!(0.06));
        params.insert("label".into(), serde_json::json!("Warm \"Curve\""));
        params.insert("enabled".into(), serde_json::json!(true));
        params.insert("fallback".into(), serde_json::Value::Null);
        params.insert("curves".into(), serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]]));
        params.insert("nested".into(), serde_json::json!({ "inner": 1.5 }));
        RasterDocument {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            camera: RasterCamera { x: 12.5, y: -4.0, zoom: 1.5 },
            assets,
            layers: vec![
                RasterLayerNode::Pixel {
                    id: "pixel-1".into(),
                    name: "Pixel One".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                    width: Some(256),
                    height: Some(256),
                    image_key: Some("asset-1".into()),
                },
                RasterLayerNode::Group {
                    id: "group-1".into(),
                    name: "Group / Nested".into(),
                    visible: false,
                    opacity: 0.5,
                    blend_mode: "screen".into(),
                    transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                    mask: None,
                    children: vec![
                        RasterLayerNode::Pixel {
                            id: "pixel-2".into(),
                            name: "Child Pixel".into(),
                            visible: true,
                            opacity: 0.75,
                            blend_mode: "multiply".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            width: None,
                            height: None,
                            image_key: None,
                        },
                        RasterLayerNode::Group {
                            id: "group-2".into(),
                            name: "Nested Group".into(),
                            visible: true,
                            opacity: 1.0,
                            blend_mode: "normal".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            children: Vec::new(),
                        },
                    ],
                },
                RasterLayerNode::Adjustment {
                    id: "adjust-1".into(),
                    name: "Curves & Co".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    adjustment_kind: "curves".into(),
                    params,
                },
            ],
        }
    }

    #[test]
    fn raster_dsl_round_trips_representative_document() {
        vcs::test_support::assert_dsl_round_trip(&representative_raster_document());
    }

    #[test]
    fn raster_dsl_round_trips_semio_example_document() {
        vcs::test_support::assert_dsl_round_trip(&semio_example_document());
    }

    #[test]
    fn raster_op_text_round_trips_every_variant() {
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::AddLayer {
            parent_id: None,
            index: 0,
            layer: RasterLayerNode::Pixel {
                id: "l1".into(),
                name: "Base".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(512),
                height: Some(512),
                image_key: None,
            },
        });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::AddLayer {
            parent_id: Some("group-1".into()),
            index: 3,
            layer: RasterLayerNode::Group {
                id: "g2".into(),
                name: "Nested".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: true, invert: false, width: Some(10), height: Some(20) }),
                children: vec![],
            },
        });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::RemoveLayer { layer_id: "l1".into() });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::PatchLayer {
            layer_id: "l1".into(),
            patch: RasterLayerPatch { name: Some("Renamed".into()), visible: Some(false), ..Default::default() },
        });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::PatchLayer {
            layer_id: "adjust-1".into(),
            patch: RasterLayerPatch::default(),
        });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::MoveLayer { layer_id: "l1".into(), parent_id: Some("g2".into()), index: 1 });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::MoveLayer { layer_id: "l1".into(), parent_id: None, index: 0 });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::SetCamera { camera: RasterCamera { x: 1.0, y: -2.5, zoom: 2.0 } });
        vcs::test_support::assert_op_line_round_trip(&RasterOperation::ReplaceDocument { document: representative_raster_document() });
    }

    #[test]
    fn raster_document_text_round_trips_store_with_applied_operation() {
        let envelope = vcs::create_document_vcs_envelope::<RasterDocument, RasterOperation>(RASTER_DOCUMENT_SCHEMA, "doc-text-test", empty_raster_document(), None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        store
            .dispatch(vcs::DocumentVcsCommand::Apply {
                operations: vec![RasterOperation::AddLayer {
                    parent_id: None,
                    index: 1,
                    layer: RasterLayerNode::Adjustment {
                        id: "adjust-text".into(),
                        name: "Levels".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        adjustment_kind: "levels".into(),
                        params: serde_json::Map::new(),
                    },
                }],
                description: None,
            })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
