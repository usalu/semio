/** 🧩️ `change-joint-plate-bolt-class` wire twin: the leaf payload `ChangeJointPlateBoltClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateBoltClass {
  id: string;
  newBoltClass: string;
}

export const parseChangeJointPlateBoltClass: NormWireReader<ChangeJointPlateBoltClass> = normWireObject<ChangeJointPlateBoltClass>({ id: normWireRequired(normWireString), newBoltClass: normWireRequired(normWireString) });
