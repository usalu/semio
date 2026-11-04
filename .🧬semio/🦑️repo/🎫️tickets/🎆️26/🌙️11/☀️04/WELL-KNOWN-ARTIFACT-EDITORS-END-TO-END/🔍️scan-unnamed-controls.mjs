/** 🔍️ Static blast-radius scan of the unnamed-control rule over every JSON UI document in the repository. Usage: node 🔍️scan-unnamed-controls.mjs <repo-root> [--json]. */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const root = process.argv[2];
const SKIP = new Set(["node_modules", "target", ".git", "dist", ".cache"]);
const CONTROLS = new Set(["input", "select", "slider", "numberStepper", "iconSelect", "ring", "toggle"]);
const blank = (text) => text === undefined || text === null || String(text).trim() === "";
const needsName = (component) => CONTROLS.has(component?.type) && (component.type !== "toggle" || blank(component.text));
const unnamed = (node, parent) => needsName(node.component) && blank(node.accessibility?.label) && !(parent?.component?.type === "container" && parent.component.role === "field" && !blank(parent.component.label));

const findings = [];
let documents = 0;
let controls = 0;
function inspectNodes(file, nodes) {
  const records = Array.isArray(nodes) ? nodes : Object.values(nodes);
  const parent = new Map();
  for (const node of records) for (const child of node.children ?? []) parent.set(child, node);
  documents += 1;
  for (const node of records) {
    if (!needsName(node.component)) continue;
    controls += 1;
    if (unnamed(node, parent.get(node.id))) findings.push({ file: relative(root, file), id: node.id, key: node.key, type: node.component.type });
  }
}
function visit(file, value, depth = 0) {
  if (depth > 40 || value === null || typeof value !== "object") return;
  if (!Array.isArray(value) && value.nodes && typeof value.nodes === "object" && (Array.isArray(value.nodes) ? value.nodes : Object.values(value.nodes)).every?.((entry) => entry && typeof entry === "object" && entry.component)) {
    inspectNodes(file, value.nodes);
    return;
  }
  for (const child of Array.isArray(value) ? value : Object.values(value)) visit(file, child, depth + 1);
}
function walk(directory) {
  for (const name of readdirSync(directory)) {
    if (SKIP.has(name)) continue;
    const path = join(directory, name);
    let stat;
    try { stat = statSync(path); } catch { continue; }
    if (stat.isDirectory()) { if (name !== ".🧬semio") walk(path); continue; }
    if (!name.endsWith(".json") || stat.size > 8_000_000) continue;
    let value;
    try { value = JSON.parse(readFileSync(path, "utf8")); } catch { continue; }
    visit(path, value);
  }
}
walk(root);
if (process.argv.includes("--json")) console.log(JSON.stringify({ documents, controls, unnamed: findings.length, findings }, null, 2));
else {
  console.log(`documents with nodes: ${documents}, controls: ${controls}, unnamed: ${findings.length}`);
  for (const finding of findings) console.log(`${finding.file} #${finding.id} ${finding.key} ${finding.type}`);
}
