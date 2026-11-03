/** ✏️ `change-building-ot-base-xm` wire twin: the leaf payload `ChangeBuildingOtBaseXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingOtBaseXM {
  mutation: "changeBuildingOtBaseXM";
  id: string;
  newOverturningBaseXM: number;
}

export const parseChangeBuildingOtBaseXM: NormWireReader<ChangeBuildingOtBaseXM> = normWireObject<ChangeBuildingOtBaseXM>({ mutation: normWireRequired(normWireLiteral("changeBuildingOtBaseXM")), id: normWireRequired(normWireString), newOverturningBaseXM: normWireRequired(normWireNumber) });
