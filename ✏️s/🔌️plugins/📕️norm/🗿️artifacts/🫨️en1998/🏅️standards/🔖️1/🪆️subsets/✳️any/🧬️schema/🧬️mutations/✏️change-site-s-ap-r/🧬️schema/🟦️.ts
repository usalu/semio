/** ✏️ `change-site-s-ap-r` wire twin: the leaf payload `ChangeSiteSApR`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteSApR {
  mutation: "changeSiteSApR";
  newSApR: number;
}

export const parseChangeSiteSApR: NormWireReader<ChangeSiteSApR> = normWireObject<ChangeSiteSApR>({ mutation: normWireRequired(normWireLiteral("changeSiteSApR")), newSApR: normWireRequired(normWireNumber) });
