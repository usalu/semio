/** 🕳️ `change-ground-investigation-position-y` wire twin: the leaf payload `ChangeGroundInvestigationPositionY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundInvestigationPositionY {
  mutation: "changeGroundInvestigationPositionY";
  id: string;
  newPositionY: number;
}

export const parseChangeGroundInvestigationPositionY: NormWireReader<ChangeGroundInvestigationPositionY> = normWireObject<ChangeGroundInvestigationPositionY>({ mutation: normWireRequired(normWireLiteral("changeGroundInvestigationPositionY")), id: normWireRequired(normWireString), newPositionY: normWireRequired(normWireNumber) });
