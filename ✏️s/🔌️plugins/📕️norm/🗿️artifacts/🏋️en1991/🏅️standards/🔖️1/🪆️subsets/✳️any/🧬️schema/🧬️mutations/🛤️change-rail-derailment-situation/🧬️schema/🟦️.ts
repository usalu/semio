/** 🛤 `change-rail-derailment-situation` wire twin: the leaf payload `ChangeRailDerailmentSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailDerailmentSituation {
  id: string;
  newSituation: string;
}

export const parseChangeRailDerailmentSituation: NormWireReader<ChangeRailDerailmentSituation> = normWireObject<ChangeRailDerailmentSituation>({ id: normWireRequired(normWireString), newSituation: normWireRequired(normWireString) });
