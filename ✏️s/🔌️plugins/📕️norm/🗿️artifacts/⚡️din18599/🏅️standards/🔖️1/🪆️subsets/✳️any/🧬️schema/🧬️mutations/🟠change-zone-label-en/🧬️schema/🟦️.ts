/** 🟠 `change-zone-label-en` wire twin: the leaf payload `ChangeZoneLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneLabelEn {
  mutation: "changezonelabelen";
  zoneId: string;
  newLabelEn: string;
}

export const parseChangeZoneLabelEn: NormWireReader<ChangeZoneLabelEn> = normWireObject<ChangeZoneLabelEn>({ mutation: normWireRequired(normWireLiteral("changezonelabelen")), zoneId: normWireRequired(normWireString), newLabelEn: normWireRequired(normWireString) });
