/** ➕️ `insert-panel-layer` wire twin: the leaf payload `InsertPanelLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PanelLayer, parsePanelLayer } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPanelLayer {
  panelId: string;
  index: number;
  layer: PanelLayer;
}

export const parseInsertPanelLayer: NormWireReader<InsertPanelLayer> = normWireObject<InsertPanelLayer>({ panelId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), layer: normWireRequired(parsePanelLayer) });
