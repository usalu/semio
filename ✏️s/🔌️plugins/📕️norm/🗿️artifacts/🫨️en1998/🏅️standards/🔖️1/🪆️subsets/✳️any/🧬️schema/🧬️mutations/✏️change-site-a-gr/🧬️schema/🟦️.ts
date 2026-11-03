/** ✏️ `change-site-a-gr` wire twin: the leaf payload `ChangeSiteAGr`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteAGr {
  mutation: "changeSiteAGr";
  newAGr: number;
}

export const parseChangeSiteAGr: NormWireReader<ChangeSiteAGr> = normWireObject<ChangeSiteAGr>({ mutation: normWireRequired(normWireLiteral("changeSiteAGr")), newAGr: normWireRequired(normWireNumber) });
