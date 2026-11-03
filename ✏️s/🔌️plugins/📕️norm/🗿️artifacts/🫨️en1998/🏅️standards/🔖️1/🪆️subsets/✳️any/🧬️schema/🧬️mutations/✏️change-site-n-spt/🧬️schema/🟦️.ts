/** ✏️ `change-site-n-spt` wire twin: the leaf payload `ChangeSiteNSpt`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteNSpt {
  mutation: "changeSiteNSpt";
  newNSpt: number;
}

export const parseChangeSiteNSpt: NormWireReader<ChangeSiteNSpt> = normWireObject<ChangeSiteNSpt>({ mutation: normWireRequired(normWireLiteral("changeSiteNSpt")), newNSpt: normWireRequired(normWireNumber) });
