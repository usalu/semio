#!/usr/bin/env bun
/**
 * 🌱 Seed / document intended lowpoly example payloads for the P4 pilot.
 *
 * Current state:
 * - DSL reuse example is handcrafted structured half-edge text (no mesh-json).
 * - pack/spr reuse examples are SEM-enveloped placeholders (>64 bytes) carrying
 *   LWPL domain framing stubs (Objects / PaintLayers / Projection) and spr
 *   record-tag stubs matching LowpolyOperation variants.
 *
 * Intended next step (once handcrafted Rust encoders exist):
 *   1. parse the DSL example with the handcrafted codec
 *   2. encode_pack -> overwrite *.pack.semio
 *   3. encode each Operation variant -> overwrite *.spr.semio (or a suite)
 *
 * Magic: framing 0x89 'L' 'W' 'P' 'L' 0x0D 0x0A 0x1A = 0x894C57504C0D0A1A
 *
 * Operation record tags (spr):
 *   1 ObjectsAdd
 *   2 ObjectsRemove
 *   3 ObjectsMove
 *   4 ObjectsPatch
 *   5 AddPaintLayer
 *   6 RemovePaintLayer
 *   7 PatchPaintLayer
 *   8 PaintStroke
 *   9 SetProjection
 *
 * Pack segments:
 *   1 Objects
 *   2 PaintLayers
 *   3 Projection
 *
 * Usage (later):
 *   bun ./.🦑️repo/🎫️tickets/.../seed-lowpoly-examples.mjs
 *
 * This script currently only verifies placeholder sizes and documents the contract.
 */
import { readFileSync, existsSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
// ticket is .🦑️repo/🎫️tickets/YY/MM/DD/SLUG — walk up to repo root
const repoRoot = resolve(here, "../../../../../..");

const artifact = join(
  repoRoot,
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/♻️reuse",
);

const files = {
  dsl: join(artifact, "🗣️dsls/♻️reuse/🧬️component.lowpoly.lowpoly.dsl.semio"),
  op: join(artifact, "🔧️ops/♻️reuse/🧬️component.lowpoly.lowpoly.op.semio"),
  pack: join(artifact, "🎒️packs/♻️reuse/🧬️component.lowpoly.lowpoly.pack.semio"),
  spr: join(artifact, "📡️sprs/♻️reuse/🧬️component.lowpoly.lowpoly.spr.semio"),
};

for (const [role, path] of Object.entries(files)) {
  if (!existsSync(path)) {
    console.error(`[seed-lowpoly] missing ${role}: ${path}`);
    process.exit(1);
  }
  const bytes = readFileSync(path);
  const text = bytes.toString("utf8");
  if (role === "dsl" || role === "op") {
    if (text.includes("mesh-json")) {
      console.error(`[seed-lowpoly] ${role} still contains mesh-json`);
      process.exit(1);
    }
    console.log(`[seed-lowpoly] ${role}: ${bytes.length} bytes, no mesh-json`);
  } else {
    if (bytes.length <= 64) {
      console.error(`[seed-lowpoly] ${role} too small (${bytes.length}) — empty SEM envelope`);
      process.exit(1);
    }
    console.log(`[seed-lowpoly] ${role}: ${bytes.length} bytes (placeholder until Rust encoder)`);
  }
}

console.log("[seed-lowpoly] ok — replace pack/spr via handcrafted encode_pack/encode_op when ready");
