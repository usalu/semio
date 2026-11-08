/** 🚿 `change-vent-humidification-provided` wire twin: the leaf payload `ChangeVentHumidificationProvided`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentHumidificationProvided {
  ventId: string;
  newHumidificationProvidedKgH: number;
}

export const parseChangeVentHumidificationProvided: NormWireReader<ChangeVentHumidificationProvided> = normWireObject<ChangeVentHumidificationProvided>({ ventId: normWireRequired(normWireString), newHumidificationProvidedKgH: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
