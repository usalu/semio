/** 📏️ `change-member-warping-support` wire twin: the leaf payload `ChangeMemberWarpingSupport`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WarpingSupport, parseWarpingSupport } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberWarpingSupport {
  id: string;
  newWarpingSupport: WarpingSupport;
}

export const parseChangeMemberWarpingSupport: NormWireReader<ChangeMemberWarpingSupport> = normWireObject<ChangeMemberWarpingSupport>({ id: normWireRequired(normWireString), newWarpingSupport: normWireRequired(parseWarpingSupport) });
