/** ✏️ `change-system-material` wire twin: the leaf payload `ChangeSystemMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StructuralMaterial, parseStructuralMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSystemMaterial {
  mutation: "changeSystemMaterial";
  buildingId: string;
  id: string;
  newMaterial: StructuralMaterial;
}

export const parseChangeSystemMaterial: NormWireReader<ChangeSystemMaterial> = normWireObject<ChangeSystemMaterial>({ mutation: normWireRequired(normWireLiteral("changeSystemMaterial")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMaterial: normWireRequired(parseStructuralMaterial) });
