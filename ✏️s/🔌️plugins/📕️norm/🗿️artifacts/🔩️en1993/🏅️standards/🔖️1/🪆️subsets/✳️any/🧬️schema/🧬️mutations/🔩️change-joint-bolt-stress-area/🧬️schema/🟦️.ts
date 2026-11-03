/** 🔩️ `change-joint-bolt-stress-area` wire twin: the leaf payload `ChangeJointBoltStressArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBoltStressArea {
  id: string;
  newBoltStressArea: number;
}

export const parseChangeJointBoltStressArea: NormWireReader<ChangeJointBoltStressArea> = normWireObject<ChangeJointBoltStressArea>({ id: normWireRequired(normWireString), newBoltStressArea: normWireRequired(normWireNumber) });
