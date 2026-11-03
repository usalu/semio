/** 🎲 `change-reliability-action-sd` wire twin: the leaf payload `ChangeReliabilityActionSd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReliabilityActionSd {
  mutation: "changeReliabilityActionSd";
  id: string;
  newActionSd: number;
}

export const parseChangeReliabilityActionSd: NormWireReader<ChangeReliabilityActionSd> = normWireObject<ChangeReliabilityActionSd>({ mutation: normWireRequired(normWireLiteral("changeReliabilityActionSd")), id: normWireRequired(normWireString), newActionSd: normWireRequired(normWireNumber) });
