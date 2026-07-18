//! 🌐 Headless static-site compiler for animate present decks.

use crate::PresentDeck;
use serde_json::json;
use std::fs;
use std::path::Path;

/// 🚨 Static-site compilation failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresentCompileError {
    pub message: String,
}

impl PresentCompileError {
    fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl std::fmt::Display for PresentCompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PresentCompileError {}

pub type Result<T> = std::result::Result<T, PresentCompileError>;

/// 📦 Writes `index.html`, `styles.css`, `manifest.json`, and embedded deck JSON for a wgpu-ready site.
pub fn compile_present_site(deck: &PresentDeck, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).map_err(|error| PresentCompileError::new(error.to_string()))?;
    let deck_json = serde_json::to_string_pretty(deck)
        .map_err(|error| PresentCompileError::new(format!("deck json: {error}")))?;
    fs::write(output_dir.join("deck.json"), &deck_json)
        .map_err(|error| PresentCompileError::new(error.to_string()))?;
    fs::write(output_dir.join("index.html"), index_html(&deck_json))
        .map_err(|error| PresentCompileError::new(error.to_string()))?;
    fs::write(output_dir.join("styles.css"), styles_css())
        .map_err(|error| PresentCompileError::new(error.to_string()))?;
    fs::write(
        output_dir.join("manifest.json"),
        serde_json::to_string_pretty(&site_manifest(deck)).map_err(|error| PresentCompileError::new(error.to_string()))?,
    )
    .map_err(|error| PresentCompileError::new(error.to_string()))?;
    fs::write(output_dir.join("player.js"), player_stub_js())
        .map_err(|error| PresentCompileError::new(error.to_string()))?;
    Ok(())
}

fn site_manifest(deck: &PresentDeck) -> serde_json::Value {
    json!({
        "schema": "animate.present.site",
        "deckSchema": deck.schema,
        "title": deck.tiles.first().map(|tile| tile.name.as_str()).unwrap_or("Animate Present"),
        "tileCount": deck.tiles.len(),
        "player": {
            "kind": "wgpu",
            "wasm": "/animate/plugin/wasm/animate_plugin_bg.wasm",
            "js": "/animate/plugin/wasm/animate_plugin.js",
            "boot": "/animate/plugin/wasm/boot.js"
        },
        "assets": {
            "deck": "deck.json",
            "styles": "styles.css",
            "player": "player.js"
        }
    })
}

fn index_html(deck_json: &str) -> String {
    let escaped = deck_json.replace('&', "&amp;").replace('<', "&lt;");
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Animate Present</title>
  <link rel="stylesheet" href="styles.css" />
  <link rel="manifest" href="manifest.json" />
</head>
<body>
  <main id="animate-present-root" data-deck-schema="animate.present.deck">
    <canvas id="animate-present-canvas" width="1280" height="720"></canvas>
    <script id="animate-present-deck" type="application/json">{escaped}</script>
  </main>
  <script type="module" src="/animate/plugin/wasm/animate_plugin.js"></script>
  <script type="module" src="player.js"></script>
</body>
</html>
"#
    )
}

fn styles_css() -> &'static str {
    r#"html, body {
  margin: 0;
  height: 100%;
  background: #0b0d12;
  color: #f4f6fb;
  font-family: system-ui, sans-serif;
}

#animate-present-root {
  display: grid;
  place-items: center;
  min-height: 100%;
}

#animate-present-canvas {
  width: min(100vw, 1280px);
  height: auto;
  aspect-ratio: 16 / 9;
  border: 1px solid #2a3140;
  border-radius: 8px;
  background: #11151d;
}
"#
}

fn player_stub_js() -> &'static str {
    r#"const root = document.getElementById("animate-present-root");
const canvas = document.getElementById("animate-present-canvas");
const deckNode = document.getElementById("animate-present-deck");
const deck = deckNode ? JSON.parse(deckNode.textContent || "{}") : {};

async function bootAnimatePresentPlayer() {
  const wasmUrl = "/animate/plugin/wasm/animate_plugin_bg.wasm";
  const init = globalThis.AnimatePluginInit || globalThis.default;
  if (typeof init !== "function") {
  console.warn("[animate-present] wasm player stub waiting for animate plugin", { wasmUrl, deck });
    return;
  }
  await init({ canvas, deck, appId: "animate-present-play" });
}

bootAnimatePresentPlayer().catch((error) => {
  console.error("[animate-present] player boot failed", error);
});
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{default_present_deck, populate_tile_drafts_from_grid, FigureTileGridSeedSpec};

    #[test]
    fn compile_present_site_writes_static_bundle() {
        let deck = default_present_deck();
        let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec {
            source: &deck.source,
            rows: 2,
            columns: 2,
            gap: 0.0,
            key_prefix: "tile",
        });
        let deck = PresentDeck {
            tiles,
            ..deck
        };
        let output = std::env::temp_dir().join(format!("animate-present-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&output);
        compile_present_site(&deck, &output).expect("compile site");
        let index = std::fs::read_to_string(output.join("index.html")).expect("index.html");
        assert!(index.contains("animate.present.deck"));
        assert!(index.contains("animate_plugin.js"));
        let manifest: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(output.join("manifest.json")).expect("manifest")).expect("json");
        assert_eq!(manifest.get("schema").and_then(|v| v.as_str()), Some("animate.present.site"));
        assert_eq!(
            manifest.pointer("/player/wasm").and_then(|v| v.as_str()),
            Some("/animate/plugin/wasm/animate_plugin_bg.wasm")
        );
        let deck_file: PresentDeck =
            serde_json::from_str(&std::fs::read_to_string(output.join("deck.json")).expect("deck.json")).expect("deck");
        assert_eq!(deck_file.tiles.len(), 4);
        let _ = std::fs::remove_dir_all(&output);
    }
}
