/** 💥 `insert-impact-action` wire twin: the leaf payload `InsertImpactAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ImpactAction, parseImpactAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertImpactAction {
  index: number;
  item: ImpactAction;
}

export const parseInsertImpactAction: NormWireReader<InsertImpactAction> = normWireObject<InsertImpactAction>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseImpactAction) });
