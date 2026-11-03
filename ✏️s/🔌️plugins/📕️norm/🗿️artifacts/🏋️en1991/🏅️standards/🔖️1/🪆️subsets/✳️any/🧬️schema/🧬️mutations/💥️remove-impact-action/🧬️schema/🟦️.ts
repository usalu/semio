/** 💥 `remove-impact-action` wire twin: the leaf payload `RemoveImpactAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveImpactAction {
  id: string;
}

export const parseRemoveImpactAction: NormWireReader<RemoveImpactAction> = normWireObject<RemoveImpactAction>({ id: normWireRequired(normWireString) });
