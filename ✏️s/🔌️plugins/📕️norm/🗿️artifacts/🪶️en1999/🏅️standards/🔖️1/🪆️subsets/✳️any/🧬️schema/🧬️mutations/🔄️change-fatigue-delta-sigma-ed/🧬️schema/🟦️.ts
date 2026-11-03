/** 🔄️ `change-fatigue-delta-sigma-ed` wire twin: the leaf payload `ChangeFatigueDeltaSigmaEd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueDeltaSigmaEd {
  mutation: "changeFatigueDeltaSigmaEd";
  fatigueDetailId: string;
  newDeltaSigmaEd: number;
}

export const parseChangeFatigueDeltaSigmaEd: NormWireReader<ChangeFatigueDeltaSigmaEd> = normWireObject<ChangeFatigueDeltaSigmaEd>({ mutation: normWireRequired(normWireLiteral("changeFatigueDeltaSigmaEd")), fatigueDetailId: normWireRequired(normWireString), newDeltaSigmaEd: normWireRequired(normWireNumber) });
