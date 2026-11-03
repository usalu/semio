/** 🔩️ `change-row-bolt-to-web` wire twin: the leaf payload `ChangeRowBoltToWeb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowBoltToWeb {
  id: string;
  newBoltToWeb: number;
}

export const parseChangeRowBoltToWeb: NormWireReader<ChangeRowBoltToWeb> = normWireObject<ChangeRowBoltToWeb>({ id: normWireRequired(normWireString), newBoltToWeb: normWireRequired(normWireNumber) });
