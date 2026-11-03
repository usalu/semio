/** 🔢 `change-pile-count` wire twin: the leaf payload `ChangePileCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileCount {
  mutation: "changePileCount";
  id: string;
  newCount: number;
}

export const parseChangePileCount: NormWireReader<ChangePileCount> = normWireObject<ChangePileCount>({ mutation: normWireRequired(normWireLiteral("changePileCount")), id: normWireRequired(normWireString), newCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
