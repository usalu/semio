/** 🔗️ `change-conn-bolts-gauge` wire twin: the leaf payload `ChangeConnBoltsGauge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsGauge {
  mutation: "changeConnBoltsGauge";
  connectionId: string;
  newBoltsGauge: number;
}

export const parseChangeConnBoltsGauge: NormWireReader<ChangeConnBoltsGauge> = normWireObject<ChangeConnBoltsGauge>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsGauge")), connectionId: normWireRequired(normWireString), newBoltsGauge: normWireRequired(normWireNumber) });
