/** 🎲 `change-reliability-resistance-mean` wire twin: the leaf payload `ChangeReliabilityResistanceMean`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReliabilityResistanceMean {
  mutation: "changeReliabilityResistanceMean";
  id: string;
  newResistanceMean: number;
}

export const parseChangeReliabilityResistanceMean: NormWireReader<ChangeReliabilityResistanceMean> = normWireObject<ChangeReliabilityResistanceMean>({ mutation: normWireRequired(normWireLiteral("changeReliabilityResistanceMean")), id: normWireRequired(normWireString), newResistanceMean: normWireRequired(normWireNumber) });
