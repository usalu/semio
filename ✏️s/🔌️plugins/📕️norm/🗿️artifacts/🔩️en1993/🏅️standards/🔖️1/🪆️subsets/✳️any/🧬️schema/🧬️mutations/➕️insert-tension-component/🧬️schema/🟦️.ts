/** ➕️ `insert-tension-component` wire twin: the leaf payload `InsertTensionComponent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TensionComponent, parseTensionComponent } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTensionComponent {
  index?: number | null;
  tensionComponent: TensionComponent;
}

export const parseInsertTensionComponent: NormWireReader<InsertTensionComponent> = normWireObject<InsertTensionComponent>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), tensionComponent: normWireRequired(parseTensionComponent) });
