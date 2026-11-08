/** ➕️ `insert-isolation-system` wire twin: the leaf payload `InsertIsolationSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998IsolationSystem, parseEn1998IsolationSystem } from "../../../📸️snapshot/🟦️.ts";

export interface InsertIsolationSystem {
  mutation: "insertIsolationSystem";
  index?: number | null;
  isolationSystem: En1998IsolationSystem;
}

export const parseInsertIsolationSystem: NormWireReader<InsertIsolationSystem> = normWireObject<InsertIsolationSystem>({ mutation: normWireRequired(normWireLiteral("insertIsolationSystem")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), isolationSystem: normWireRequired(parseEn1998IsolationSystem) });
