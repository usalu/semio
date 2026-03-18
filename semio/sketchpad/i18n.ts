// #region 🔖Header
// [👤semio📚js💻i18n](semiorepo://p/u/semio/b/l/js/f/i18n.ts)

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
// [👤semio📚js💻i18n🔖i18n](semiorepo://p/u/semio/b/l/js/f/i18n.ts/s/I18n)
// Initializes i18next with language detection, React bindings and expertise-aware label hooks.
// MUST fall back to English when the detected language is unavailable.

import { i18next as i18n, LanguageDetector, initReactI18next } from "../../semio-elements/ui";
import de from "./sketchpad/locales/de.json?raw";
import en from "./sketchpad/locales/en.json?raw";

// Re-export generic i18n primitives from @semio-elements/ui
export { Expertise, setExpertiseProvider, useLabel, useTranslatedHotkey as useHotkey } from "../../semio-elements/ui";

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      en: { translation: JSON.parse(en) },
      de: { translation: JSON.parse(de) },
    },
    fallbackLng: "en",
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

export default i18n;

// #endregion 🔖I18n
