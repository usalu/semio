/** 🧱️ `change-plated-stress-x` wire twin: the leaf payload `ChangePlatedStressX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStressX {
  id: string;
  newStressX: number;
}

export const parseChangePlatedStressX: NormWireReader<ChangePlatedStressX> = normWireObject<ChangePlatedStressX>({ id: normWireRequired(normWireString), newStressX: normWireRequired(normWireNumber) });
