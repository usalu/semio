/** 📦️ `change-envelope-volume` wire twin: the leaf payload `ChangeEnvelopeVolume`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeEnvelopeVolume {
  newEnvelopeVolumeM3: number;
}

export const parseChangeEnvelopeVolume: NormWireReader<ChangeEnvelopeVolume> = normWireObject<ChangeEnvelopeVolume>({ newEnvelopeVolumeM3: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
