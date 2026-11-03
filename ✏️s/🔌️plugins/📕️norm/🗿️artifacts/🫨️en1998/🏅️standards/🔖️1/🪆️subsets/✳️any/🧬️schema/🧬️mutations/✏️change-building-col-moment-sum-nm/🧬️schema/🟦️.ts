/** ✏️ `change-building-col-moment-sum-nm` wire twin: the leaf payload `ChangeBuildingColMomentSumNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingColMomentSumNm {
  mutation: "changeBuildingColMomentSumNm";
  id: string;
  newColumnMomentSumNm: number;
}

export const parseChangeBuildingColMomentSumNm: NormWireReader<ChangeBuildingColMomentSumNm> = normWireObject<ChangeBuildingColMomentSumNm>({ mutation: normWireRequired(normWireLiteral("changeBuildingColMomentSumNm")), id: normWireRequired(normWireString), newColumnMomentSumNm: normWireRequired(normWireNumber) });
