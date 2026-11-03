/** ➕️ `insert-tower-variable` wire twin: the leaf payload `InsertTowerVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998VariableAction, parseEn1998VariableAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTowerVariable {
  mutation: "insertTowerVariable";
  towerId: string;
  index?: number | null;
  variable: En1998VariableAction;
}

export const parseInsertTowerVariable: NormWireReader<InsertTowerVariable> = normWireObject<InsertTowerVariable>({ mutation: normWireRequired(normWireLiteral("insertTowerVariable")), towerId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), variable: normWireRequired(parseEn1998VariableAction) });
