/** 🎲 `change-reliability-action-dist` wire twin: the leaf payload `ChangeReliabilityActionDist`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990DistributionKind, parseEn1990DistributionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeReliabilityActionDist {
  mutation: "changeReliabilityActionDist";
  id: string;
  newActionDistribution: En1990DistributionKind;
}

export const parseChangeReliabilityActionDist: NormWireReader<ChangeReliabilityActionDist> = normWireObject<ChangeReliabilityActionDist>({ mutation: normWireRequired(normWireLiteral("changeReliabilityActionDist")), id: normWireRequired(normWireString), newActionDistribution: normWireRequired(parseEn1990DistributionKind) });
