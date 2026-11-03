/** 🏋️ `remove-beam-action` wire twin: the leaf payload `RemoveBeamAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveBeamAction {
  beamId: string;
  id: string;
}

export const parseRemoveBeamAction: NormWireReader<RemoveBeamAction> = normWireObject<RemoveBeamAction>({ beamId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
