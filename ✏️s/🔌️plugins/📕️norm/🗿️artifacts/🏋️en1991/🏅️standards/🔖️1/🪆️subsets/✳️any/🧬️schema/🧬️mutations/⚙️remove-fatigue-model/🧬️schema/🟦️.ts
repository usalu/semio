/** ⚙ `remove-fatigue-model` wire twin: the leaf payload `RemoveFatigueModel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueModel {
  id: string;
}

export const parseRemoveFatigueModel: NormWireReader<RemoveFatigueModel> = normWireObject<RemoveFatigueModel>({ id: normWireRequired(normWireString) });
