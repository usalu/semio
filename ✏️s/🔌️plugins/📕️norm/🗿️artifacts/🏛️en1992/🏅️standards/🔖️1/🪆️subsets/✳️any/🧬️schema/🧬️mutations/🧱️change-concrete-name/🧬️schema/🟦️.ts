/** 🧱 `change-concrete-name` wire twin: the leaf payload `ChangeConcreteName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConcreteName {
  gradeId: string;
  newName: string;
}

export const parseChangeConcreteName: NormWireReader<ChangeConcreteName> = normWireObject<ChangeConcreteName>({ gradeId: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
