/** 🕳️ `change-ground-investigation-level` wire twin: the leaf payload `ChangeGroundInvestigationLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundInvestigationLevel {
  mutation: "changeGroundInvestigationLevel";
  id: string;
  newGroundLevel: number;
}

export const parseChangeGroundInvestigationLevel: NormWireReader<ChangeGroundInvestigationLevel> = normWireObject<ChangeGroundInvestigationLevel>({ mutation: normWireRequired(normWireLiteral("changeGroundInvestigationLevel")), id: normWireRequired(normWireString), newGroundLevel: normWireRequired(normWireNumber) });
