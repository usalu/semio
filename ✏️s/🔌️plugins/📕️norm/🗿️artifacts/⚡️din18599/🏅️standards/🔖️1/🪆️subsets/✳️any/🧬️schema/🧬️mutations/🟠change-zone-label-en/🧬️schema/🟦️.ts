/** 🟠 `change-zone-label-en` wire twin: the leaf payload `ChangeZoneLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneLabelEn {
  mutation: "changezonelabelen";
  zoneId: string;
  newLabelEn: string;
}

export const parseChangeZoneLabelEn: NormWireReader<ChangeZoneLabelEn> = normWireObject<ChangeZoneLabelEn>({ mutation: normWireRequired(normWireLiteral("changezonelabelen")), zoneId: normWireRequired(normWireString), newLabelEn: normWireRequired(normWireString) });
