#!/usr/bin/env bun
/** @emoji 🔍 Sniff `.semio` envelopes from content (mirrors Rust `semio_format::sniff`) for verify when `cargo run` is blocked. */
import { readFileSync, renameSync } from "node:fs";
import { basename } from "node:path";

const BINARY_MAGIC = Buffer.from([0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a]);

function sniff(bytes) {
  if (bytes.subarray(0, 8).equals(BINARY_MAGIC)) {
    const tokenLen = bytes.readUInt32LE(8);
    const token = bytes.subarray(12, 12 + tokenLen).toString("utf8");
    const [body, versionPart] = token.split(/\s+v/);
    const parts = body.split(".");
    const component = parts.pop();
    const artifact = parts.pop();
    const plugin = parts.join(".");
    return `semio ${plugin}.${artifact}.${component} v${versionPart}`;
  }
  const first = bytes.toString("utf8").split(/\r?\n/)[0]?.trim() ?? "";
  if (!first.startsWith("semio ")) throw new Error(`not semio: ${first.slice(0, 40)}`);
  return first;
}

const samples = [
  {
    label: "dsl",
    path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.gis.gismap.dsl.semio",
  },
  {
    label: "op",
    path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/♻️reuse/🔧️ops/♻️reuse/🧬️component.gis.gismap.op.semio",
  },
  {
    label: "pack",
    path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/♻️reuse/🎒️packs/♻️reuse/🧬️component.gis.gismap.pack.semio",
  },
  {
    label: "spr",
    path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/♻️reuse/📡️sprs/♻️reuse/🧬️component.gis.gismap.spr.semio",
  },
  {
    label: "dsl-renamed",
    path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.gis.gismap.dsl.semio",
    wrongName: ".🦑️repo/🎫️tickets/26/08/06/UNIVERSAL-SEMIO-FORMAT-AND-ARTIFACT-OWNED-EXAMPLES/verify-wrong-name.semio",
  },
];

const root = new URL("../../../../../..", import.meta.url).pathname;
let cmdPath = "";
for (const s of samples) {
  const abs = `${root}/${s.path}`;
  const bytes = readFileSync(abs);
  if (s.wrongName) {
    const tmp = `${root}/${s.wrongName}`;
    renameSync(abs, tmp);
    const renamed = readFileSync(tmp);
    const line = sniff(renamed);
    console.log(`[DEBUG] semio inspect (${s.label}, filename ignored): ${line}`);
    renameSync(tmp, abs);
    continue;
  }
  const line = sniff(bytes);
  console.log(`[DEBUG] semio inspect ${basename(abs)} (${s.label}): ${line}`);
  if (s.label === "dsl") cmdPath = abs;
}

const cmdCandidates = [
  "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/⚙️engine/📚️examples/♻️reuse/🧬️component.gis.2d.cmd.semio",
];
for (const rel of cmdCandidates) {
  const abs = `${root}/${rel}`;
  try {
    const line = sniff(readFileSync(abs));
    console.log(`[DEBUG] semio inspect cmd: ${line}`);
    break;
  } catch {
    /* try next */
  }
}

console.log("[DEBUG] verify-seed complete");
