/** 🏗️ `change-member-mat-partial-factor` wire twin: the leaf payload `ChangeMemberMatPartialFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMatPartialFactor {
  mutation: "changeMemberMatPartialFactor";
  id: string;
  newMaterialPartialFactor: number;
}

export const parseChangeMemberMatPartialFactor: NormWireReader<ChangeMemberMatPartialFactor> = normWireObject<ChangeMemberMatPartialFactor>({ mutation: normWireRequired(normWireLiteral("changeMemberMatPartialFactor")), id: normWireRequired(normWireString), newMaterialPartialFactor: normWireRequired(normWireNumber) });
