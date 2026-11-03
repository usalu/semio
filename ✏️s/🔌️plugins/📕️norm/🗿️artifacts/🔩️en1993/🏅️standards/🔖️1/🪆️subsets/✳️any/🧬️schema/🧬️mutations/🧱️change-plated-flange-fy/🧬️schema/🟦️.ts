/** 🧱️ `change-plated-flange-fy` wire twin: the leaf payload `ChangePlatedFlangeFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFlangeFy {
  id: string;
  newFlangeFy: number;
}

export const parseChangePlatedFlangeFy: NormWireReader<ChangePlatedFlangeFy> = normWireObject<ChangePlatedFlangeFy>({ id: normWireRequired(normWireString), newFlangeFy: normWireRequired(normWireNumber) });
