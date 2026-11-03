/** 🔗 `change-tie-run-y` wire twin: the leaf payload `ChangeTieRunY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunY {
  id: string;
  newY: number;
}

export const parseChangeTieRunY: NormWireReader<ChangeTieRunY> = normWireObject<ChangeTieRunY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
