# 📸 Remodel plugin end to end — status

## 2026-09-16

- Ticket opened (manual on disk; repo MCP down).
- Native `cargo check -p semio-s-plugin-remodel --lib --tests`: passes clean (6m57s under load ~100, `🗑️generated/check-remodel-native.txt`; only peer `unused_qualifications` warnings in `📡️replication`).
- Unlike shooting/forms, remodel already carries the bounded-tool migration (35 verbs in `REMODELING_RETAINED_TOOL_IDS`, exact publication contracts, one-item store preparation factory) and an `args_bridge::command_from_action`, so the expected gaps are lane wiring + runtime faults.
- Fault 1 (same as forms #1/shooting #1): playground block had no `app` → `VITE_SEMIO_APP_ID=""`. Added `app = "s.remodel.remodeling@1/*#editor"` to the remodel `Cargo.toml` playground block.
- Scripts: `📜️check-remodel-native.sh`, `📜️activate-remodel-react.sh` (`activate-remodel-react-dev`, port 6063), `📜️serve-remodel-react.sh` (`?plugin=remodel`), probes `🐍️remodel-console-dump-probe.mjs` (boot) + `🐍️remodel-interact-probe.mjs` (Actions `addGcp` → undo → mesh layer toggle → Capture mode). Launch entry `remodel-react-attach` added.
- Restage started 22:01Z (`screen remodel-activate`); the staged `🔌️plugin-modules/📸️remodel` wasm dated Aug 17 (107 MB dev build).
