/** ✏️ `change-site-alluvium-vsms` wire twin: the leaf payload `ChangeSiteAlluviumVSMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteAlluviumVSMS {
  mutation: "changeSiteAlluviumVSMS";
  newAlluviumVSMS: number;
}

export const parseChangeSiteAlluviumVSMS: NormWireReader<ChangeSiteAlluviumVSMS> = normWireObject<ChangeSiteAlluviumVSMS>({ mutation: normWireRequired(normWireLiteral("changeSiteAlluviumVSMS")), newAlluviumVSMS: normWireRequired(normWireNumber) });
