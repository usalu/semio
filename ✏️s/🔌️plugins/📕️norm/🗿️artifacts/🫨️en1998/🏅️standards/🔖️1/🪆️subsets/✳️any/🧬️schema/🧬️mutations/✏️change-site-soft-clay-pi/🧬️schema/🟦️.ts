/** ✏️ `change-site-soft-clay-pi` wire twin: the leaf payload `ChangeSiteSoftClayPi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteSoftClayPi {
  mutation: "changeSiteSoftClayPi";
  newSoftClayPi: number;
}

export const parseChangeSiteSoftClayPi: NormWireReader<ChangeSiteSoftClayPi> = normWireObject<ChangeSiteSoftClayPi>({ mutation: normWireRequired(normWireLiteral("changeSiteSoftClayPi")), newSoftClayPi: normWireRequired(normWireNumber) });
