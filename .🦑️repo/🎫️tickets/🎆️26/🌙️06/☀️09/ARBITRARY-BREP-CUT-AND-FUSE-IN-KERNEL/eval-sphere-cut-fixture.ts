/** @emoji 🔬️ Verify sphere-cut-with-torus fixture eval completes and returns preview text. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const root = resolve(import.meta.dir, "../../../../../../");
const { default: initFlowWasm, FlowSession } = await import(resolve(root, "flow/core/pkg/flow_core.js"));

const fixturePath = resolve(root, "procedural/fixture/sphere-cut-with-torus.procedural.json");
const fixture = readFileSync(fixturePath, "utf8");
await initFlowWasm();
const session = new FlowSession();
session.loadFixtureJson(fixture);
const started = performance.now();
const outputsJson = await session.evaluate();
const elapsed = performance.now() - started;
const text = session.previewText();
console.log(`[DEBUG] eval elapsed ${elapsed.toFixed(0)}ms preview: ${text}`);
console.log(`[DEBUG] outputs length ${outputsJson.length}`);
