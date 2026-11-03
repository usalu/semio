/** ✏️ `change-member-material` wire twin: the leaf payload `ChangeMemberMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StructuralMaterial, parseStructuralMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberMaterial {
  mutation: "changeMemberMaterial";
  buildingId: string;
  id: string;
  newMaterial: StructuralMaterial;
}

export const parseChangeMemberMaterial: NormWireReader<ChangeMemberMaterial> = normWireObject<ChangeMemberMaterial>({ mutation: normWireRequired(normWireLiteral("changeMemberMaterial")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMaterial: normWireRequired(parseStructuralMaterial) });
