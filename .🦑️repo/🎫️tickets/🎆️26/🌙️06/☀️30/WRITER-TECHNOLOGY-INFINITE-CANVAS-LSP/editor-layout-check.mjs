/** @emoji 🧪️ Top-left editor viewport smoke. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const wasmBytes = readFileSync(join(process.cwd(), "writer/rs/pkg/writer_bg.wasm"));
const wasmModule = await WebAssembly.compile(wasmBytes);
const { WriterSession, initSync } = await import(join(process.cwd(), "writer/rs/pkg/writer.js"));
initSync({ module: wasmModule });

const session = new WriterSession();
session.setText("MATCH (a:Piece)");
session.setSize(480, 320, 1);
session.setEditorSettingsJson(JSON.stringify({ fontPx: 14, lineHeight: 22, showLineNumbers: true, tabSize: 2 }));
session.setCamera(0, 0, 1);

const caretWorld = JSON.parse(session.caretWorldJson());
const caretScreen = JSON.parse(session.worldToScreenJson(caretWorld.x, caretWorld.y));
if (caretScreen.x > 120 || caretScreen.y > 40) {
  throw new Error(`expected caret near top-left, got ${JSON.stringify(caretScreen)}`);
}
if (caretScreen.x < 50) throw new Error(`expected caret after gutter, got ${JSON.stringify(caretScreen)}`);

console.log("[DEBUG] editor-layout-check ok", caretScreen);
