/** 🎲 `change-reliability-resistance-dist` wire twin: the leaf payload `ChangeReliabilityResistanceDist`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990DistributionKind, parseEn1990DistributionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeReliabilityResistanceDist {
  mutation: "changeReliabilityResistanceDist";
  id: string;
  newResistanceDistribution: En1990DistributionKind;
}

export const parseChangeReliabilityResistanceDist: NormWireReader<ChangeReliabilityResistanceDist> = normWireObject<ChangeReliabilityResistanceDist>({ mutation: normWireRequired(normWireLiteral("changeReliabilityResistanceDist")), id: normWireRequired(normWireString), newResistanceDistribution: normWireRequired(parseEn1990DistributionKind) });
