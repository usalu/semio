//! 🎞️ Animate app engine facet: 🔤️text (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/🔤️text`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod color {
    //! 🎨️ RGBA colors, named palette, and gradient interpolation.

    use serde::{Deserialize, Serialize};

    /// 🌈️ Linear RGBA color with premultiplication left to the renderer.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Color {
        pub r: f64,
        pub g: f64,
        pub b: f64,
        pub a: f64,
    }

    impl Color {
        pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
        pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
        pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
        pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
        pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
        pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
        pub const ORANGE: Self = Self::rgb(1.0, 0.5, 0.0);
        pub const PURPLE: Self = Self::rgb(0.5, 0.0, 0.5);
        pub const TEAL: Self = Self::rgb(0.0, 0.5, 0.5);
        pub const GRAY: Self = Self::rgb(0.5, 0.5, 0.5);
        pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

        pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
            Self { r, g, b, a: 1.0 }
        }

        pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
            Self { r, g, b, a }
        }

        pub fn hex(hex: &str) -> Self {
            let s = hex.trim_start_matches('#');
            let (r, g, b, a) = match s.len() {
                6 => {
                    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                    (r, g, b, 255)
                }
                8 => {
                    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                    let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
                    (r, g, b, a)
                }
                _ => (0, 0, 0, 255),
            };
            Self::rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
        }

        pub fn with_alpha(mut self, alpha: f64) -> Self {
            self.a = alpha;
            self
        }

        pub fn lerp(self, other: Self, t: f64) -> Self {
            let t = t.clamp(0.0, 1.0);
            Self { r: self.r + (other.r - self.r) * t, g: self.g + (other.g - self.g) * t, b: self.b + (other.b - self.b) * t, a: self.a + (other.a - self.a) * t }
        }

        pub fn to_array(self) -> [f64; 4] {
            [self.r, self.g, self.b, self.a]
        }
    }

    /// 🌅️ Multi-stop color gradient.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Gradient {
        pub stops: Vec<(f64, Color)>,
    }

    impl Gradient {
        pub fn new(stops: Vec<(f64, Color)>) -> Self {
            let mut stops = stops;
            stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            Self { stops }
        }

        pub fn sample(&self, t: f64) -> Color {
            let t = t.clamp(0.0, 1.0);
            if self.stops.is_empty() {
                return Color::WHITE;
            }
            if self.stops.len() == 1 {
                return self.stops[0].1;
            }
            if t <= self.stops[0].0 {
                return self.stops[0].1;
            }
            if t >= self.stops[self.stops.len() - 1].0 {
                return self.stops[self.stops.len() - 1].1;
            }
            for pair in self.stops.windows(2) {
                let (t0, c0) = pair[0];
                let (t1, c1) = pair[1];
                if t >= t0 && t <= t1 {
                    let u = if (t1 - t0).abs() < 1e-9 { 0.0 } else { (t - t0) / (t1 - t0) };
                    return c0.lerp(c1, u);
                }
            }
            self.stops[0].1
        }
    }

    pub fn named_color(name: &str) -> Color {
        match name.to_ascii_lowercase().as_str() {
            "white" => Color::WHITE,
            "black" => Color::BLACK,
            "red" => Color::RED,
            "green" => Color::GREEN,
            "blue" => Color::BLUE,
            "yellow" => Color::YELLOW,
            "orange" => Color::ORANGE,
            "purple" => Color::PURPLE,
            "teal" => Color::TEAL,
            "gray" | "grey" => Color::GRAY,
            "manim_blue" | "semio_blue" => Color::hex("#58C4DD"),
            "manim_green" | "semio_green" => Color::hex("#83C167"),
            "manim_red" | "semio_red" => Color::hex("#FC6255"),
            "manim_yellow" | "semio_yellow" => Color::hex("#FFFF00"),
            other => Color::hex(other),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn lerp_midpoint_is_average() {
            let c = Color::BLACK.lerp(Color::WHITE, 0.5);
            assert!((c.r - 0.5).abs() < 1e-9);
        }

        #[test]
        fn gradient_samples_stops() {
            let g = Gradient::new(vec![(0.0, Color::RED), (1.0, Color::BLUE)]);
            let mid = g.sample(0.5);
            assert!(mid.r > 0.0 && mid.b > 0.0);
        }

        #[test]
        fn hex_parses_six_and_eight_digit_forms() {
            let rgb = Color::hex("#ff0000");
            assert!((rgb.r - 1.0).abs() < 1e-9);
            assert!((rgb.a - 1.0).abs() < 1e-9);
            let rgba = Color::hex("00ff0080");
            assert!((rgba.g - 1.0).abs() < 1e-9);
            assert!((rgba.a - 128.0 / 255.0).abs() < 1e-9);
        }

        #[test]
        fn hex_falls_back_to_black_on_invalid_length() {
            let bad = Color::hex("#abc");
            assert_eq!(bad, Color::BLACK);
        }

        #[test]
        fn named_color_covers_aliases_and_hex_fallback() {
            assert_eq!(named_color("WHITE"), Color::WHITE);
            assert_eq!(named_color("grey"), Color::GRAY);
            assert_eq!(named_color("gray"), Color::GRAY);
            assert_eq!(named_color("semio_blue"), Color::hex("#58C4DD"));
            assert_eq!(named_color("manim_blue"), Color::hex("#58C4DD"));
            assert_eq!(named_color("semio_green"), Color::hex("#83C167"));
            assert_eq!(named_color("manim_red"), Color::hex("#FC6255"));
            assert_eq!(named_color("manim_yellow"), Color::hex("#FFFF00"));
            assert_eq!(named_color("ff00ff"), Color::hex("ff00ff"));
        }

        #[test]
        fn gradient_edge_cases() {
            let empty = Gradient::new(vec![]);
            assert_eq!(empty.sample(0.5), Color::WHITE);
            let single = Gradient::new(vec![(0.3, Color::RED)]);
            assert_eq!(single.sample(0.0), Color::RED);
            assert_eq!(single.sample(1.0), Color::RED);
            let g = Gradient::new(vec![(0.2, Color::RED), (0.8, Color::BLUE)]);
            assert_eq!(g.sample(0.0), Color::RED);
            assert_eq!(g.sample(1.0), Color::BLUE);
        }

        #[test]
        fn gradient_new_sorts_unordered_stops() {
            let g = Gradient::new(vec![(1.0, Color::BLUE), (0.0, Color::RED)]);
            assert_eq!(g.stops[0].0, 0.0);
            assert_eq!(g.stops[1].0, 1.0);
        }

        #[test]
        fn with_alpha_and_to_array_roundtrip() {
            let c = Color::rgb(0.2, 0.4, 0.6).with_alpha(0.5);
            assert_eq!(c.to_array(), [0.2, 0.4, 0.6, 0.5]);
        }
    }
}

#[allow(clippy::module_inception)]
pub mod text {
    //! 🔤️ Text and math labels via Typst-to-SVG compilation.

    use crate::editor::animate::engine::scene::sobject::{Sobject, VSobject};
    use crate::editor::animate::engine::text::color::Color;
    use geometry::{append_shape_to_path, BezPath, Rect};
    use semio_framework_typeset::MarkupTypesetter;

    const TEXT_PAGE_PT: f64 = 400.0;
    const TEXT_MARGIN_PT: f64 = 8.0;
    const TEXT_SIZE_PT: f64 = 36.0;

    /// 📝️ Plain text Sobject rendered through Typst.
    #[derive(Clone)]
    pub struct Text {
        pub inner: VSobject,
        pub content: String,
        pub font_size: f64,
    }

    impl Text {
        pub fn new(content: impl Into<String>, color: Color) -> Self {
            let content = content.into();
            let renderer = default_text_renderer();
            let svg = typst_markup_to_validated_svg(&renderer, &wrap_text(&content, TEXT_SIZE_PT));
            let mut inner = svg_to_vobject(&svg, color);
            inner.set_name(content.to_string());
            Self { inner, content, font_size: TEXT_SIZE_PT }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner
        }

        pub fn as_sobject_mut(&mut self) -> &mut VSobject {
            &mut self.inner
        }
    }

    fn format_decimal(value: f64, decimals: u32) -> String {
        format!("{value:.prec$}", prec = decimals as usize)
    }

    /// 🔢️ Decimal number label with interpolatable value.
    #[derive(Clone)]
    pub struct DecimalNumber {
        pub value: f64,
        pub inner: Text,
        pub decimals: u32,
    }

    impl DecimalNumber {
        pub fn new(value: f64, decimals: u32, color: Color) -> Self {
            let inner = Text::new(format_decimal(value, decimals), color);
            Self { value, inner, decimals }
        }

        pub fn lerp_value(&mut self, target: f64, t: f64, color: Color) {
            let t = t.clamp(0.0, 1.0);
            self.value = self.value + (target - self.value) * t;
            self.inner = Text::new(format_decimal(self.value, self.decimals), color);
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// 🔢️ Integer label wrapper.
    #[derive(Clone)]
    pub struct Integer {
        pub value: i64,
        pub inner: Text,
    }

    impl Integer {
        pub fn new(value: i64, color: Color) -> Self {
            Self { value, inner: Text::new(value.to_string(), color) }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// 📄️ Multi-line paragraph wrapper.
    #[derive(Clone)]
    pub struct Paragraph {
        pub lines: Vec<String>,
        pub inner: Text,
    }

    impl Paragraph {
        pub fn new(lines: Vec<impl Into<String>>, color: Color) -> Self {
            let lines: Vec<String> = lines.into_iter().map(Into::into).collect();
            let body = lines.iter().map(|l| l.as_str()).collect::<Vec<_>>().join("\n");
            Self { lines, inner: Text::new(body, color) }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// 💻️ Monospace code block wrapper.
    #[derive(Clone)]
    pub struct Code {
        pub source: String,
        pub inner: Text,
    }

    impl Code {
        pub fn new(source: impl Into<String>, color: Color) -> Self {
            let source = source.into();
            let wrapped = format!("#set page(width: {TEXT_PAGE_PT}pt, height: {TEXT_PAGE_PT}pt, margin: {TEXT_MARGIN_PT}pt, fill: none)\n#set text(size: {TEXT_SIZE_PT}pt, font: \"Courier New\")\n`{source}`");
            let renderer = default_text_renderer();
            let svg = typst_markup_to_validated_svg(&renderer, &wrapped);
            let mut inner_v = svg_to_vobject(&svg, color);
            inner_v.set_name(source.to_string());
            Self { source: source.clone(), inner: Text { inner: inner_v, content: source, font_size: TEXT_SIZE_PT } }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// ∑ Math-mode label rendered through Typst.
    #[derive(Clone)]
    pub struct MathText {
        pub inner: VSobject,
        pub latex: String,
    }

    impl MathText {
        pub fn new(expr: impl Into<String>, color: Color) -> Self {
            let latex = expr.into();
            let wrapped = format!("#set page(width: {}pt, height: {}pt, margin: {}pt, fill: none)\n#set text(size: {}pt)\n$ {latex} $", TEXT_PAGE_PT, TEXT_PAGE_PT, TEXT_MARGIN_PT, TEXT_SIZE_PT);
            let renderer = default_text_renderer();
            let svg = typst_markup_to_validated_svg(&renderer, &wrapped);
            let mut inner = svg_to_vobject(&svg, color);
            inner.set_name(latex.to_string());
            Self { inner, latex }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner
        }
    }

    fn wrap_text(text: &str, size: f64) -> String {
        let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
        format!("#set page(width: {TEXT_PAGE_PT}pt, height: {TEXT_PAGE_PT}pt, margin: {TEXT_MARGIN_PT}pt, fill: none)\n#set align(center + horizon)\n#set text(size: {size}pt)\n\"{escaped}\"")
    }

    fn svg_to_vobject(svg: &str, color: Color) -> VSobject {
        let mut v = VSobject::new();
        if svg.is_empty() {
            v.set_paths(vec![BezPath::new()]);
            v.set_fill(color);
            v.style.stroke = None;
            return v;
        }
        if let Some((_, height)) = semio_framework_typeset::svg_natural_size(svg) {
            let scale = if height > 1e-9 { 2.0 / height } else { 0.01 };
            let offset_y = height * scale;
            let mut paths = semio_framework_typeset::svg_outline_paths(svg, scale, offset_y).unwrap_or_default();
            if paths.is_empty() {
                paths.push(fallback_text_rect());
            }
            v.set_paths(paths);
        } else {
            v.set_paths(vec![fallback_text_rect()]);
        }
        v.set_fill(color);
        v.style.stroke = None;
        v
    }

    fn fallback_text_rect() -> BezPath {
        let rect = Rect::new(-1.0, -0.5, 1.0, 0.5);
        let mut p = BezPath::new();
        append_shape_to_path(&mut p, &rect, 0.01);
        p
    }

    //#region 🔖️TextRenderer
    /// 🖨️ `typst`/`typst-svg`/`typst-assets`/`usvg` now live entirely behind
    /// `semio_framework_typeset` (CLAUDE.md: "external libs behind an interface" / "MUST NOT export
    /// api that ... requires an interface/class/type outside of this codebase") — nothing in this
    /// plugin ever names a `typst::*`/`usvg::*` type directly. Distinct from the FFmpeg deletion in
    /// `⚙️engine/🎥️video` — Typst is a real, working, in-process library call (no subprocess), so
    /// it is isolated, not deleted.
    fn default_text_renderer() -> semio_framework_typeset::TypstTypesetter {
        semio_framework_typeset::default_typesetter()
    }

    /// 🧬️ Renders `markup` through `renderer` (isolated behind `semio_framework_typeset`) and feeds
    /// the resulting SVG string through stdio's real SVG codec (`parse_svg_xml`), producing a real
    /// `SvgSnapshot` — this is the "output feeds a real `SvgSnapshot` encoded via stdio's svg
    /// engine" leg of the isolation. Returns `None` if either the render or the stdio parse fails
    /// (Typst's compiled SVG is expected to already be well-formed; a parse failure here would be
    /// a real bug, not a normal-flow case, so callers fall back the same way a render failure does).
    // 🔀️ R11 "exactly one impl" case: `MarkupTypesetter` has a single implementor (`TypstTypesetter`),
    // so the trait-object parameter is dropped for the concrete type instead of routed through
    // `dyn_enum_close!` (an enum of one variant is worse than none — see 📓️terra-dedyn-fleet-animate-report.md).
    fn render_markup_to_svg_snapshot(renderer: &semio_framework_typeset::TypstTypesetter, markup: &str) -> Option<semio_s_plugin_stdio::artifacts::svg::SvgSnapshot> {
        use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::parse_svg_xml;
        let svg_text = renderer.render_svg(markup)?;
        let doc = parse_svg_xml(&svg_text).ok()?;
        Some(semio_s_plugin_stdio::artifacts::svg::SvgSnapshot { schema: semio_s_plugin_stdio::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
    }

    /// 🖨️ Renders `markup` and returns the SVG text stdio's own real codec re-serialized from the
    /// parsed `SvgSnapshot` — the geometry extraction above (`svg_to_vobject`) keeps consuming a
    /// plain SVG string (via `semio_framework_typeset::svg_outline_paths`, which wraps `usvg`, a
    /// full SVG resolver/rasterizer doing `<use>`/`<defs>`/CSS resolution that stdio's structural
    /// svg codec deliberately does not attempt — a rendering concern, not a duplicated codec), but
    /// that string is now stdio-validated first instead of Typst's raw, unchecked output.
    fn typst_markup_to_validated_svg(renderer: &semio_framework_typeset::TypstTypesetter, markup: &str) -> String {
        use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::write_svg_xml;
        match render_markup_to_svg_snapshot(renderer, markup) {
            Some(snapshot) => write_svg_xml(&snapshot.doc),
            None => String::new(),
        }
    }
    //#endregion 🔖️TextRenderer

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn typst_plain_text_compiles() {
            let svg = default_text_renderer().render_svg(&wrap_text("hello", 24.0));
            assert!(svg.is_some());
            assert!(svg.unwrap().contains("svg"));
        }

        #[test]
        fn math_text_builds_vobject() {
            let m = MathText::new("x^2", Color::WHITE);
            assert!(!m.latex.is_empty());
        }

        #[test]
        fn decimal_number_lerps() {
            let mut d = DecimalNumber::new(0.0, 2, Color::WHITE);
            d.lerp_value(10.0, 0.5, Color::WHITE);
            assert!((d.value - 5.0).abs() < 1e-9);
        }

        #[test]
        fn text_wrappers_build() {
            let i = Integer::new(42, Color::WHITE);
            assert_eq!(i.value, 42);
            let p = Paragraph::new(vec!["line one", "line two"], Color::WHITE);
            assert_eq!(p.lines.len(), 2);
            let c = Code::new("fn main() {}", Color::WHITE);
            assert!(!c.source.is_empty());
        }
    }
}
