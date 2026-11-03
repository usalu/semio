/** ✏️ `change-building-nu` wire twin: the leaf payload `ChangeBuildingNu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingNu {
  mutation: "changeBuildingNu";
  id: string;
  newNu: number;
}

export const parseChangeBuildingNu: NormWireReader<ChangeBuildingNu> = normWireObject<ChangeBuildingNu>({ mutation: normWireRequired(normWireLiteral("changeBuildingNu")), id: normWireRequired(normWireString), newNu: normWireRequired(normWireNumber) });
