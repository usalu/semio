/** 🕳️ `change-ground-investigation-position-x` wire twin: the leaf payload `ChangeGroundInvestigationPositionX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundInvestigationPositionX {
  mutation: "changeGroundInvestigationPositionX";
  id: string;
  newPositionX: number;
}

export const parseChangeGroundInvestigationPositionX: NormWireReader<ChangeGroundInvestigationPositionX> = normWireObject<ChangeGroundInvestigationPositionX>({ mutation: normWireRequired(normWireLiteral("changeGroundInvestigationPositionX")), id: normWireRequired(normWireString), newPositionX: normWireRequired(normWireNumber) });
