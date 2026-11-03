/** 🧱️ `change-panel-layer-orientation` wire twin: the leaf payload `ChangePanelLayerOrientation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LayerOrientation, parseLayerOrientation } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePanelLayerOrientation {
  panelId: string;
  layerId: string;
  newValue: LayerOrientation;
}

export const parseChangePanelLayerOrientation: NormWireReader<ChangePanelLayerOrientation> = normWireObject<ChangePanelLayerOrientation>({ panelId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newValue: normWireRequired(parseLayerOrientation) });
