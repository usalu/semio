/** ✏️ `change-foundation-liq-magnitude` wire twin: the leaf payload `ChangeFoundationLiqMagnitude`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqMagnitude {
  mutation: "changeFoundationLiqMagnitude";
  id: string;
  newLiqMagnitude: number;
}

export const parseChangeFoundationLiqMagnitude: NormWireReader<ChangeFoundationLiqMagnitude> = normWireObject<ChangeFoundationLiqMagnitude>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqMagnitude")), id: normWireRequired(normWireString), newLiqMagnitude: normWireRequired(normWireNumber) });
