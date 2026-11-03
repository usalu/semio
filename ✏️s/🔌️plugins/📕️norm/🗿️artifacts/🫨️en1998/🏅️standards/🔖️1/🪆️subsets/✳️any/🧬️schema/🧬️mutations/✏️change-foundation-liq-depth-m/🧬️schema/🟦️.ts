/** ✏️ `change-foundation-liq-depth-m` wire twin: the leaf payload `ChangeFoundationLiqDepthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqDepthM {
  mutation: "changeFoundationLiqDepthM";
  id: string;
  newLiqDepthM: number;
}

export const parseChangeFoundationLiqDepthM: NormWireReader<ChangeFoundationLiqDepthM> = normWireObject<ChangeFoundationLiqDepthM>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqDepthM")), id: normWireRequired(normWireString), newLiqDepthM: normWireRequired(normWireNumber) });
