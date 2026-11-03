/** 🧭️ `change-slab-action-stage` wire twin: the leaf payload `ChangeSlabActionStage`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionStage, parseActionStage } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSlabActionStage {
  slabId: string;
  id: string;
  newStage: ActionStage;
}

export const parseChangeSlabActionStage: NormWireReader<ChangeSlabActionStage> = normWireObject<ChangeSlabActionStage>({ slabId: normWireRequired(normWireString), id: normWireRequired(normWireString), newStage: normWireRequired(parseActionStage) });
