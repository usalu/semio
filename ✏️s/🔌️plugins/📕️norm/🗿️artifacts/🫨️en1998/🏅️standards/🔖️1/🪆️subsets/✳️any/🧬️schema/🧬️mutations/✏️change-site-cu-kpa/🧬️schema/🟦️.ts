/** ✏️ `change-site-cu-kpa` wire twin: the leaf payload `ChangeSiteCUKpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiteCUKpa {
  mutation: "changeSiteCUKpa";
  newCUKpa: number;
}

export const parseChangeSiteCUKpa: NormWireReader<ChangeSiteCUKpa> = normWireObject<ChangeSiteCUKpa>({ mutation: normWireRequired(normWireLiteral("changeSiteCUKpa")), newCUKpa: normWireRequired(normWireNumber) });
