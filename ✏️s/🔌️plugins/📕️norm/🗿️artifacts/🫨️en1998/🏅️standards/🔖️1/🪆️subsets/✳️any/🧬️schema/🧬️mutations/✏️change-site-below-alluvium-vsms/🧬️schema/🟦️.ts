/** ✏️ `change-site-below-alluvium-vsms` wire twin: the leaf payload `ChangeSiteBelowAlluviumVSMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteBelowAlluviumVSMS {
  mutation: "changeSiteBelowAlluviumVSMS";
  newBelowAlluviumVSMS: number;
}

export const parseChangeSiteBelowAlluviumVSMS: NormWireReader<ChangeSiteBelowAlluviumVSMS> = normWireObject<ChangeSiteBelowAlluviumVSMS>({ mutation: normWireRequired(normWireLiteral("changeSiteBelowAlluviumVSMS")), newBelowAlluviumVSMS: normWireRequired(normWireNumber) });
