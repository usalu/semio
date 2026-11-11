/** 🪪 `change-vent-id` wire twin: the leaf payload `ChangeVentId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentId {
  ventId: string;
  newId: string;
}

export const parseChangeVentId: NormWireReader<ChangeVentId> = normWireObject<ChangeVentId>({ ventId: normWireRequired(normWireString), newId: normWireRequired(normWireString) });
