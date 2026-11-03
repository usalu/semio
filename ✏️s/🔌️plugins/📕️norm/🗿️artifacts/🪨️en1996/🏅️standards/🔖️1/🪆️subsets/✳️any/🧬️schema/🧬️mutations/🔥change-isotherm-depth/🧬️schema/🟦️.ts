/** 🔥 `change-isotherm-depth` wire twin: the leaf payload `ChangeIsothermDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeIsothermDepth {
  id: string;
  newIsothermDepthM: number;
}

export const parseChangeIsothermDepth: NormWireReader<ChangeIsothermDepth> = normWireObject<ChangeIsothermDepth>({ id: normWireRequired(normWireString), newIsothermDepthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
