/** 🥶️ `change-cf-thickness` wire twin: the leaf payload `ChangeCfThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfThickness {
  id: string;
  newThickness: number;
}

export const parseChangeCfThickness: NormWireReader<ChangeCfThickness> = normWireObject<ChangeCfThickness>({ id: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
