/** ⚙ `insert-rotating-machine` wire twin: the leaf payload `InsertRotatingMachine`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RotatingMachine, parseRotatingMachine } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRotatingMachine {
  index: number;
  item: RotatingMachine;
}

export const parseInsertRotatingMachine: NormWireReader<InsertRotatingMachine> = normWireObject<InsertRotatingMachine>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseRotatingMachine) });
