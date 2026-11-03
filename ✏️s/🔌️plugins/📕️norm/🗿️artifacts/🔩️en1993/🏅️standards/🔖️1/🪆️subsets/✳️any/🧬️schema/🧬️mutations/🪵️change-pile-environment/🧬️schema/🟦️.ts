/** 🪵️ `change-pile-environment` wire twin: the leaf payload `ChangePileEnvironment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SheetPileEnvironment, parseSheetPileEnvironment } from "../../../📸️snapshot/🟦️.ts";

export interface ChangePileEnvironment {
  id: string;
  newEnvironment: SheetPileEnvironment;
}

export const parseChangePileEnvironment: NormWireReader<ChangePileEnvironment> = normWireObject<ChangePileEnvironment>({ id: normWireRequired(normWireString), newEnvironment: normWireRequired(parseSheetPileEnvironment) });
