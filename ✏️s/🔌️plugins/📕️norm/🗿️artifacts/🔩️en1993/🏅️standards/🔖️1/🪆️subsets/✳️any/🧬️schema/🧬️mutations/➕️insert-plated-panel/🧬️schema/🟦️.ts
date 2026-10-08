/** ➕️ `insert-plated-panel` wire twin: the leaf payload `InsertPlatedPanel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parsePlatedPanel, type PlatedPanel } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPlatedPanel {
  index?: number | null;
  platedPanel: PlatedPanel;
}

export const parseInsertPlatedPanel: NormWireReader<InsertPlatedPanel> = normWireObject<InsertPlatedPanel>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), platedPanel: normWireRequired(parsePlatedPanel) });
