/** 🔩️ `change-joint-column-web-depth` wire twin: the leaf payload `ChangeJointColumnWebDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnWebDepth {
  id: string;
  newColumnWebDepth: number;
}

export const parseChangeJointColumnWebDepth: NormWireReader<ChangeJointColumnWebDepth> = normWireObject<ChangeJointColumnWebDepth>({ id: normWireRequired(normWireString), newColumnWebDepth: normWireRequired(normWireNumber) });
