/** ➕️ `insert-panel` wire twin: the leaf payload `InsertPanel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Panel, parsePanel } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPanel {
  index: number;
  panel: Panel;
}

export const parseInsertPanel: NormWireReader<InsertPanel> = normWireObject<InsertPanel>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), panel: normWireRequired(parsePanel) });
