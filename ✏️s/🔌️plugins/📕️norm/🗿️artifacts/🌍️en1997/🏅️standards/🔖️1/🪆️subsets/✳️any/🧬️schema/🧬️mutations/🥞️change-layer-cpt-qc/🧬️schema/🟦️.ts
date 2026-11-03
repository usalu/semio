/** 🥞️ `change-layer-cpt-qc` wire twin: the leaf payload `ChangeLayerCptQc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerCptQc {
  mutation: "changeLayerCptQc";
  id: string;
  newCptQc: number;
}

export const parseChangeLayerCptQc: NormWireReader<ChangeLayerCptQc> = normWireObject<ChangeLayerCptQc>({ mutation: normWireRequired(normWireLiteral("changeLayerCptQc")), id: normWireRequired(normWireString), newCptQc: normWireRequired(normWireNumber) });
