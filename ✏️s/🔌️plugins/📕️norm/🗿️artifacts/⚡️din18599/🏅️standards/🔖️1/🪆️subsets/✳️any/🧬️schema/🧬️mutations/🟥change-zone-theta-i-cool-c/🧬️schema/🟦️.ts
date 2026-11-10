/** 🟥 `change-zone-theta-i-cool-c` wire twin: the leaf payload `ChangeZoneThetaICoolC`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeZoneThetaICoolC {
  mutation: "changezonethetaicoolc";
  zoneId: string;
  newThetaICoolC: number;
}

export const parseChangeZoneThetaICoolC: NormWireReader<ChangeZoneThetaICoolC> = normWireObject<ChangeZoneThetaICoolC>({ mutation: normWireRequired(normWireLiteral("changezonethetaicoolc")), zoneId: normWireRequired(normWireString), newThetaICoolC: normWireRequired(normWireRange(normWireNumber, {"minimum":18.0,"maximum":35.0})) });
