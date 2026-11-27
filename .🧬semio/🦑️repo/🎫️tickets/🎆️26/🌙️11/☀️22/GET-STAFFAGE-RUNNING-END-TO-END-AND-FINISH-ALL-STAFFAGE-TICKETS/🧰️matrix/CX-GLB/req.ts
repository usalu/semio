import { NodeIO } from "@gltf-transform/core";
import { readFileSync } from "node:fs";
const b = new Uint8Array(readFileSync(process.argv[2]!));
const io = new NodeIO();
const json = await io.binaryToJSON(b);
for (const mode of ["used", "required"]) {
  const j = structuredClone(json.json) as any; j.extensionsUsed = [...(j.extensionsUsed ?? []), "KHR_mesh_quantization"]; if (mode === "required") j.extensionsRequired = ["KHR_mesh_quantization"];
  try { await io.readJSON({ json: j, resources: json.resources }); console.log(mode, "ok"); } catch (e) { console.log(mode, "throws", (e as Error).message); }
}
