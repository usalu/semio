/** 🧱️ `change-plated-stiffener-depth` wire twin: the leaf payload `ChangePlatedStiffenerDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerDepth {
  id: string;
  newStiffenerDepth: number;
}

export const parseChangePlatedStiffenerDepth: NormWireReader<ChangePlatedStiffenerDepth> = normWireObject<ChangePlatedStiffenerDepth>({ id: normWireRequired(normWireString), newStiffenerDepth: normWireRequired(normWireNumber) });
