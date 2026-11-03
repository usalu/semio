/** ✏️ `change-foundation-liq-unit-wt-nm3` wire twin: the leaf payload `ChangeFoundationLiqUnitWtNM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqUnitWtNM3 {
  mutation: "changeFoundationLiqUnitWtNM3";
  id: string;
  newLiqUnitWeightNM3: number;
}

export const parseChangeFoundationLiqUnitWtNM3: NormWireReader<ChangeFoundationLiqUnitWtNM3> = normWireObject<ChangeFoundationLiqUnitWtNM3>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqUnitWtNM3")), id: normWireRequired(normWireString), newLiqUnitWeightNM3: normWireRequired(normWireNumber) });
