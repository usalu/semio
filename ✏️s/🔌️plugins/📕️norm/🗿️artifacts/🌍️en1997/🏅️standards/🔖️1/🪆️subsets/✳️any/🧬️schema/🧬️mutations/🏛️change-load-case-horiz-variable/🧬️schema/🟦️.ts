/** 🏛️ `change-load-case-horiz-variable` wire twin: the leaf payload `ChangeLoadCaseHorizVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseHorizVariable {
  mutation: "changeLoadCaseHorizVariable";
  footingId: string;
  id: string;
  newHorizontalVariable: number;
}

export const parseChangeLoadCaseHorizVariable: NormWireReader<ChangeLoadCaseHorizVariable> = normWireObject<ChangeLoadCaseHorizVariable>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseHorizVariable")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newHorizontalVariable: normWireRequired(normWireNumber) });
