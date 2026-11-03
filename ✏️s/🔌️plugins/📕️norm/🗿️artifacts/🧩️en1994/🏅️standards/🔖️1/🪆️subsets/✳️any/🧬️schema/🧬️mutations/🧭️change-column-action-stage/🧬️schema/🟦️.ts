/** 🧭️ `change-column-action-stage` wire twin: the leaf payload `ChangeColumnActionStage`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionStage, parseActionStage } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeColumnActionStage {
  columnId: string;
  id: string;
  newStage: ActionStage;
}

export const parseChangeColumnActionStage: NormWireReader<ChangeColumnActionStage> = normWireObject<ChangeColumnActionStage>({ columnId: normWireRequired(normWireString), id: normWireRequired(normWireString), newStage: normWireRequired(parseActionStage) });
