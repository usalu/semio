import { MetabolismKit } from "@semio-tech/semio-assets";
import { deserializeKit, serializeKit, areKitsEqual, importKit, Kit } from "../../../compose/js/compose";
import * as fsModule from "node:fs";
import * as pathModule from "node:path";

const fs = fsModule;
const path = pathModule;
const kit = MetabolismKit as unknown as Kit;
const serializedKit = serializeKit(kit);
const deserializedKit = deserializeKit(serializedKit);
console.log("kit === deserializedKit:", areKitsEqual(kit, deserializedKit));

const zipPath = path.join("/workspaces/semio/assets/compose/metabolism.zip");
const zipBuffer = fs.readFileSync(zipPath);
const result = await importKit(zipBuffer.buffer);
const zipKit = result.kit;
console.log("kit.types.len:", kit.types?.length, "zipKit.types.len:", zipKit.types?.length);
console.log("kit.designs.len:", kit.designs?.length, "zipKit.designs.len:", zipKit.designs?.length);

// Check type by type
for (const kt of (kit.types || [])) {
  const zt = (zipKit.types || []).find((t: any) => t.guid === kt.guid);
  if (!zt) { console.log("type missing:", kt.name); continue; }
  if (kt.name !== zt.name) console.log("type name diff:", kt.name, zt.name);
  if (kt.stock !== zt.stock) console.log("type stock diff:", kt.name, kt.stock, zt.stock);
  if ((kt.isAbstract ? true : undefined) !== (zt.isAbstract ? true : undefined)) console.log("type isAbstract diff:", kt.name, kt.isAbstract, zt.isAbstract);
  if ((kt.connectors?.length||0) !== (zt.connectors?.length||0)) { console.log("type connectors diff:", kt.name, kt.connectors?.length, zt.connectors?.length); continue; }
  for (const kc of (kt.connectors || [])) {
    const zc = (zt.connectors || []).find((c: any) => c.guid === kc.guid);
    if (!zc) { console.log("connector missing:", kt.name, kc.guid); continue; }
    if (Math.abs(kc.point.x - zc.point.x) > 0.001 || Math.abs(kc.point.y - zc.point.y) > 0.001 || Math.abs(kc.point.z - zc.point.z) > 0.001) 
      console.log("connector point diff:", kt.name, kc.name, kc.point, zc.point);
    if (Math.abs(kc.t - zc.t) > 0.001) console.log("connector t diff:", kt.name, kc.name, kc.t, zc.t);
    if ((kc.port?.guid||undefined) !== (zc.port?.guid||undefined)) console.log("connector port diff:", kt.name, kc.name, kc.port?.guid, zc.port?.guid);
  }
}

console.log("areKitsEqual:", areKitsEqual(kit, zipKit));
