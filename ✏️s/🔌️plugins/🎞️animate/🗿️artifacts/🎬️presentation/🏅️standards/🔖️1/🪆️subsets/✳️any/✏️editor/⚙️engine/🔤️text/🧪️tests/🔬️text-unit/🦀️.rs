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
