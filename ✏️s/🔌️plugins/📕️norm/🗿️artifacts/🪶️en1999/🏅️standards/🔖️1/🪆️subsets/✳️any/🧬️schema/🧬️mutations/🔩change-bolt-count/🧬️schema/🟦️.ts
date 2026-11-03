/** 🔩 `change-bolt-count` wire twin: the leaf payload `ChangeBoltCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBoltCount {
  mutation: "changeBoltCount";
  connectionId: string;
  newRows: number;
  newBoltsPerRow: number;
}

export const parseChangeBoltCount: NormWireReader<ChangeBoltCount> = normWireObject<ChangeBoltCount>({ mutation: normWireRequired(normWireLiteral("changeBoltCount")), connectionId: normWireRequired(normWireString), newRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), newBoltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
