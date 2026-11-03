/** 🏗️ `remove-construction-stage` wire twin: the leaf payload `RemoveConstructionStage`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveConstructionStage {
  id: string;
}

export const parseRemoveConstructionStage: NormWireReader<RemoveConstructionStage> = normWireObject<RemoveConstructionStage>({ id: normWireRequired(normWireString) });
