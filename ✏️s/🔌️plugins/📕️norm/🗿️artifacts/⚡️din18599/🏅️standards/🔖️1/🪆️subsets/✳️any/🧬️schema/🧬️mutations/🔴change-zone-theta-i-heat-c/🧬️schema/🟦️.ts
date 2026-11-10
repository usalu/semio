/** 🔴 `change-zone-theta-i-heat-c` wire twin: the leaf payload `ChangeZoneThetaIHeatC`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneThetaIHeatC {
  mutation: "changezonethetaiheatc";
  zoneId: string;
  newThetaIHeatC: number;
}

export const parseChangeZoneThetaIHeatC: NormWireReader<ChangeZoneThetaIHeatC> = normWireObject<ChangeZoneThetaIHeatC>({ mutation: normWireRequired(normWireLiteral("changezonethetaiheatc")), zoneId: normWireRequired(normWireString), newThetaIHeatC: normWireRequired(normWireRange(normWireNumber, {"minimum":10.0,"maximum":30.0})) });
