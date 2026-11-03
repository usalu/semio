/** 🧱️ `change-panel-steel-subgrade` wire twin: the leaf payload `ChangePanelSteelSubgrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePanelSteelSubgrade {
  id: string;
  newSteelSubgrade: string;
}

export const parseChangePanelSteelSubgrade: NormWireReader<ChangePanelSteelSubgrade> = normWireObject<ChangePanelSteelSubgrade>({ id: normWireRequired(normWireString), newSteelSubgrade: normWireRequired(normWireString) });
