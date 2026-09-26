# Catalogue reference tables (app-surface)

## Signature

```rust
pub fn render_catalogue(
    examples: &[ExampleSource],
    tables: &[CatalogueTable],
    locale: Locale,
    controller_id: &'static str,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode>;
```

See `📓️impl-b2-app-surface.md` for the full `CatalogueTable` model and family call example.

## Wired families

| Family | Tables |
|--------|--------|
| en1992 | Table 3.1 concrete + B500 steel |
| en1999 | Table 3.2 alloys |
| vdi3805 | PN / fuel / connection code lists |
| en1990 | EN + DE NA ψ + γ_I |
| din16798 | SFP, DR, B.6/B.7, annex CO₂ |
| others | `reference_tables() -> vec![]` for now |
