/** 🥞️ `change-layer-spt-n` wire twin: the leaf payload `ChangeLayerSptN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLayerSptN {
  mutation: "changeLayerSptN";
  id: string;
  newSptN: number;
}

export const parseChangeLayerSptN: NormWireReader<ChangeLayerSptN> = normWireObject<ChangeLayerSptN>({ mutation: normWireRequired(normWireLiteral("changeLayerSptN")), id: normWireRequired(normWireString), newSptN: normWireRequired(normWireNumber) });
