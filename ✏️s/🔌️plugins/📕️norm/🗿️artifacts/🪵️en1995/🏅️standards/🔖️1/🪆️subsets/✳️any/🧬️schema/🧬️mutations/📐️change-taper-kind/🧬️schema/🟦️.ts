/** 📐️ `change-taper-kind` wire twin: the leaf payload `ChangeTaperKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TaperKind, parseTaperKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTaperKind {
  taperId: string;
  newValue: TaperKind;
}

export const parseChangeTaperKind: NormWireReader<ChangeTaperKind> = normWireObject<ChangeTaperKind>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(parseTaperKind) });
