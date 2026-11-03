/** 🧵️ `change-weld-run-process` wire twin: the leaf payload `ChangeWeldRunProcess`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldProcess, parseWeldProcess } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWeldRunProcess {
  mutation: "changeWeldRunProcess";
  weldRunId: string;
  newProcess: WeldProcess;
}

export const parseChangeWeldRunProcess: NormWireReader<ChangeWeldRunProcess> = normWireObject<ChangeWeldRunProcess>({ mutation: normWireRequired(normWireLiteral("changeWeldRunProcess")), weldRunId: normWireRequired(normWireString), newProcess: normWireRequired(parseWeldProcess) });
