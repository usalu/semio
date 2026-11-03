/** 🧱️ `change-plated-stress-z` wire twin: the leaf payload `ChangePlatedStressZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStressZ {
  id: string;
  newStressZ: number;
}

export const parseChangePlatedStressZ: NormWireReader<ChangePlatedStressZ> = normWireObject<ChangePlatedStressZ>({ id: normWireRequired(normWireString), newStressZ: normWireRequired(normWireNumber) });
