/** 🧱️ `change-panel-load` wire twin: the leaf payload `ChangePanelLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelLoad {
  panelId: string;
  newValue: number;
}

export const parseChangePanelLoad: NormWireReader<ChangePanelLoad> = normWireObject<ChangePanelLoad>({ panelId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
