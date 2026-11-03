/** 🥞️ `change-layer-cpt-nkt` wire twin: the leaf payload `ChangeLayerCptNkt`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerCptNkt {
  mutation: "changeLayerCptNkt";
  id: string;
  newCptNkt: number;
}

export const parseChangeLayerCptNkt: NormWireReader<ChangeLayerCptNkt> = normWireObject<ChangeLayerCptNkt>({ mutation: normWireRequired(normWireLiteral("changeLayerCptNkt")), id: normWireRequired(normWireString), newCptNkt: normWireRequired(normWireNumber) });
