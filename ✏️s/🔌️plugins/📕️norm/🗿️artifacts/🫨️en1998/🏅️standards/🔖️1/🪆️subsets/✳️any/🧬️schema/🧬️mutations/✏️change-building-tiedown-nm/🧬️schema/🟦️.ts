/** ✏️ `change-building-tiedown-nm` wire twin: the leaf payload `ChangeBuildingTiedownNm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingTiedownNm {
  mutation: "changeBuildingTiedownNm";
  id: string;
  newTieDownMomentNm: number;
}

export const parseChangeBuildingTiedownNm: NormWireReader<ChangeBuildingTiedownNm> = normWireObject<ChangeBuildingTiedownNm>({ mutation: normWireRequired(normWireLiteral("changeBuildingTiedownNm")), id: normWireRequired(normWireString), newTieDownMomentNm: normWireRequired(normWireNumber) });
