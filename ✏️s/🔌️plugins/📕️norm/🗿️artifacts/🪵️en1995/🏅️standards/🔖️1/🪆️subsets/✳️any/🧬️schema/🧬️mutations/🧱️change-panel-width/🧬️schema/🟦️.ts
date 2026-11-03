/** 🧱️ `change-panel-width` wire twin: the leaf payload `ChangePanelWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelWidth {
  panelId: string;
  newValue: number;
}

export const parseChangePanelWidth: NormWireReader<ChangePanelWidth> = normWireObject<ChangePanelWidth>({ panelId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
