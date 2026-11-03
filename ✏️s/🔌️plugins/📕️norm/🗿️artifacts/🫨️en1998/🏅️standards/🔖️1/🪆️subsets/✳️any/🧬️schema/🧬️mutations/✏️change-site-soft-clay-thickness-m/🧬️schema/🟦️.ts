/** ✏️ `change-site-soft-clay-thickness-m` wire twin: the leaf payload `ChangeSiteSoftClayThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteSoftClayThicknessM {
  mutation: "changeSiteSoftClayThicknessM";
  newSoftClayThicknessM: number;
}

export const parseChangeSiteSoftClayThicknessM: NormWireReader<ChangeSiteSoftClayThicknessM> = normWireObject<ChangeSiteSoftClayThicknessM>({ mutation: normWireRequired(normWireLiteral("changeSiteSoftClayThicknessM")), newSoftClayThicknessM: normWireRequired(normWireNumber) });
