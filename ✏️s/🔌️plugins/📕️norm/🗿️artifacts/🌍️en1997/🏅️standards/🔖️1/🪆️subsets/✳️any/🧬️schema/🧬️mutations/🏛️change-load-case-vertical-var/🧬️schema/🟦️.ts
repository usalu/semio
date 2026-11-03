/** 🏛️ `change-load-case-vertical-var` wire twin: the leaf payload `ChangeLoadCaseVerticalVar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseVerticalVar {
  mutation: "changeLoadCaseVerticalVar";
  footingId: string;
  id: string;
  newVerticalVariable: number;
}

export const parseChangeLoadCaseVerticalVar: NormWireReader<ChangeLoadCaseVerticalVar> = normWireObject<ChangeLoadCaseVerticalVar>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseVerticalVar")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newVerticalVariable: normWireRequired(normWireNumber) });
