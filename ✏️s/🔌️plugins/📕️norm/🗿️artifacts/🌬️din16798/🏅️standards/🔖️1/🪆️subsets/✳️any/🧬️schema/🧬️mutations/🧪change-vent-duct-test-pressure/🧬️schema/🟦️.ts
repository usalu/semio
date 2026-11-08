/** 🧪 `change-vent-duct-test-pressure` wire twin: the leaf payload `ChangeVentDuctTestPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentDuctTestPressure {
  ventId: string;
  newDuctTestPressurePa: number;
}

export const parseChangeVentDuctTestPressure: NormWireReader<ChangeVentDuctTestPressure> = normWireObject<ChangeVentDuctTestPressure>({ ventId: normWireRequired(normWireString), newDuctTestPressurePa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
