/** 🔴 `change-zone-theta-i-heat-c` wire twin: the leaf payload `ChangeZoneThetaIHeatC`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneThetaIHeatC {
  mutation: "changezonethetaiheatc";
  zoneId: string;
  newThetaIHeatC: number;
}

export const parseChangeZoneThetaIHeatC: NormWireReader<ChangeZoneThetaIHeatC> = normWireObject<ChangeZoneThetaIHeatC>({ mutation: normWireRequired(normWireLiteral("changezonethetaiheatc")), zoneId: normWireRequired(normWireString), newThetaIHeatC: normWireRequired(normWireRange(normWireNumber, {"minimum":10,"maximum":30})) });
