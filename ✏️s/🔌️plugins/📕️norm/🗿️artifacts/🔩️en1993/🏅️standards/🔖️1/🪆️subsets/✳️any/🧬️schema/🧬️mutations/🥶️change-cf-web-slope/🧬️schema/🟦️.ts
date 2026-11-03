/** 🥶️ `change-cf-web-slope` wire twin: the leaf payload `ChangeCfWebSlope`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfWebSlope {
  id: string;
  newWebSlope: number;
}

export const parseChangeCfWebSlope: NormWireReader<ChangeCfWebSlope> = normWireObject<ChangeCfWebSlope>({ id: normWireRequired(normWireString), newWebSlope: normWireRequired(normWireNumber) });
