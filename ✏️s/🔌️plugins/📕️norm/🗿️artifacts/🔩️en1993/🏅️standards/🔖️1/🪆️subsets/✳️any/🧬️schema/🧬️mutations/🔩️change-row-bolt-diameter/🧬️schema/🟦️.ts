/** 🔩️ `change-row-bolt-diameter` wire twin: the leaf payload `ChangeRowBoltDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowBoltDiameter {
  id: string;
  newBoltDiameter: number;
}

export const parseChangeRowBoltDiameter: NormWireReader<ChangeRowBoltDiameter> = normWireObject<ChangeRowBoltDiameter>({ id: normWireRequired(normWireString), newBoltDiameter: normWireRequired(normWireNumber) });
