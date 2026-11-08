/** 🟡 `change-zone-label-de` wire twin: the leaf payload `ChangeZoneLabelDe`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneLabelDe {
  mutation: "changezonelabelde";
  zoneId: string;
  newLabelDe: string;
}

export const parseChangeZoneLabelDe: NormWireReader<ChangeZoneLabelDe> = normWireObject<ChangeZoneLabelDe>({ mutation: normWireRequired(normWireLiteral("changezonelabelde")), zoneId: normWireRequired(normWireString), newLabelDe: normWireRequired(normWireString) });
