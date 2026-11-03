/** 🧱 `change-self-weight-material` wire twin: the leaf payload `ChangeSelfWeightMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSelfWeightMaterial {
  id: string;
  newMaterial: string;
}

export const parseChangeSelfWeightMaterial: NormWireReader<ChangeSelfWeightMaterial> = normWireObject<ChangeSelfWeightMaterial>({ id: normWireRequired(normWireString), newMaterial: normWireRequired(normWireString) });
