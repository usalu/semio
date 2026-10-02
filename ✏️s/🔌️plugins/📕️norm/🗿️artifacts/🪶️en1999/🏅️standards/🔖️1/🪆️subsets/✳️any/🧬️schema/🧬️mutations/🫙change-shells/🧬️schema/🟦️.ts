/** 🫙 `change-shells` wire twin: the leaf payload `ChangeShells`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumShell, parseAluminiumShell } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeShells {
  mutation: "changeShells";
  shells: AluminiumShell[];
}

export const parseChangeShells: NormWireReader<ChangeShells> = normWireObject<ChangeShells>({ mutation: normWireRequired(normWireLiteral("changeShells")), shells: normWireRequired(normWireArray(parseAluminiumShell)) });
