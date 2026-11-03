/** 🔩️ `change-joint-bolt-class` wire twin: the leaf payload `ChangeJointBoltClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBoltClass {
  id: string;
  newBoltClass: string;
}

export const parseChangeJointBoltClass: NormWireReader<ChangeJointBoltClass> = normWireObject<ChangeJointBoltClass>({ id: normWireRequired(normWireString), newBoltClass: normWireRequired(normWireString) });
