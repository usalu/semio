/** 🪜️ `change-built-up-panel-spacing` wire twin: the leaf payload `ChangeBuiltUpPanelSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuiltUpPanelSpacing {
  id: string;
  newPanelSpacing: number;
}

export const parseChangeBuiltUpPanelSpacing: NormWireReader<ChangeBuiltUpPanelSpacing> = normWireObject<ChangeBuiltUpPanelSpacing>({ id: normWireRequired(normWireString), newPanelSpacing: normWireRequired(normWireNumber) });
