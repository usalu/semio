/** 🧮️ `change-uls-equation` wire twin: the leaf payload `ChangeUlsEquation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type UlsEquation, parseUlsEquation } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeUlsEquation {
  newValue: UlsEquation;
}

export const parseChangeUlsEquation: NormWireReader<ChangeUlsEquation> = normWireObject<ChangeUlsEquation>({ newValue: normWireRequired(parseUlsEquation) });
