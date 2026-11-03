/** ➖️ `remove-panel-layer` wire twin: the leaf payload `RemovePanelLayer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePanelLayer {
  panelId: string;
  id: string;
}

export const parseRemovePanelLayer: NormWireReader<RemovePanelLayer> = normWireObject<RemovePanelLayer>({ panelId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
