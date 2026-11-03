/** ✏️ `change-building-ot-base-ym` wire twin: the leaf payload `ChangeBuildingOtBaseYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingOtBaseYM {
  mutation: "changeBuildingOtBaseYM";
  id: string;
  newOverturningBaseYM: number;
}

export const parseChangeBuildingOtBaseYM: NormWireReader<ChangeBuildingOtBaseYM> = normWireObject<ChangeBuildingOtBaseYM>({ mutation: normWireRequired(normWireLiteral("changeBuildingOtBaseYM")), id: normWireRequired(normWireString), newOverturningBaseYM: normWireRequired(normWireNumber) });
