/** 🗼️ `remove-tower-action` wire twin: the leaf payload `RemoveTowerAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTowerAction {
  id: string;
  index: number;
}

export const parseRemoveTowerAction: NormWireReader<RemoveTowerAction> = normWireObject<RemoveTowerAction>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
