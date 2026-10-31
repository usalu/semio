// #region 🧲️Header
/** ⚛️ Registers the demonstrator page's texts with the UI translation port, English AND German (no default language). */
// #endregion 🧲️Header

import { registerUiTranslationBundles } from "@semio-tech/ui-react";
import { demonstratorTranslationTree, type DemonstratorLabelKey } from "./🟦️.ts";

const registerKey = registerUiTranslationBundles({
  en: { translation: demonstratorTranslationTree("en") },
  de: { translation: demonstratorTranslationTree("de") },
});

/** 🏷️ The registered translation key of a demonstrator label. */
export const demonstratorUiLabel = (key: DemonstratorLabelKey) => registerKey(key);
