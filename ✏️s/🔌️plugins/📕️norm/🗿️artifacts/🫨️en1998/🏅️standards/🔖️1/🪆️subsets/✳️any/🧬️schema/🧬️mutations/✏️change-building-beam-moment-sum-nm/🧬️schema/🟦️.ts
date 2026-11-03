/** ✏️ `change-building-beam-moment-sum-nm` wire twin: the leaf payload `ChangeBuildingBeamMomentSumNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingBeamMomentSumNm {
  mutation: "changeBuildingBeamMomentSumNm";
  id: string;
  newBeamMomentSumNm: number;
}

export const parseChangeBuildingBeamMomentSumNm: NormWireReader<ChangeBuildingBeamMomentSumNm> = normWireObject<ChangeBuildingBeamMomentSumNm>({ mutation: normWireRequired(normWireLiteral("changeBuildingBeamMomentSumNm")), id: normWireRequired(normWireString), newBeamMomentSumNm: normWireRequired(normWireNumber) });
