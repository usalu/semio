mod tests {
    use super::*;
    use crate::default_presentation_snapshot;
    use crate::schema::{FigureTileGridSeedSpec, populate_tile_drafts_from_grid};

    #[semio_framework_async_macros::async_test]
    async fn compile_presentation_site_writes_static_bundle() {
        let deck = default_presentation_snapshot();
        let (source, _) = crate::presentation_working_scene(&deck);
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 2, columns: 2, gap: 0.0, key_prefix: "tile" });
        let deck = crate::presentation_snapshot_with_tiles(&source, &tiles);
        let output = std::env::temp_dir().join(format!("animate-presentation-{}", std::process::id()));
        let _ = fs::remove_dir_all(&output);
        compile_presentation_site(&deck, &output).expect("compile site");
        let index = fs::read_to_string(output.join("🌐️.html")).expect("🌐️.html");
        assert!(index.contains("animate.presentation.deck"));
        assert!(index.contains("semio_s_plugin_animate.js"));
        let player = fs::read_to_string(output.join("player.js")).expect("player.js");
        assert!(player.contains("sceneClips"));
        let manifest = dsl::os_pack::json::parse(&fs::read_to_string(output.join("manifest.json")).expect("manifest")).expect("json");
        assert_eq!(manifest.get("schema").and_then(|v| v.as_str()), Some("animate.presentation.site"));
        assert_eq!(manifest.pointer("/player/wasm").and_then(|v| v.as_str()), Some("/animate/plugin/wasm/animate_plugin_bg.wasm"));
        let deck_value = dsl::os_pack::json::parse(&fs::read_to_string(output.join("deck.json")).expect("deck.json")).expect("json");
        let deck_file: PresentationSnapshot = dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(&deck_value)).expect("deck");
        assert_eq!(crate::presentation_working_scene(&deck_file).1.len(), 4);
        let _ = fs::remove_dir_all(&output);
    }

    /// 🧪️ Native/host-only: `compile_scene_to_assets` renders real frames through
    /// `renderer::VelloRenderer`, which always reports "no adapter" on `wasm32-wasip2` by
    /// design — see `⚙️engine/🎥️video/🦀️.rs`'s `renderer` module.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[semio_framework_async_macros::async_test]
    async fn compile_scene_to_assets_writes_mp4() {
        let output = std::env::temp_dir().join(format!("animate-scene-assets-{}", std::process::id()));
        let _ = fs::remove_dir_all(&output);
        let bundle = compile_scene_to_assets("demo123", &output).await.expect("compile scene");
        assert_eq!(bundle.scene_hash, "demo123");
        assert!(bundle.mp4.as_ref().is_some_and(|path| path.exists()));
        let _ = fs::remove_dir_all(&output);
    }
}
