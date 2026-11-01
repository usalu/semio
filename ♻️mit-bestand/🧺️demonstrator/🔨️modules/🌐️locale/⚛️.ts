// #region 🧲️Header
/** ⚛️ Registers the demonstrator page's texts with the UI translation port, English AND German (no default language). */
// #endregion 🧲️Header

import { createBrowserStoragePort } from "@semio-tech/framework";
import { initUiLocaleSync, readStoredUiChromeLocale, registerUiTranslationBundles } from "@semio-tech/ui-react";
import { demonstratorText, demonstratorTranslationTree, resolveDemonstratorLocale, type DemonstratorLabelKey, type DemonstratorLocale } from "./🟦️.ts";

const registerKey = registerUiTranslationBundles({
  en: { translation: demonstratorTranslationTree("en") },
  de: { translation: demonstratorTranslationTree("de") },
});

/** 🏷️ The registered translation key of a demonstrator label. */
export const demonstratorUiLabel = (key: DemonstratorLabelKey) => registerKey(key);

/** 🧭️ The viewer's language: the stored choice, else the browser's preferred languages; `undefined` until the viewer picks one. */
export const readDemonstratorLocale = (): DemonstratorLocale | undefined => resolveDemonstratorLocale([readStoredUiChromeLocale(createBrowserStoragePort()), ...(typeof navigator === "undefined" ? [] : [...navigator.languages, navigator.language])]);

/** 💾️ Commits the viewer's language: persisted, declared on the document with its title, and handed to the UI translation port before the next render. */
export function commitDemonstratorLocale(locale: DemonstratorLocale): void {
  initUiLocaleSync(locale);
  document.title = demonstratorText("demonstrator.landing.title", locale);
}
