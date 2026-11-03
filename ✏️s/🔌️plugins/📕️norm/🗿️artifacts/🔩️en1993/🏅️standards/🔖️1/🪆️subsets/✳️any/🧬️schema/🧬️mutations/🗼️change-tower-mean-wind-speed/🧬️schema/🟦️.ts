/** 🗼️ `change-tower-mean-wind-speed` wire twin: the leaf payload `ChangeTowerMeanWindSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerMeanWindSpeed {
  id: string;
  newMeanWindSpeed: number;
}

export const parseChangeTowerMeanWindSpeed: NormWireReader<ChangeTowerMeanWindSpeed> = normWireObject<ChangeTowerMeanWindSpeed>({ id: normWireRequired(normWireString), newMeanWindSpeed: normWireRequired(normWireNumber) });
