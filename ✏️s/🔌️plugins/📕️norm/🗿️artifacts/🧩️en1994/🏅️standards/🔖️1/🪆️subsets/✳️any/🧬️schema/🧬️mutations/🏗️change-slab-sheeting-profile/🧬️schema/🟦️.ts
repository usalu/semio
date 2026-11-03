/** 🏗️ `change-slab-sheeting-profile` wire twin: the leaf payload `ChangeSlabSheetingProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SheetingProfile, parseSheetingProfile } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSlabSheetingProfile {
  id: string;
  newProfile: SheetingProfile;
}

export const parseChangeSlabSheetingProfile: NormWireReader<ChangeSlabSheetingProfile> = normWireObject<ChangeSlabSheetingProfile>({ id: normWireRequired(normWireString), newProfile: normWireRequired(parseSheetingProfile) });
