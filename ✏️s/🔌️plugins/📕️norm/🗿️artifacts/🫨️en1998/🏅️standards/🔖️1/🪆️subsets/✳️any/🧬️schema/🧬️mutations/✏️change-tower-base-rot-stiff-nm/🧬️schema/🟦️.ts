/** ✏️ `change-tower-base-rot-stiff-nm` wire twin: the leaf payload `ChangeTowerBaseRotStiffNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerBaseRotStiffNm {
  mutation: "changeTowerBaseRotStiffNm";
  id: string;
  newBaseRotationalStiffnessNm: number;
}

export const parseChangeTowerBaseRotStiffNm: NormWireReader<ChangeTowerBaseRotStiffNm> = normWireObject<ChangeTowerBaseRotStiffNm>({ mutation: normWireRequired(normWireLiteral("changeTowerBaseRotStiffNm")), id: normWireRequired(normWireString), newBaseRotationalStiffnessNm: normWireRequired(normWireNumber) });
