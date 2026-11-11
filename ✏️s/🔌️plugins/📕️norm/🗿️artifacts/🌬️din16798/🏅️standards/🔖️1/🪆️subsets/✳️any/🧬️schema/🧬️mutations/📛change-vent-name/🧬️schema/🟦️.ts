/** 📛 `change-vent-name` wire twin: the leaf payload `ChangeVentName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentName {
  ventId: string;
  newName: string;
}

export const parseChangeVentName: NormWireReader<ChangeVentName> = normWireObject<ChangeVentName>({ ventId: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
