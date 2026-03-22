// #region 🔖Header
// [👤semio📚js💻i18n](repo://p/u/semio/b/l/js/f/i18n.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Internationalization setup and translation utilities for the UI.

// #endregion 🔖Header

// #region 🔖I18n
// [👤semio📚js💻i18n🔖i18n](repo://p/u/semio/b/l/js/f/i18n.ts/s/I18n)
// Initializes i18next with language detection, React bindings and expertise-aware label hooks.
// MUST fall back to English when the detected language is unavailable.

import { i18next as i18n, initReactI18next, LanguageDetector } from "@semio/ui";

// Re-export generic i18n primitives from @elements/ui
export { Expertise, setExpertiseProvider, useTranslatedHotkey as useHotkey, useLabel } from "@semio/ui";

type LocaleCode = "de" | "en";

const localeLoaders: Record<LocaleCode, () => Promise<{ default: string }>> = {
  de: () => import("./locales/de.json?raw"),
  en: () => import("./locales/en.json?raw"),
};

const loadedLocales = new Set<LocaleCode>();

function normalizeLocale(language?: string): LocaleCode {
  return language?.toLowerCase().startsWith("de") ? "de" : "en";
}

async function ensureLocaleLoaded(language: LocaleCode): Promise<void> {
  if (loadedLocales.has(language)) {
    return;
  }

  const module = await localeLoaders[language]();
  i18n.addResourceBundle(language, "translation", JSON.parse(module.default), true, true);
  loadedLocales.add(language);
}

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {},
    fallbackLng: "en",
    lng: "en",
    returnObjects: true,
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
      bindI18n: "languageChanged",
      bindI18nStore: "added removed",
    },
  });

void (async () => {
  const requestedLocale = normalizeLocale(i18n.resolvedLanguage || i18n.language || (typeof navigator !== "undefined" ? navigator.language : undefined));
  await ensureLocaleLoaded("en");
  if (requestedLocale !== "en") {
    await ensureLocaleLoaded(requestedLocale);
  }
  if (i18n.language !== requestedLocale) {
    await i18n.changeLanguage(requestedLocale);
  }
})();

export default i18n;

// #endregion 🔖I18n
