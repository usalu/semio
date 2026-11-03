/** ✏️ `change-site-en-ground-type` wire twin: the leaf payload `ChangeSiteEnGroundType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type EnGroundType, parseEnGroundType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSiteEnGroundType {
  mutation: "changeSiteEnGroundType";
  newEnGroundType: EnGroundType;
}

export const parseChangeSiteEnGroundType: NormWireReader<ChangeSiteEnGroundType> = normWireObject<ChangeSiteEnGroundType>({ mutation: normWireRequired(normWireLiteral("changeSiteEnGroundType")), newEnGroundType: normWireRequired(parseEnGroundType) });
