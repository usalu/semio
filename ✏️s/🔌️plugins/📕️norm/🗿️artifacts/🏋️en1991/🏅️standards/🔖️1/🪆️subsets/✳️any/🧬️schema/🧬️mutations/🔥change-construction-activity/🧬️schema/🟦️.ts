/** 🔥 `change-construction-activity` wire twin: the leaf payload `ChangeConstructionActivity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConstructionActivity {
  newConstructionActivity: string;
}

export const parseChangeConstructionActivity: NormWireReader<ChangeConstructionActivity> = normWireObject<ChangeConstructionActivity>({ newConstructionActivity: normWireRequired(normWireString) });
