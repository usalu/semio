/** 🕳️ `change-ground-investigation-depth` wire twin: the leaf payload `ChangeGroundInvestigationDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundInvestigationDepth {
  mutation: "changeGroundInvestigationDepth";
  id: string;
  newDepth: number;
}

export const parseChangeGroundInvestigationDepth: NormWireReader<ChangeGroundInvestigationDepth> = normWireObject<ChangeGroundInvestigationDepth>({ mutation: normWireRequired(normWireLiteral("changeGroundInvestigationDepth")), id: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });
