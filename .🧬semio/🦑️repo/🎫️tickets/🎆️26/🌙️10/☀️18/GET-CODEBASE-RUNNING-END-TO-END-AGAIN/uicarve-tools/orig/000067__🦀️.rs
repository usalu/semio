use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser, ClickCounter, CursorEmitter};
use crate::tui::engine::Tui;
use crate::tui::event::Event;
use crate::tui::geometry::Size;
use crate::tui::theme::Theme;
use ui_styling::appearance::AppearanceName;

/// 🏃️ A pure bytes-in/string-out host: feed terminal input, get an ANSI patch back.
pub struct WasmHost {
    pub tui: Tui,
    parser: AnsiParser,
    clicks: ClickCounter,
    cursor: CursorEmitter,
}

impl WasmHost {
    pub fn new(width: u16, height: u16, dark: bool) -> Self {
        let appearance = if dark { AppearanceName::Dark } else { AppearanceName::Light };
        Self { tui: Tui::new(Size { width, height }, Theme::new(appearance)), parser: AnsiParser::new(), clicks: ClickCounter::default(), cursor: CursorEmitter::default() }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Vec<Event> {
        self.feed_at(bytes, 0)
    }

    /// 🖱️ Feeds terminal input observed at `now_ms` (monotonic milliseconds) so double and triple clicks are counted.
    pub fn feed_at(&mut self, bytes: &[u8], now_ms: u64) -> Vec<Event> {
        let mut events = Vec::new();
        self.parser.feed(bytes, &mut events);
        self.clicks.stamp_all(&mut events, now_ms);
        for event in &events {
            self.tui.dispatch(event);
        }
        events
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.tui.dispatch(&Event::Resize(Size { width, height }));
    }

    pub fn render(&mut self) -> String {
        let patch = self.tui.render().0;
        self.cursor.frame(&patch, self.tui.cursor(), false)
    }

    pub fn setup(&self) -> String {
        setup_sequence().to_string()
    }

    pub fn teardown(&self) -> String {
        teardown_sequence().to_string()
    }
}

/// 🌉️ `target_arch = "wasm32"` is true for `wasm32-wasip2` too; this is a browser-only xterm.js bridge around the pure `WasmHost` renderer, so it excludes the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "tui-bindgen"))]
mod bindgen_host {
    use super::WasmHost;
    use wasm_bindgen::prelude::*;

    /// 🌐️ The `wasm-bindgen` surface for browser hosts (e.g. an xterm.js terminal).
    #[wasm_bindgen]
    pub struct TuiHost(WasmHost);

    #[wasm_bindgen]
    impl TuiHost {
        #[wasm_bindgen(constructor)]
        pub fn new(width: u16, height: u16, dark: bool) -> TuiHost {
            TuiHost(WasmHost::new(width, height, dark))
        }

        pub fn feed(&mut self, bytes: &[u8]) {
            self.0.feed(bytes);
        }

        pub fn feed_at(&mut self, bytes: &[u8], now_ms: f64) {
            self.0.feed_at(bytes, now_ms as u64);
        }

        pub fn resize(&mut self, width: u16, height: u16) {
            self.0.resize(width, height);
        }

        pub fn render(&mut self) -> String {
            self.0.render()
        }

        pub fn setup(&self) -> String {
            self.0.setup()
        }

        pub fn teardown(&self) -> String {
            self.0.teardown()
        }
    }
}
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "tui-bindgen"))]
pub use bindgen_host::TuiHost;
