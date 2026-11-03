/** 🥞️ `change-layer-gamma-prime` wire twin: the leaf payload `ChangeLayerGammaPrime`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerGammaPrime {
  mutation: "changeLayerGammaPrime";
  id: string;
  newGammaPrime: number;
}

export const parseChangeLayerGammaPrime: NormWireReader<ChangeLayerGammaPrime> = normWireObject<ChangeLayerGammaPrime>({ mutation: normWireRequired(normWireLiteral("changeLayerGammaPrime")), id: normWireRequired(normWireString), newGammaPrime: normWireRequired(normWireNumber) });
