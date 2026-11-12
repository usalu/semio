/** 🩺️ Runs the maintained `validateModel` over every solid of the volume snapshot (one solid per document) and lists errors per asset. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED } from "./volume-lib.ts";
import { validateModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

const assets = JSON.parse(readFileSync(join(GENERATED, "kernel-maintenance", "volumes-snapshot.json"), "utf8")) as Record<string, any>;
const text = { en: "x", de: "x" };
const base = { version: "semio.model/v1", id: "check", title: text, nodes: [{ id: "n", parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] }], meshes: [], skins: [], materials: [{ id: "m", title: text, baseColor: [1, 1, 1, 1], metallic: 0, roughness: 1, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }], variants: [], poses: [], animations: [] };
const codes: Record<string, number> = {};
const lines: string[] = [];
let solids = 0;
for (const [key, asset] of Object.entries(assets)) for (const solid of asset.volume.solids) {
  solids++;
  const { bone, weights, ...rest } = solid;
  const issues = validateModel({ ...base, solids: [{ ...rest, node: "n", material: "m" }] } as never).filter((i) => i.severity === "error");
  for (const i of issues) {
    codes[i.code] = (codes[i.code] ?? 0) + 1;
    if (["self-intersection", "bad-holes"].includes(i.code)) lines.push(`${key} ${solid.id} (${solid.kind}) ${i.code} ${i.path}`);
  }
}
console.log({ solids, codes });
console.log(lines.join("\n"));
