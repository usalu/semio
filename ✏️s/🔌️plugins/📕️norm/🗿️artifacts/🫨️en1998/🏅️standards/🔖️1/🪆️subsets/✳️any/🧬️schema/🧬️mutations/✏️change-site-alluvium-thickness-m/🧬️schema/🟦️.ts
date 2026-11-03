/** ✏️ `change-site-alluvium-thickness-m` wire twin: the leaf payload `ChangeSiteAlluviumThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteAlluviumThicknessM {
  mutation: "changeSiteAlluviumThicknessM";
  newAlluviumThicknessM: number;
}

export const parseChangeSiteAlluviumThicknessM: NormWireReader<ChangeSiteAlluviumThicknessM> = normWireObject<ChangeSiteAlluviumThicknessM>({ mutation: normWireRequired(normWireLiteral("changeSiteAlluviumThicknessM")), newAlluviumThicknessM: normWireRequired(normWireNumber) });
