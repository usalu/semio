/** 🔗 `change-tie-run-x` wire twin: the leaf payload `ChangeTieRunX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunX {
  id: string;
  newX: number;
}

export const parseChangeTieRunX: NormWireReader<ChangeTieRunX> = normWireObject<ChangeTieRunX>({ id: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
