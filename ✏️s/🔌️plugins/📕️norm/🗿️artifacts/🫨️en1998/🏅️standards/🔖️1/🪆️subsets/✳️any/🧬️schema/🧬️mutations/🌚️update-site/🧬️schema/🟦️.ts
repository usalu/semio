/** 🌚️ `update-site` wire twin: the leaf payload `UpdateSite`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Site, parseEn1998Site } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateSite {
  mutation: "updateSite";
  site: En1998Site;
}

export const parseUpdateSite: NormWireReader<UpdateSite> = normWireObject<UpdateSite>({ mutation: normWireRequired(normWireLiteral("updateSite")), site: normWireRequired(parseEn1998Site) });
