/** 🏗️ `change-member-material-conv-factor` wire twin: the leaf payload `ChangeMemberMaterialConvFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMaterialConvFactor {
  mutation: "changeMemberMaterialConvFactor";
  id: string;
  newMaterialConversionFactor: number;
}

export const parseChangeMemberMaterialConvFactor: NormWireReader<ChangeMemberMaterialConvFactor> = normWireObject<ChangeMemberMaterialConvFactor>({ mutation: normWireRequired(normWireLiteral("changeMemberMaterialConvFactor")), id: normWireRequired(normWireString), newMaterialConversionFactor: normWireRequired(normWireNumber) });
