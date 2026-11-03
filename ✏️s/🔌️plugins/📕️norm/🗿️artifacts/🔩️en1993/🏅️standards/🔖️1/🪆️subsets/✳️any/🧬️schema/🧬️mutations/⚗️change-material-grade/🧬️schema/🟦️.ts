/** ⚗️ `change-material-grade` wire twin: the leaf payload `ChangeMaterialGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialGrade {
  id: string;
  newGrade: string;
}

export const parseChangeMaterialGrade: NormWireReader<ChangeMaterialGrade> = normWireObject<ChangeMaterialGrade>({ id: normWireRequired(normWireString), newGrade: normWireRequired(normWireString) });
