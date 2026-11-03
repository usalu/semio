/** 🧱️ `change-panel-fire-duration` wire twin: the leaf payload `ChangePanelFireDuration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelFireDuration {
  panelId: string;
  newValue: number;
}

export const parseChangePanelFireDuration: NormWireReader<ChangePanelFireDuration> = normWireObject<ChangePanelFireDuration>({ panelId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
