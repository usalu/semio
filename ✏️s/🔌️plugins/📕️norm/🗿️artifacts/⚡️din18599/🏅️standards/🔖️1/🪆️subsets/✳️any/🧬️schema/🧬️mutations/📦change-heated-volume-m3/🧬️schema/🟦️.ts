/** 📦 `change-heated-volume-m3` wire twin: the leaf payload `ChangeHeatedVolumeM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeHeatedVolumeM3 {
  mutation: "changeHeatedVolumeM3";
  newHeatedVolumeM3: number;
}

export const parseChangeHeatedVolumeM3: NormWireReader<ChangeHeatedVolumeM3> = normWireObject<ChangeHeatedVolumeM3>({ mutation: normWireRequired(normWireLiteral("changeHeatedVolumeM3")), newHeatedVolumeM3: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
