// #region 🔖Header

// [💻semio/js/i18n.ts](semiorepo://file/semio/js/i18n.ts)

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
// Initializes i18next with language detection, React bindings and expertise-aware label hooks.
// MUST fall back to English when the detected language is unavailable.

import i18n from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import { initReactI18next, useTranslation as useI18nTranslation } from "react-i18next";
import de from "./sketchpad/locales/de.json?raw";
import en from "./sketchpad/locales/en.json?raw";

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

// Expertise levels for label resolution.
// MUST map to the key structure in locale JSON files.
export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

// Mutable holder for the active expertise provider function.
// MUST be set via setExpertiseProvider before expertise-dependent labels are resolved.
let getExpertiseFunction: (() => Expertise) | undefined;

// Registers a function that returns the current expertise level.
// MUST be called once during app initialization.
export function setExpertiseProvider(fn: () => Expertise) {
  getExpertiseFunction = fn;
}

// React hook that resolves a localized label by i18n key and expertise level.
// MUST fall back to normal expertise when no provider is set.
export function useLabel(id: string): string | undefined {
  const { t } = useI18nTranslation();
  const expertise = getExpertiseFunction ? getExpertiseFunction() : Expertise.NORMAL;
  const value = t(id as any) as any;

  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object" && "label" in value) {
    const label = value.label;

    if (typeof label === "string") {
      return label;
    }

    if (label && typeof label === "object") {
      if (expertise === Expertise.BEGINNER && "beginner" in label && label.beginner !== undefined) {
        return String(label.beginner);
      }
      if ("normal" in label && label.normal !== undefined) {
        return String(label.normal);
      }
      if ("beginner" in label && label.beginner !== undefined) {
        return String(label.beginner);
      }
    }
  }

  return undefined;
}

// React hook that resolves a hotkey string by i18n key.
// MUST return undefined when no hotkey is configured.
export function useHotkey(id: string): string | undefined {
  const { t } = useI18nTranslation();
  const value = t(id as any) as any;

  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object" && value.hotkey) {
    return typeof value.hotkey === "string" ? value.hotkey : undefined;
  }

  const hotkeyKey = `${id}.hotkey`;
  const hotkeyValue = t(hotkeyKey as any) as any;
  if (typeof hotkeyValue === "string") {
    return hotkeyValue;
  }
  if (hotkeyValue && typeof hotkeyValue === "object" && hotkeyValue.hotkey) {
    return typeof hotkeyValue.hotkey === "string" ? hotkeyValue.hotkey : undefined;
  }

  return undefined;
}

export default i18n;

// #endregion 🔖I18n
