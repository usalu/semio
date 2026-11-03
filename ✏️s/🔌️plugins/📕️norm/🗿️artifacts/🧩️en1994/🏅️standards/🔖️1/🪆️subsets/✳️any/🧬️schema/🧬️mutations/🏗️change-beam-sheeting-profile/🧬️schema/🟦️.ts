/** 🏗️ `change-beam-sheeting-profile` wire twin: the leaf payload `ChangeBeamSheetingProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SheetingProfile, parseSheetingProfile } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBeamSheetingProfile {
  id: string;
  newProfile: SheetingProfile;
}

export const parseChangeBeamSheetingProfile: NormWireReader<ChangeBeamSheetingProfile> = normWireObject<ChangeBeamSheetingProfile>({ id: normWireRequired(normWireString), newProfile: normWireRequired(parseSheetingProfile) });
