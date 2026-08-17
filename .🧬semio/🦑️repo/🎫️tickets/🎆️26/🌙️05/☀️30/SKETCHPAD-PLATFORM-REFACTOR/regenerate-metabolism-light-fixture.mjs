/**
 * Regenerates `compose/fixtures/metabolism.kit.light.compose.json` from `metabolism.kit.snapshot.compose.json`.
 * Adds bundle hash/items blocks, nodeKind on types, handleKind on ports (families + connector refs).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dir, "../../../../../..");
const snapshotPath = path.join(repoRoot, "compose/fixtures/metabolism.kit.snapshot.compose.json");
const outPath = path.join(repoRoot, "compose/fixtures/metabolism.kit.light.compose.json");
const HASH = "…";
const SCHEMA = "🎆️26🌙️06⬆️1";
const KIT_ID = "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc";

/** @param {unknown} value */
function wrapCollection(value) {
  if (Array.isArray(value)) {
    return { hash: HASH, items: value.map((entry) => annotateValue(entry)) };
  }
  return annotateValue(value);
}

/** @param {unknown} value */
function annotateValue(value) {
  if (value == null || typeof value !== "object") return value;
  if (Array.isArray(value)) return wrapCollection(value);
  const row = /** @type {Record<string, unknown>} */ ({ ...value });
  if (typeof row.id === "string") row.hash = HASH;
  for (const [key, child] of Object.entries(row)) {
    if (Array.isArray(child)) row[key] = wrapCollection(child);
    else if (child != null && typeof child === "object") row[key] = annotateValue(child);
  }
  return row;
}

/** @param {Record<string, unknown>} kit */
function annotateKitSemantics(kit) {
  for (const type of /** @type {Record<string, unknown>[]} */ (kit.types ?? [])) {
    if (typeof type.id === "string") {
      type.nodeKind = `compose.metabolism.light.node.${type.id}`;
    }
    for (const connector of /** @type {Record<string, unknown>[]} */ (type.connectors ?? [])) {
      const port = connector.port;
      if (port != null && typeof port === "object" && !Array.isArray(port)) {
        const portRow = /** @type {Record<string, unknown>} */ (port);
        if (typeof portRow.id === "string") {
          portRow.handleKind = `compose.metabolism.light.handle.${portRow.id}`;
        }
      }
    }
  }
  for (const family of /** @type {Record<string, unknown>[]} */ (kit.families ?? [])) {
    for (const port of /** @type {Record<string, unknown>[]} */ (family.ports ?? [])) {
      if (typeof port.id === "string") {
        port.handleKind = `compose.metabolism.light.handle.${port.id}`;
      }
    }
  }
}

function main() {
  const snapshot = JSON.parse(fs.readFileSync(snapshotPath, "utf8"));
  annotateKitSemantics(/** @type {Record<string, unknown>} */ (snapshot));
  const initialKit = annotateValue(snapshot);
  const bundle = {
    schema: SCHEMA,
    wip: {
      id: KIT_ID,
      hash: HASH,
      authors: { hash: HASH, items: [] },
      initialKit,
    },
  };
  fs.writeFileSync(outPath, `${JSON.stringify(bundle)}\n`);
  const parsed = JSON.parse(fs.readFileSync(outPath, "utf8"));
  const kit = parsed.wip.initialKit;
  const families = kit.families?.items ?? [];
  const nakagin = families.find((f) => f.name === "Nakagin Capsule Tower");
  const portCount = nakagin?.ports?.items?.length ?? 0;
  console.log(`[regenerate-metabolism-light-fixture] wrote ${outPath}`);
  console.log(`[regenerate-metabolism-light-fixture] types=${kit.types?.items?.length ?? 0} designs=${kit.designs?.items?.length ?? 0} nakaginPorts=${portCount}`);
}

main();
