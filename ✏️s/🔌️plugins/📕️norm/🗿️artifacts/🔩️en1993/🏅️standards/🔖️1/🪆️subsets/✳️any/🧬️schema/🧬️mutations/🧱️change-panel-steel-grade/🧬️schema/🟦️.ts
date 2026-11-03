/** 🧱️ `change-panel-steel-grade` wire twin: the leaf payload `ChangePanelSteelGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelSteelGrade {
  id: string;
  newSteelGrade: string;
}

export const parseChangePanelSteelGrade: NormWireReader<ChangePanelSteelGrade> = normWireObject<ChangePanelSteelGrade>({ id: normWireRequired(normWireString), newSteelGrade: normWireRequired(normWireString) });
