/** 💦 `change-vent-humidification-required` wire twin: the leaf payload `ChangeVentHumidificationRequired`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentHumidificationRequired {
  ventId: string;
  newHumidificationRequiredKgH: number;
}

export const parseChangeVentHumidificationRequired: NormWireReader<ChangeVentHumidificationRequired> = normWireObject<ChangeVentHumidificationRequired>({ ventId: normWireRequired(normWireString), newHumidificationRequiredKgH: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
