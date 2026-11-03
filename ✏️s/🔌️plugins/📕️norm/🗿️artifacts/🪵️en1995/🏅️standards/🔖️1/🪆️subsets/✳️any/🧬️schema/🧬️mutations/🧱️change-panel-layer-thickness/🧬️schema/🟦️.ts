/** 🧱️ `change-panel-layer-thickness` wire twin: the leaf payload `ChangePanelLayerThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelLayerThickness {
  panelId: string;
  layerId: string;
  newValue: number;
}

export const parseChangePanelLayerThickness: NormWireReader<ChangePanelLayerThickness> = normWireObject<ChangePanelLayerThickness>({ panelId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
