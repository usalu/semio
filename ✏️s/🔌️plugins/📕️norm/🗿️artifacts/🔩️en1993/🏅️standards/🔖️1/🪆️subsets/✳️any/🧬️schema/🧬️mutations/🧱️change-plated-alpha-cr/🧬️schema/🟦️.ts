/** 🧱️ `change-plated-alpha-cr` wire twin: the leaf payload `ChangePlatedAlphaCr`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedAlphaCr {
  id: string;
  newAlphaCr: number;
}

export const parseChangePlatedAlphaCr: NormWireReader<ChangePlatedAlphaCr> = normWireObject<ChangePlatedAlphaCr>({ id: normWireRequired(normWireString), newAlphaCr: normWireRequired(normWireNumber) });
