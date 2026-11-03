/** 🛡️ `change-diaphragm-panel-width` wire twin: the leaf payload `ChangeDiaphragmPanelWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmPanelWidth {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmPanelWidth: NormWireReader<ChangeDiaphragmPanelWidth> = normWireObject<ChangeDiaphragmPanelWidth>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
