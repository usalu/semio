/** ⚙ `remove-support-collision` wire twin: the leaf payload `RemoveSupportCollision`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSupportCollision {
  id: string;
}

export const parseRemoveSupportCollision: NormWireReader<RemoveSupportCollision> = normWireObject<RemoveSupportCollision>({ id: normWireRequired(normWireString) });
