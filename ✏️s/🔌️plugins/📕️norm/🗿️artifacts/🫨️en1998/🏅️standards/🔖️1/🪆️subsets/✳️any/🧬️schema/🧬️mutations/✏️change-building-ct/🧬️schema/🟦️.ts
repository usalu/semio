/** ✏️ `change-building-ct` wire twin: the leaf payload `ChangeBuildingCt`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingCt {
  mutation: "changeBuildingCt";
  id: string;
  newCt: number;
}

export const parseChangeBuildingCt: NormWireReader<ChangeBuildingCt> = normWireObject<ChangeBuildingCt>({ mutation: normWireRequired(normWireLiteral("changeBuildingCt")), id: normWireRequired(normWireString), newCt: normWireRequired(normWireNumber) });
