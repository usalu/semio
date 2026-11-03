/** 🧵️ `change-weld-run-start` wire twin: the leaf payload `ChangeWeldRunStart`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunStart {
  mutation: "changeWeldRunStart";
  weldRunId: string;
  newStart: number;
}

export const parseChangeWeldRunStart: NormWireReader<ChangeWeldRunStart> = normWireObject<ChangeWeldRunStart>({ mutation: normWireRequired(normWireLiteral("changeWeldRunStart")), weldRunId: normWireRequired(normWireString), newStart: normWireRequired(normWireNumber) });
