/** ✏️ `change-site-vs30-ms` wire twin: the leaf payload `ChangeSiteVS30MS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteVS30MS {
  mutation: "changeSiteVS30MS";
  newVS30MS: number;
}

export const parseChangeSiteVS30MS: NormWireReader<ChangeSiteVS30MS> = normWireObject<ChangeSiteVS30MS>({ mutation: normWireRequired(normWireLiteral("changeSiteVS30MS")), newVS30MS: normWireRequired(normWireNumber) });
