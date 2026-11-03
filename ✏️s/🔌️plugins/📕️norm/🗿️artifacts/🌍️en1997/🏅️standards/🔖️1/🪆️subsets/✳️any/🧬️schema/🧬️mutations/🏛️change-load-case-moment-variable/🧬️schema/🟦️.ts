/** 🏛️ `change-load-case-moment-variable` wire twin: the leaf payload `ChangeLoadCaseMomentVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseMomentVariable {
  mutation: "changeLoadCaseMomentVariable";
  footingId: string;
  id: string;
  newMomentVariable: number;
}

export const parseChangeLoadCaseMomentVariable: NormWireReader<ChangeLoadCaseMomentVariable> = normWireObject<ChangeLoadCaseMomentVariable>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseMomentVariable")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMomentVariable: normWireRequired(normWireNumber) });
