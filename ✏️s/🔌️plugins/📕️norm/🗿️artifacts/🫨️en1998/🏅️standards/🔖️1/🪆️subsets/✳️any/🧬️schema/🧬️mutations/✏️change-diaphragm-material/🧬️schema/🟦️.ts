/** ✏️ `change-diaphragm-material` wire twin: the leaf payload `ChangeDiaphragmMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DiaphragmMaterial, parseDiaphragmMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDiaphragmMaterial {
  mutation: "changeDiaphragmMaterial";
  id: string;
  newMaterial: DiaphragmMaterial;
}

export const parseChangeDiaphragmMaterial: NormWireReader<ChangeDiaphragmMaterial> = normWireObject<ChangeDiaphragmMaterial>({ mutation: normWireRequired(normWireLiteral("changeDiaphragmMaterial")), id: normWireRequired(normWireString), newMaterial: normWireRequired(parseDiaphragmMaterial) });
