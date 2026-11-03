/** ➕️ `insert-bridge-variable` wire twin: the leaf payload `InsertBridgeVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998VariableAction, parseEn1998VariableAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridgeVariable {
  mutation: "insertBridgeVariable";
  bridgeId: string;
  index?: number | null;
  variable: En1998VariableAction;
}

export const parseInsertBridgeVariable: NormWireReader<InsertBridgeVariable> = normWireObject<InsertBridgeVariable>({ mutation: normWireRequired(normWireLiteral("insertBridgeVariable")), bridgeId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), variable: normWireRequired(parseEn1998VariableAction) });
