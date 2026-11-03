/** ✏️ `change-building-ns-height-m` wire twin: the leaf payload `ChangeBuildingNsHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingNsHeightM {
  mutation: "changeBuildingNsHeightM";
  id: string;
  newNonstructuralHeightM: number;
}

export const parseChangeBuildingNsHeightM: NormWireReader<ChangeBuildingNsHeightM> = normWireObject<ChangeBuildingNsHeightM>({ mutation: normWireRequired(normWireLiteral("changeBuildingNsHeightM")), id: normWireRequired(normWireString), newNonstructuralHeightM: normWireRequired(normWireNumber) });
