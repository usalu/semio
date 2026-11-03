/** 🏗️ `change-member-dimension-tolerance` wire twin: the leaf payload `ChangeMemberDimensionTolerance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDimensionTolerance {
  mutation: "changeMemberDimensionTolerance";
  id: string;
  newDimensionTolerance: number;
}

export const parseChangeMemberDimensionTolerance: NormWireReader<ChangeMemberDimensionTolerance> = normWireObject<ChangeMemberDimensionTolerance>({ mutation: normWireRequired(normWireLiteral("changeMemberDimensionTolerance")), id: normWireRequired(normWireString), newDimensionTolerance: normWireRequired(normWireNumber) });
