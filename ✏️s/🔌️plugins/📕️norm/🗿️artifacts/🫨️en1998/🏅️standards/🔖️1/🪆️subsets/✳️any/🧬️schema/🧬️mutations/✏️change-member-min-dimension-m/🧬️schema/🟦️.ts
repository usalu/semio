/** ✏️ `change-member-min-dimension-m` wire twin: the leaf payload `ChangeMemberMinDimensionM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMinDimensionM {
  mutation: "changeMemberMinDimensionM";
  buildingId: string;
  id: string;
  newMinDimensionM: number;
}

export const parseChangeMemberMinDimensionM: NormWireReader<ChangeMemberMinDimensionM> = normWireObject<ChangeMemberMinDimensionM>({ mutation: normWireRequired(normWireLiteral("changeMemberMinDimensionM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMinDimensionM: normWireRequired(normWireNumber) });
