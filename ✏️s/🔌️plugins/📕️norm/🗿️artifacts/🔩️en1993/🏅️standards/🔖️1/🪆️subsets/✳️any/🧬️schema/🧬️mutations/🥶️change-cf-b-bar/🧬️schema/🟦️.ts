/** 🥶️ `change-cf-b-bar` wire twin: the leaf payload `ChangeCfBBar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfBBar {
  id: string;
  newBBar: number;
}

export const parseChangeCfBBar: NormWireReader<ChangeCfBBar> = normWireObject<ChangeCfBBar>({ id: normWireRequired(normWireString), newBBar: normWireRequired(normWireNumber) });
