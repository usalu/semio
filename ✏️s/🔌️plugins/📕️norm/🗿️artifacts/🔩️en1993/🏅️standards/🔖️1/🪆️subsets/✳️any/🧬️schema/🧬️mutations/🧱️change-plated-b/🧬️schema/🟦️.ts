/** 🧱️ `change-plated-b` wire twin: the leaf payload `ChangePlatedB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedB {
  id: string;
  newB: number;
}

export const parseChangePlatedB: NormWireReader<ChangePlatedB> = normWireObject<ChangePlatedB>({ id: normWireRequired(normWireString), newB: normWireRequired(normWireNumber) });
