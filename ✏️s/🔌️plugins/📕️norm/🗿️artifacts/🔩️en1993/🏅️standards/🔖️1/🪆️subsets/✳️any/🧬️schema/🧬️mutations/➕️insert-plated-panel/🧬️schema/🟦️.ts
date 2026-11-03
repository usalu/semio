/** ➕️ `insert-plated-panel` wire twin: the leaf payload `InsertPlatedPanel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PlatedPanel, parsePlatedPanel } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPlatedPanel {
  index?: number | null;
  platedPanel: PlatedPanel;
}

export const parseInsertPlatedPanel: NormWireReader<InsertPlatedPanel> = normWireObject<InsertPlatedPanel>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), platedPanel: normWireRequired(parsePlatedPanel) });
