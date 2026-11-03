/** 🧱 `insert-concrete-grade` wire twin: the leaf payload `InsertConcreteGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ConcreteGrade, parseConcreteGrade } from "../../../📸️snapshot/🟦️.ts";

export interface InsertConcreteGrade {
  index: number;
  concreteGrade: ConcreteGrade;
}

export const parseInsertConcreteGrade: NormWireReader<InsertConcreteGrade> = normWireObject<InsertConcreteGrade>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), concreteGrade: normWireRequired(parseConcreteGrade) });
