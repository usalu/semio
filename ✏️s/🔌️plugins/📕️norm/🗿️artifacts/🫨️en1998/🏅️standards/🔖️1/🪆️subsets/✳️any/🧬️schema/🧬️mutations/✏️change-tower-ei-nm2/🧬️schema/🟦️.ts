/** ✏️ `change-tower-ei-nm2` wire twin: the leaf payload `ChangeTowerEiNm2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerEiNm2 {
  mutation: "changeTowerEiNm2";
  id: string;
  newEiNm2: number;
}

export const parseChangeTowerEiNm2: NormWireReader<ChangeTowerEiNm2> = normWireObject<ChangeTowerEiNm2>({ mutation: normWireRequired(normWireLiteral("changeTowerEiNm2")), id: normWireRequired(normWireString), newEiNm2: normWireRequired(normWireNumber) });
