/** 🏗️ `change-member-material-strength-k` wire twin: the leaf payload `ChangeMemberMaterialStrengthK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMaterialStrengthK {
  mutation: "changeMemberMaterialStrengthK";
  id: string;
  newMaterialStrengthK: number;
}

export const parseChangeMemberMaterialStrengthK: NormWireReader<ChangeMemberMaterialStrengthK> = normWireObject<ChangeMemberMaterialStrengthK>({ mutation: normWireRequired(normWireLiteral("changeMemberMaterialStrengthK")), id: normWireRequired(normWireString), newMaterialStrengthK: normWireRequired(normWireNumber) });
