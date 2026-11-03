/** 🧱️ `change-plated-web-height` wire twin: the leaf payload `ChangePlatedWebHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedWebHeight {
  id: string;
  newWebHeight: number;
}

export const parseChangePlatedWebHeight: NormWireReader<ChangePlatedWebHeight> = normWireObject<ChangePlatedWebHeight>({ id: normWireRequired(normWireString), newWebHeight: normWireRequired(normWireNumber) });
