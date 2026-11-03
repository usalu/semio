/** 🛢️ `change-silo-course-elevation` wire twin: the leaf payload `ChangeSiloCourseElevation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloCourseElevation {
  id: string;
  newCourseElevation: number;
}

export const parseChangeSiloCourseElevation: NormWireReader<ChangeSiloCourseElevation> = normWireObject<ChangeSiloCourseElevation>({ id: normWireRequired(normWireString), newCourseElevation: normWireRequired(normWireNumber) });
