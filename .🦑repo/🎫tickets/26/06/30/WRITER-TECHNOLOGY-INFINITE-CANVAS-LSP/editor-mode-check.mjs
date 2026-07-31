/** @emoji 🧪 WASM smoke: text-editor scroll + settings (no zoom/pan). */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = process.cwd();
const wasmPath = join(root, "writer/rs/pkg/writer_bg.wasm");
const wasmBytes = readFileSync(wasmPath);
const wasmModule = await WebAssembly.compile(wasmBytes);
const { default: init, WriterSession, initSync } = await import(join(root, "writer/rs/pkg/writer.js"));
initSync({ module: wasmModule });

const session = new WriterSession();
session.setText("line one\nline two\nline three\nline four\nline five\nline six\nline seven\nline eight\nline nine\nline ten");
session.setSize(400, 120, 1);

session.setEditorSettingsJson(JSON.stringify({ fontPx: 16, lineHeight: 28, showLineNumbers: true, tabSize: 4 }));
const tab = session.tabInsertText();
if (tab !== "    ") throw new Error(`expected 4-space tab, got ${JSON.stringify(tab)}`);

session.setCamera(99, 0, 2.5);
let camera = JSON.parse(session.cameraJson());
if (camera.x !== 0 || camera.zoom !== 1) throw new Error(`camera must lock x=0 zoom=1, got ${JSON.stringify(camera)}`);

session.wheelScrollScreen(200);
camera = JSON.parse(session.cameraJson());
if (camera.y <= 0) throw new Error("wheel should scroll vertically");
if (camera.x !== 0 || camera.zoom !== 1) throw new Error("wheel must not change x/zoom");

session.setEditorSettingsJson(JSON.stringify({ fontPx: 14, lineHeight: 22, showLineNumbers: false, tabSize: 2 }));
const tab2 = session.tabInsertText();
if (tab2 !== "  ") throw new Error(`expected 2-space tab after settings change`);

console.log("[DEBUG] editor-mode-check ok", { scrollY: camera.y, tab, tab2 });
