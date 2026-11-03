/** 🧱️ `insert-slab-action` wire twin: the leaf payload `InsertSlabAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SlabAction, parseSlabAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSlabAction {
  slabId: string;
  index: number;
  action: SlabAction;
}

export const parseInsertSlabAction: NormWireReader<InsertSlabAction> = normWireObject<InsertSlabAction>({ slabId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), action: normWireRequired(parseSlabAction) });
