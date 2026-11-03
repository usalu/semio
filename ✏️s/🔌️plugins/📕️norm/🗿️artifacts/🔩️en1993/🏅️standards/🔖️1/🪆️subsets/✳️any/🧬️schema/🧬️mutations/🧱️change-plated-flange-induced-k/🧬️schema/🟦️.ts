/** 🧱️ `change-plated-flange-induced-k` wire twin: the leaf payload `ChangePlatedFlangeInducedK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFlangeInducedK {
  id: string;
  newFlangeInducedK: number;
}

export const parseChangePlatedFlangeInducedK: NormWireReader<ChangePlatedFlangeInducedK> = normWireObject<ChangePlatedFlangeInducedK>({ id: normWireRequired(normWireString), newFlangeInducedK: normWireRequired(normWireNumber) });
