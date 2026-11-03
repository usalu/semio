/** ⚙ `remove-rail-derailment` wire twin: the leaf payload `RemoveRailDerailment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveRailDerailment {
  id: string;
}

export const parseRemoveRailDerailment: NormWireReader<RemoveRailDerailment> = normWireObject<RemoveRailDerailment>({ id: normWireRequired(normWireString) });
