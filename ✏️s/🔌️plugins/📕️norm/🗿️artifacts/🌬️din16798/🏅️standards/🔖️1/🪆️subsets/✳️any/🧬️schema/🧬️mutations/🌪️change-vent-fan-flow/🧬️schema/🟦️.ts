/** 🌪️ `change-vent-fan-flow` wire twin: the leaf payload `ChangeVentFanFlow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeVentFanFlow {
  ventId: string;
  newFanQVM3S: number;
}

export const parseChangeVentFanFlow: NormWireReader<ChangeVentFanFlow> = normWireObject<ChangeVentFanFlow>({ ventId: normWireRequired(normWireString), newFanQVM3S: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
