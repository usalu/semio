/** ⭕ `change-bar-layer-bond-condition` wire twin: the leaf payload `ChangeBarLayerBondCondition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BondCondition, parseBondCondition } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBarLayerBondCondition {
  memberId: string;
  layerId: string;
  newBondCondition: BondCondition;
}

export const parseChangeBarLayerBondCondition: NormWireReader<ChangeBarLayerBondCondition> = normWireObject<ChangeBarLayerBondCondition>({ memberId: normWireRequired(normWireString), layerId: normWireRequired(normWireString), newBondCondition: normWireRequired(parseBondCondition) });
