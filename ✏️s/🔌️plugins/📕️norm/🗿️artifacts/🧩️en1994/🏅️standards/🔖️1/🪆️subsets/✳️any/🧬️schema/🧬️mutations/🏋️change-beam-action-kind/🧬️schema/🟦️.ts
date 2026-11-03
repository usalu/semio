/** 🏋️ `change-beam-action-kind` wire twin: the leaf payload `ChangeBeamActionKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBeamActionKind {
  beamId: string;
  id: string;
  newKind: ActionKind;
}

export const parseChangeBeamActionKind: NormWireReader<ChangeBeamActionKind> = normWireObject<ChangeBeamActionKind>({ beamId: normWireRequired(normWireString), id: normWireRequired(normWireString), newKind: normWireRequired(parseActionKind) });
