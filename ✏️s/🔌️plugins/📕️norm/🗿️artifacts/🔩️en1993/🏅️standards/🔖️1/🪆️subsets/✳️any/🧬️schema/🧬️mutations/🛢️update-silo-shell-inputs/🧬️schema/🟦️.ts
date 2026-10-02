/** 🛢️ `update-silo-shell-inputs` wire twin: the leaf payload `UpdateSiloShellInputs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSiloShell, type SiloShell } from "../../../📸️snapshot/🟦️.ts";

export interface UpdateSiloShellInputs {
  siloShell: SiloShell;
}

export const parseUpdateSiloShellInputs: NormWireReader<UpdateSiloShellInputs> = normWireObject<UpdateSiloShellInputs>({ siloShell: normWireRequired(parseSiloShell) });
