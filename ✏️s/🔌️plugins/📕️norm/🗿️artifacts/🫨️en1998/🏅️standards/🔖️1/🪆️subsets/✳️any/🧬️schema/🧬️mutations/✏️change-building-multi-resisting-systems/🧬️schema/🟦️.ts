/** ✏️ `change-building-multi-resisting-systems` wire twin: the leaf payload `ChangeBuildingMultiResistingSystems`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingMultiResistingSystems {
  mutation: "changeBuildingMultiResistingSystems";
  id: string;
  newMultipleResistingSystems: boolean;
}

export const parseChangeBuildingMultiResistingSystems: NormWireReader<ChangeBuildingMultiResistingSystems> = normWireObject<ChangeBuildingMultiResistingSystems>({ mutation: normWireRequired(normWireLiteral("changeBuildingMultiResistingSystems")), id: normWireRequired(normWireString), newMultipleResistingSystems: normWireRequired(normWireBoolean) });
