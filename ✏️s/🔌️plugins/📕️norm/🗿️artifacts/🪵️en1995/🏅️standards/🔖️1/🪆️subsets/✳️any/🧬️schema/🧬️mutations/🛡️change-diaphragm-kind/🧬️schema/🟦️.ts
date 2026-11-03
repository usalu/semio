/** 🛡️ `change-diaphragm-kind` wire twin: the leaf payload `ChangeDiaphragmKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DiaphragmKind, parseDiaphragmKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDiaphragmKind {
  diaphragmId: string;
  newValue: DiaphragmKind;
}

export const parseChangeDiaphragmKind: NormWireReader<ChangeDiaphragmKind> = normWireObject<ChangeDiaphragmKind>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(parseDiaphragmKind) });
