/** 🪚️ `change-chase-depth` wire twin: the leaf payload `ChangeChaseDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeChaseDepth {
  id: string;
  newDepthM: number;
}

export const parseChangeChaseDepth: NormWireReader<ChangeChaseDepth> = normWireObject<ChangeChaseDepth>({ id: normWireRequired(normWireString), newDepthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
