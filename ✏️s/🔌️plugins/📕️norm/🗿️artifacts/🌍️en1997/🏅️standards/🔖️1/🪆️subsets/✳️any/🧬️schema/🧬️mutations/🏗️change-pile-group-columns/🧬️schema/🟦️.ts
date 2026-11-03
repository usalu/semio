/** 🏗️ `change-pile-group-columns` wire twin: the leaf payload `ChangePileGroupColumns`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupColumns {
  mutation: "changePileGroupColumns";
  id: string;
  newColumns: number;
}

export const parseChangePileGroupColumns: NormWireReader<ChangePileGroupColumns> = normWireObject<ChangePileGroupColumns>({ mutation: normWireRequired(normWireLiteral("changePileGroupColumns")), id: normWireRequired(normWireString), newColumns: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
