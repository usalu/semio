/** 🔄️ `change-fatigue-delta-sigma-c` wire twin: the leaf payload `ChangeFatigueDeltaSigmaC`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueDeltaSigmaC {
  mutation: "changeFatigueDeltaSigmaC";
  fatigueDetailId: string;
  newDeltaSigmaC: number;
}

export const parseChangeFatigueDeltaSigmaC: NormWireReader<ChangeFatigueDeltaSigmaC> = normWireObject<ChangeFatigueDeltaSigmaC>({ mutation: normWireRequired(normWireLiteral("changeFatigueDeltaSigmaC")), fatigueDetailId: normWireRequired(normWireString), newDeltaSigmaC: normWireRequired(normWireNumber) });
