/** 🏗️ `change-crane-rail-contact-width` wire twin: the leaf payload `ChangeCraneRailContactWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneRailContactWidth {
  id: string;
  newRailContactWidth: number;
}

export const parseChangeCraneRailContactWidth: NormWireReader<ChangeCraneRailContactWidth> = normWireObject<ChangeCraneRailContactWidth>({ id: normWireRequired(normWireString), newRailContactWidth: normWireRequired(normWireNumber) });
