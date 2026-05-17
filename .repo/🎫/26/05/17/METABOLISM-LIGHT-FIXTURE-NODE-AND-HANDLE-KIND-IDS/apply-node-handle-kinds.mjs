/**
 * @emoji 🧩 One-shot: inject `nodeKind` / `handleKind` into `semio/assets/fixtures/metabolism.kit.light.semio.json` (wip.initialKit).
 * Run from repo root: `bun .repo/🎫/26/05/17/METABOLISM-LIGHT-FIXTURE-NODE-AND-HANDLE-KIND-IDS/apply-node-handle-kinds.mjs`
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "..", "..", "..", "..", "..", "..");
const fixtureRel = path.join("semio", "assets", "fixtures", "metabolism.kit.light.semio.json");
const fixturePath = path.join(repoRoot, fixtureRel);

const NODE_PREFIX = "semio.metabolism.light.node.";
const HANDLE_PREFIX = "semio.metabolism.light.handle.";

function nodeKindForTypeId(id) {
  return `${NODE_PREFIX}${id}`;
}
function handleKindForPortId(id) {
  return `${HANDLE_PREFIX}${id}`;
}

const raw = fs.readFileSync(fixturePath, "utf8");
const j = JSON.parse(raw);
const kit = j.wip?.initialKit;
if (!kit?.types?.items) throw new Error("missing wip.initialKit.types.items");

let typeCount = 0;
for (const t of kit.types.items) {
  if (!t.id) continue;
  t.nodeKind = nodeKindForTypeId(t.id);
  typeCount++;
  for (const c of t.connectors?.items ?? []) {
    const p = c.port;
    if (p && typeof p === "object" && p.id) {
      p.handleKind = handleKindForPortId(p.id);
    }
  }
}

let portCount = 0;
for (const fam of kit.families?.items ?? []) {
  for (const p of fam.ports?.items ?? []) {
    if (!p.id) continue;
    p.handleKind = handleKindForPortId(p.id);
    portCount++;
  }
}

fs.writeFileSync(fixturePath, `${JSON.stringify(j, null, 4)}\n`, "utf8");
console.log(`[apply-node-handle-kinds] wrote ${fixtureRel}: types ${typeCount}, family ports ${portCount}`);
