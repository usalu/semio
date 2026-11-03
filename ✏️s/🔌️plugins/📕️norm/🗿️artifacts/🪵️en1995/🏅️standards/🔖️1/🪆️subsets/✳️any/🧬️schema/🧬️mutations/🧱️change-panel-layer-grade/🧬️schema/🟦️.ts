/** 🧱️ `change-panel-layer-grade` wire twin: the leaf payload `ChangePanelLayerGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelLayerGrade {
  panelId: string;
  layerId: string;
  newValue: string;
}

export const parseChangePanelLayerGrade: NormWireReader<ChangePanelLayerGrade> = normWireObject<ChangePanelLayerGrade>({ panelId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
