/** 🏔️ `change-north-german-lowland-snow` wire twin: the leaf payload `ChangeNorthGermanLowlandSnow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeNorthGermanLowlandSnow {
  newNorthGermanLowlandSnow: boolean;
}

export const parseChangeNorthGermanLowlandSnow: NormWireReader<ChangeNorthGermanLowlandSnow> = normWireObject<ChangeNorthGermanLowlandSnow>({ newNorthGermanLowlandSnow: normWireRequired(normWireBoolean) });
