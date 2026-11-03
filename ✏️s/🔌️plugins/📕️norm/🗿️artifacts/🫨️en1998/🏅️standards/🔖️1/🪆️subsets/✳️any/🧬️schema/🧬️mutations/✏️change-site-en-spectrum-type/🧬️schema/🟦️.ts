/** ✏️ `change-site-en-spectrum-type` wire twin: the leaf payload `ChangeSiteEnSpectrumType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type EnSpectrumType, parseEnSpectrumType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSiteEnSpectrumType {
  mutation: "changeSiteEnSpectrumType";
  newEnSpectrumType: EnSpectrumType;
}

export const parseChangeSiteEnSpectrumType: NormWireReader<ChangeSiteEnSpectrumType> = normWireObject<ChangeSiteEnSpectrumType>({ mutation: normWireRequired(normWireLiteral("changeSiteEnSpectrumType")), newEnSpectrumType: normWireRequired(parseEnSpectrumType) });
