/** 🔩️ `change-joint-bolt-to-web` wire twin: the leaf payload `ChangeJointBoltToWeb`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBoltToWeb {
  id: string;
  newBoltToWeb: number;
}

export const parseChangeJointBoltToWeb: NormWireReader<ChangeJointBoltToWeb> = normWireObject<ChangeJointBoltToWeb>({ id: normWireRequired(normWireString), newBoltToWeb: normWireRequired(normWireNumber) });
