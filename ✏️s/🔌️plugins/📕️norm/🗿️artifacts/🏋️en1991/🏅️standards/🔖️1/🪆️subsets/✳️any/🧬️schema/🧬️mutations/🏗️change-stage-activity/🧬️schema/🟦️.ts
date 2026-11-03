/** 🏗️ `change-stage-activity` wire twin: the leaf payload `ChangeStageActivity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStageActivity {
  id: string;
  newActivity: string;
}

export const parseChangeStageActivity: NormWireReader<ChangeStageActivity> = normWireObject<ChangeStageActivity>({ id: normWireRequired(normWireString), newActivity: normWireRequired(normWireString) });
