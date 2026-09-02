/** ✒️ Writer direct-mutation discriminated union. */
import type { ChangeLanguage } from "./🌐change-language/🟦️.ts";
import type { ChangeUri } from "./🔗change-uri/🟦️.ts";
import type { EditText } from "./✏️edit-text/🟦️.ts";
import type { RenameWriter } from "./🏷️rename-writer/🟦️.ts";

export type WriterMutation =
  | ({ mutation: "renameWriter" } & RenameWriter)
  | ({ mutation: "changeUri" } & ChangeUri)
  | ({ mutation: "changeLanguage" } & ChangeLanguage)
  | ({ mutation: "editText" } & EditText);
