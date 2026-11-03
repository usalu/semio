/** 🎲 `change-reliability-action-mean` wire twin: the leaf payload `ChangeReliabilityActionMean`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReliabilityActionMean {
  mutation: "changeReliabilityActionMean";
  id: string;
  newActionMean: number;
}

export const parseChangeReliabilityActionMean: NormWireReader<ChangeReliabilityActionMean> = normWireObject<ChangeReliabilityActionMean>({ mutation: normWireRequired(normWireLiteral("changeReliabilityActionMean")), id: normWireRequired(normWireString), newActionMean: normWireRequired(normWireNumber) });
