/** ⚗️ `change-material-designation` wire twin: the leaf payload `ChangeMaterialDesignation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMaterialDesignation {
  mutation: "changeMaterialDesignation";
  materialId: string;
  newDesignation: string;
}

export const parseChangeMaterialDesignation: NormWireReader<ChangeMaterialDesignation> = normWireObject<ChangeMaterialDesignation>({ mutation: normWireRequired(normWireLiteral("changeMaterialDesignation")), materialId: normWireRequired(normWireString), newDesignation: normWireRequired(normWireString) });
