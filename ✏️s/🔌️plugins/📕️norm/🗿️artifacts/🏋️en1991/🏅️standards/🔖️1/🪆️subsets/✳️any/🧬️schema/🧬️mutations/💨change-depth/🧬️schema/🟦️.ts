/** 💨 `change-depth` wire twin: the leaf payload `ChangeDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDepth {
  newDepth: number;
}

export const parseChangeDepth: NormWireReader<ChangeDepth> = normWireObject<ChangeDepth>({ newDepth: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
