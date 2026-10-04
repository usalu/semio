/** ⚖️ `change-silo-kind` wire twin: the leaf payload `ChangeSiloKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloKind {
  newSiloKind: string;
}

export const parseChangeSiloKind: NormWireReader<ChangeSiloKind> = normWireObject<ChangeSiloKind>({ newSiloKind: normWireRequired(normWireString) });
