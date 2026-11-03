/** 🛤 `change-fatigue-model-model` wire twin: the leaf payload `ChangeFatigueModelModel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueModelModel {
  id: string;
  newModel: string;
}

export const parseChangeFatigueModelModel: NormWireReader<ChangeFatigueModelModel> = normWireObject<ChangeFatigueModelModel>({ id: normWireRequired(normWireString), newModel: normWireRequired(normWireString) });
