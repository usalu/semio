/** 🥞 `change-self-weight-thickness` wire twin: the leaf payload `ChangeSelfWeightThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSelfWeightThickness {
  id: string;
  newThickness: number;
}

export const parseChangeSelfWeightThickness: NormWireReader<ChangeSelfWeightThickness> = normWireObject<ChangeSelfWeightThickness>({ id: normWireRequired(normWireString), newThickness: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
