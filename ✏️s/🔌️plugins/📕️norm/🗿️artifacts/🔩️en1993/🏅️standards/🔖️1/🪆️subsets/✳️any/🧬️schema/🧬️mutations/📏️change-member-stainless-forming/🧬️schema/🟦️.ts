/** 📏️ `change-member-stainless-forming` wire twin: the leaf payload `ChangeMemberStainlessForming`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StainlessForming, parseStainlessForming } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberStainlessForming {
  id: string;
  newStainlessForming: StainlessForming;
}

export const parseChangeMemberStainlessForming: NormWireReader<ChangeMemberStainlessForming> = normWireObject<ChangeMemberStainlessForming>({ id: normWireRequired(normWireString), newStainlessForming: normWireRequired(parseStainlessForming) });
