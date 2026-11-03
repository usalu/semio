/** 🧽 `clear-member-tightness` wire twin: the leaf payload `ClearMemberTightness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ClearMemberTightness {
  memberId: string;
}

export const parseClearMemberTightness: NormWireReader<ClearMemberTightness> = normWireObject<ClearMemberTightness>({ memberId: normWireRequired(normWireString) });
