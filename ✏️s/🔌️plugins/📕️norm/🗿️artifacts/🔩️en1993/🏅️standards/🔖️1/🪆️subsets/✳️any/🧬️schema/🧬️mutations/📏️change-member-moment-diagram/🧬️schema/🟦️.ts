/** 📏️ `change-member-moment-diagram` wire twin: the leaf payload `ChangeMemberMomentDiagram`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MomentDiagram, parseMomentDiagram } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberMomentDiagram {
  id: string;
  newMomentDiagram: MomentDiagram;
}

export const parseChangeMemberMomentDiagram: NormWireReader<ChangeMemberMomentDiagram> = normWireObject<ChangeMemberMomentDiagram>({ id: normWireRequired(normWireString), newMomentDiagram: normWireRequired(parseMomentDiagram) });
