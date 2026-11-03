/** ✏️ `change-tank-filling-ratio` wire twin: the leaf payload `ChangeTankFillingRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTankFillingRatio {
  mutation: "changeTankFillingRatio";
  id: string;
  newFillingRatio: number;
}

export const parseChangeTankFillingRatio: NormWireReader<ChangeTankFillingRatio> = normWireObject<ChangeTankFillingRatio>({ mutation: normWireRequired(normWireLiteral("changeTankFillingRatio")), id: normWireRequired(normWireString), newFillingRatio: normWireRequired(normWireNumber) });
