/** 🟫 `change-element-zone-id` wire twin: the leaf payload `ChangeElementZoneId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementZoneId {
  mutation: "changeelementzoneid";
  elementId: string;
  newZoneId: string;
}

export const parseChangeElementZoneId: NormWireReader<ChangeElementZoneId> = normWireObject<ChangeElementZoneId>({ mutation: normWireRequired(normWireLiteral("changeelementzoneid")), elementId: normWireRequired(normWireString), newZoneId: normWireRequired(normWireString) });
