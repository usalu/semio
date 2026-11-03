/** 🧱️ `change-plated-half-width` wire twin: the leaf payload `ChangePlatedHalfWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedHalfWidth {
  id: string;
  newHalfWidth: number;
}

export const parseChangePlatedHalfWidth: NormWireReader<ChangePlatedHalfWidth> = normWireObject<ChangePlatedHalfWidth>({ id: normWireRequired(normWireString), newHalfWidth: normWireRequired(normWireNumber) });
