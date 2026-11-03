/** 🌀 `insert-dynamic-element` wire twin: the leaf payload `InsertDynamicElement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DynamicElement, parseDynamicElement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertDynamicElement {
  index: number;
  item: DynamicElement;
}

export const parseInsertDynamicElement: NormWireReader<InsertDynamicElement> = normWireObject<InsertDynamicElement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseDynamicElement) });
