/** 🧱 `remove-concrete-grade` wire twin: the leaf payload `RemoveConcreteGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConcreteGrade {
  index: number;
}

export const parseRemoveConcreteGrade: NormWireReader<RemoveConcreteGrade> = normWireObject<RemoveConcreteGrade>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
