/** 🎯 `change-reliability-class` wire twin: the leaf payload `ChangeReliabilityClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReliabilityClass {
  newReliabilityClass: "RC1" | "RC2" | "RC3";
}

export const parseChangeReliabilityClass: NormWireReader<ChangeReliabilityClass> = normWireObject<ChangeReliabilityClass>({ newReliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")) });
