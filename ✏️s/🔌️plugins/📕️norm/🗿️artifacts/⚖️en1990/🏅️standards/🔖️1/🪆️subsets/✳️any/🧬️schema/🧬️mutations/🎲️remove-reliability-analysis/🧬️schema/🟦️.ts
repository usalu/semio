/** 🎲 `remove-reliability-analysis` wire twin: the leaf payload `RemoveReliabilityAnalysis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveReliabilityAnalysis {
  mutation: "removeReliabilityAnalysis";
  index: number;
}

export const parseRemoveReliabilityAnalysis: NormWireReader<RemoveReliabilityAnalysis> = normWireObject<RemoveReliabilityAnalysis>({ mutation: normWireRequired(normWireLiteral("removeReliabilityAnalysis")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
