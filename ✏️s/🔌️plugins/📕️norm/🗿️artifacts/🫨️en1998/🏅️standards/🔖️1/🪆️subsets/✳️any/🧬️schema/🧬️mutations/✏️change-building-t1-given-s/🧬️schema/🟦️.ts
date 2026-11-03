/** ✏️ `change-building-t1-given-s` wire twin: the leaf payload `ChangeBuildingT1GivenS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingT1GivenS {
  mutation: "changeBuildingT1GivenS";
  id: string;
  newT1GivenS: number;
}

export const parseChangeBuildingT1GivenS: NormWireReader<ChangeBuildingT1GivenS> = normWireObject<ChangeBuildingT1GivenS>({ mutation: normWireRequired(normWireLiteral("changeBuildingT1GivenS")), id: normWireRequired(normWireString), newT1GivenS: normWireRequired(normWireNumber) });
