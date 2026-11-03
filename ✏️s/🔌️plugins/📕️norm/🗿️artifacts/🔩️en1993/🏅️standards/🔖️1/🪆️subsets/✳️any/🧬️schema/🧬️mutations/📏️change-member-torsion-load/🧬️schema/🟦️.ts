/** 📏️ `change-member-torsion-load` wire twin: the leaf payload `ChangeMemberTorsionLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TorsionLoad, parseTorsionLoad } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberTorsionLoad {
  id: string;
  newTorsionLoad: TorsionLoad;
}

export const parseChangeMemberTorsionLoad: NormWireReader<ChangeMemberTorsionLoad> = normWireObject<ChangeMemberTorsionLoad>({ id: normWireRequired(normWireString), newTorsionLoad: normWireRequired(parseTorsionLoad) });
