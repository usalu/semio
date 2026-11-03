/** ✏️ `change-foundation-liq-layer-m` wire twin: the leaf payload `ChangeFoundationLiqLayerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqLayerM {
  mutation: "changeFoundationLiqLayerM";
  id: string;
  newLiqLayerThicknessM: number;
}

export const parseChangeFoundationLiqLayerM: NormWireReader<ChangeFoundationLiqLayerM> = normWireObject<ChangeFoundationLiqLayerM>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqLayerM")), id: normWireRequired(normWireString), newLiqLayerThicknessM: normWireRequired(normWireNumber) });
