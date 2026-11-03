/** ➕️ `insert-pile` wire twin: the leaf payload `InsertPile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SteelPile, parseSteelPile } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPile {
  index?: number | null;
  pile: SteelPile;
}

export const parseInsertPile: NormWireReader<InsertPile> = normWireObject<InsertPile>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), pile: normWireRequired(parseSteelPile) });
