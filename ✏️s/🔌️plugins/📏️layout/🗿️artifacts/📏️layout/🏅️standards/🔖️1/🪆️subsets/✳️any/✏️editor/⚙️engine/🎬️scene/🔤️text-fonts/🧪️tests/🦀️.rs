//! 🧪️ Headless sources retain catalog identity through the host asset boundary.
use super::*;
#[test]
fn neutral_catalog_is_available_without_registration() {
    let catalog: serde_json::Value = serde_json::from_str(include_str!("../📦️catalog/🔣️.json")).unwrap();
    let fonts = catalog["fonts"].as_array().unwrap();
    assert_eq!(export_font_count(), fonts.len());
    for (index, record) in fonts.iter().enumerate() {
        let source = export_font_source(index).unwrap();
        assert_eq!(source.len() as u64, record["size"].as_u64().unwrap());
        assert_eq!(&source[..4], &[0, 1, 0, 0]);
        assert!(std::ptr::eq(source, export_font_source(index).unwrap()));
    }
    assert!(export_font_source(fonts.len()).is_none());
    assert!(export_font_source(usize::MAX).is_none());
}
#[test]
fn package_sources_preserve_role_order_without_registration() {
    let fonts = package_text_font_set(|name| TextFontRole::ALL.iter().position(|role| text_font_asset_name(*role) == name).and_then(|index| export_font_source(index + 1))).unwrap();
    for (index, role) in TextFontRole::ALL.iter().enumerate() {
        assert!(std::ptr::eq(export_font_source_from_set(index + 1, &fonts).unwrap(), fonts.get(*role)));
    }
    assert!(std::ptr::eq(export_font_source_from_set(0, &fonts).unwrap(), LAYOUT_SANS));
    assert!(export_font_source_from_set(usize::MAX, &fonts).is_none());
    assert!(matches!(package_text_font_set(|_| None), Err(TextFontsInstallError::PackageAssetMissing { .. })));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn missing_native_assets_can_arrive_later() {
    let files = OnceLock::new();
    assert!(cached_font_files(&files, || None).is_none());
    let directory = native_text_fonts_directory().unwrap();
    let first = cached_font_files(&files, || TextFontFiles::read(&directory).ok()).unwrap();
    let next = cached_font_files(&files, || panic!("successful source must remain cached")).unwrap();
    assert!(std::ptr::eq(first, next));
    for role in TextFontRole::ALL { assert!(std::ptr::eq(first.set().get(role), next.set().get(role))); }
}

#[test]
fn export_catalog_freezes_first_success_across_late_installation() {
    let cache = OnceLock::new();
    assert!(cached_export_font_set(&cache, || None).is_none());
    let original = TextFontSet::try_from_loader::<()>(|_| Ok(LAYOUT_SANS)).unwrap();
    let installed = TextFontSet::try_from_loader::<()>(|_| Ok(&[1, 2, 3, 4])).unwrap();
    let first = cached_export_font_set(&cache, || Some(original)).unwrap();
    let later = cached_export_font_set(&cache, || Some(installed)).unwrap();
    assert!(std::ptr::eq(first, later));
    for role in TextFontRole::ALL { assert!(std::ptr::eq(later.get(role), original.get(role))); }
}
