/** ✒️ Writer direct-mutation discriminated union. */
import type { ChangeLanguage } from "./🌐change-language/🟦️component.ts";
import type { ChangeUri } from "./🔗change-uri/🟦️component.ts";
import type { EditText } from "./✏️edit-text/🟦️component.ts";
import type { RenameWriter } from "./🏷️rename-writer/🟦️component.ts";

export type WriterMutation =
  | ({ mutation: "renameWriter" } & RenameWriter)
  | ({ mutation: "changeUri" } & ChangeUri)
  | ({ mutation: "changeLanguage" } & ChangeLanguage)
  | ({ mutation: "editText" } & EditText);
