/** 🧱️ `change-element-weld-process` wire twin: the leaf payload `ChangeElementWeldProcess`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldProcess, parseWeldProcess } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeElementWeldProcess {
  mutation: "changeElementWeldProcess";
  sectionId: string;
  elementId: string;
  newWeldProcess: WeldProcess;
}

export const parseChangeElementWeldProcess: NormWireReader<ChangeElementWeldProcess> = normWireObject<ChangeElementWeldProcess>({ mutation: normWireRequired(normWireLiteral("changeElementWeldProcess")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newWeldProcess: normWireRequired(parseWeldProcess) });
