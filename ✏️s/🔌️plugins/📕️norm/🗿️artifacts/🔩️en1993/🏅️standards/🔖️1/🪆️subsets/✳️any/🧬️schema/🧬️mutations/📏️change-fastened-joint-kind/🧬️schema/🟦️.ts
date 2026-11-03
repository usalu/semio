/** 📏️ `change-fastened-joint-kind` wire twin: the leaf payload `ChangeFastenedJointKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FastenerKind, parseFastenerKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFastenedJointKind {
  id: string;
  newKind: FastenerKind;
}

export const parseChangeFastenedJointKind: NormWireReader<ChangeFastenedJointKind> = normWireObject<ChangeFastenedJointKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseFastenerKind) });
