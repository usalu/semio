/** 🎲 `change-reliability-resistance-sd` wire twin: the leaf payload `ChangeReliabilityResistanceSd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReliabilityResistanceSd {
  mutation: "changeReliabilityResistanceSd";
  id: string;
  newResistanceSd: number;
}

export const parseChangeReliabilityResistanceSd: NormWireReader<ChangeReliabilityResistanceSd> = normWireObject<ChangeReliabilityResistanceSd>({ mutation: normWireRequired(normWireLiteral("changeReliabilityResistanceSd")), id: normWireRequired(normWireString), newResistanceSd: normWireRequired(normWireNumber) });
