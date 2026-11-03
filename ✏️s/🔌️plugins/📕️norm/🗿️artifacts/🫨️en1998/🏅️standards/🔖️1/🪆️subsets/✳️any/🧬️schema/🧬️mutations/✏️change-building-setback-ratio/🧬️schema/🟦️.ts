/** ✏️ `change-building-setback-ratio` wire twin: the leaf payload `ChangeBuildingSetbackRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingSetbackRatio {
  mutation: "changeBuildingSetbackRatio";
  id: string;
  newSetbackRatio: number;
}

export const parseChangeBuildingSetbackRatio: NormWireReader<ChangeBuildingSetbackRatio> = normWireObject<ChangeBuildingSetbackRatio>({ mutation: normWireRequired(normWireLiteral("changeBuildingSetbackRatio")), id: normWireRequired(normWireString), newSetbackRatio: normWireRequired(normWireNumber) });
