/** ✖️ `change-bracing-kind` wire twin: the leaf payload `ChangeBracingKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BracingKind, parseBracingKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBracingKind {
  bracingId: string;
  newValue: BracingKind;
}

export const parseChangeBracingKind: NormWireReader<ChangeBracingKind> = normWireObject<ChangeBracingKind>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(parseBracingKind) });
