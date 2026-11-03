/** 🧱️ `change-forming-gain-used-yield` wire twin: the leaf payload `ChangeFormingGainUsedYield`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFormingGainUsedYield {
  id: string;
  newUsedYield: number;
}

export const parseChangeFormingGainUsedYield: NormWireReader<ChangeFormingGainUsedYield> = normWireObject<ChangeFormingGainUsedYield>({ id: normWireRequired(normWireString), newUsedYield: normWireRequired(normWireNumber) });
