/** 🧱️ `change-plated-fy` wire twin: the leaf payload `ChangePlatedFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFy {
  id: string;
  newFy: number;
}

export const parseChangePlatedFy: NormWireReader<ChangePlatedFy> = normWireObject<ChangePlatedFy>({ id: normWireRequired(normWireString), newFy: normWireRequired(normWireNumber) });
