/** 🕳️ `change-ground-investigation-kind` wire twin: the leaf payload `ChangeGroundInvestigationKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeGroundInvestigationKind {
  mutation: "changeGroundInvestigationKind";
  id: string;
  newKind: "borehole" | "cpt" | "spt";
}

export const parseChangeGroundInvestigationKind: NormWireReader<ChangeGroundInvestigationKind> = normWireObject<ChangeGroundInvestigationKind>({ mutation: normWireRequired(normWireLiteral("changeGroundInvestigationKind")), id: normWireRequired(normWireString), newKind: normWireRequired(normWireLiteral("borehole", "cpt", "spt")) });
