/** 🏗️ `change-pile-group-rows` wire twin: the leaf payload `ChangePileGroupRows`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupRows {
  mutation: "changePileGroupRows";
  id: string;
  newRows: number;
}

export const parseChangePileGroupRows: NormWireReader<ChangePileGroupRows> = normWireObject<ChangePileGroupRows>({ mutation: normWireRequired(normWireLiteral("changePileGroupRows")), id: normWireRequired(normWireString), newRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
