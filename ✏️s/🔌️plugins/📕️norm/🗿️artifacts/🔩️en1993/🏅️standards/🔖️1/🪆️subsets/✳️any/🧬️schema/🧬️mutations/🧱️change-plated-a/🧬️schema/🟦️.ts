/** 🧱️ `change-plated-a` wire twin: the leaf payload `ChangePlatedA`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedA {
  id: string;
  newA: number;
}

export const parseChangePlatedA: NormWireReader<ChangePlatedA> = normWireObject<ChangePlatedA>({ id: normWireRequired(normWireString), newA: normWireRequired(normWireNumber) });
