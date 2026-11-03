/** 🔩️ `change-row-bolt-stress-area` wire twin: the leaf payload `ChangeRowBoltStressArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowBoltStressArea {
  id: string;
  newBoltStressArea: number;
}

export const parseChangeRowBoltStressArea: NormWireReader<ChangeRowBoltStressArea> = normWireObject<ChangeRowBoltStressArea>({ id: normWireRequired(normWireString), newBoltStressArea: normWireRequired(normWireNumber) });
