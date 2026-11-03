/** ✏️ `change-foundation-liq-water-depth-m` wire twin: the leaf payload `ChangeFoundationLiqWaterDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqWaterDepthM {
  mutation: "changeFoundationLiqWaterDepthM";
  id: string;
  newLiqWaterDepthM: number;
}

export const parseChangeFoundationLiqWaterDepthM: NormWireReader<ChangeFoundationLiqWaterDepthM> = normWireObject<ChangeFoundationLiqWaterDepthM>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqWaterDepthM")), id: normWireRequired(normWireString), newLiqWaterDepthM: normWireRequired(normWireNumber) });
