/** 🚿 `change-member-tightness` wire twin: the leaf payload `ChangeMemberTightness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TightnessClass, parseTightnessClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberTightness {
  memberId: string;
  newTightness: TightnessClass;
}

export const parseChangeMemberTightness: NormWireReader<ChangeMemberTightness> = normWireObject<ChangeMemberTightness>({ memberId: normWireRequired(normWireString), newTightness: normWireRequired(parseTightnessClass) });
