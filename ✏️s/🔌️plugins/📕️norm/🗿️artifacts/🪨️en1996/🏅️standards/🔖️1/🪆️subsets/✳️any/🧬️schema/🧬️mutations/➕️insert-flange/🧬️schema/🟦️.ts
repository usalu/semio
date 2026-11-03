/** ➕️ `insert-flange` wire twin: the leaf payload `InsertFlange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ShearFlange, parseShearFlange } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFlange {
  wallId: string;
  index: number;
  flange: ShearFlange;
}

export const parseInsertFlange: NormWireReader<InsertFlange> = normWireObject<InsertFlange>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), flange: normWireRequired(parseShearFlange) });
