/** ➕️ `insert-tension-component` wire twin: the leaf payload `InsertTensionComponent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTensionComponent, type TensionComponent } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTensionComponent {
  index?: number | null;
  tensionComponent: TensionComponent;
}

export const parseInsertTensionComponent: NormWireReader<InsertTensionComponent> = normWireObject<InsertTensionComponent>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), tensionComponent: normWireRequired(parseTensionComponent) });
