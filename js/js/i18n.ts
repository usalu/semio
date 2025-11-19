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

export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

let getExpertiseFunction: (() => Expertise) | undefined;

export function setExpertiseProvider(fn: () => Expertise) {
  getExpertiseFunction = fn;
}

export function useLabel(id: string, defaultValue?: string): string {
  const { t } = useI18nTranslation();
  const expertise = getExpertiseFunction ? getExpertiseFunction() : Expertise.NORMAL;
  const value = t(id);

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

  // Safety: ensure we always return a string, never an object
  const fallback = defaultValue ?? id;
  return typeof fallback === "string" ? fallback : String(fallback);
}

export function useHotkey(id: string): string | undefined {
  const { t } = useI18nTranslation();
  const value = t(id);

  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object" && value.hotkey) {
    return typeof value.hotkey === "string" ? value.hotkey : undefined;
  }

  const hotkeyKey = `${id}.hotkey`;
  const hotkeyValue = t(hotkeyKey);
  if (typeof hotkeyValue === "string") {
    return hotkeyValue;
  }
  if (hotkeyValue && typeof hotkeyValue === "object" && hotkeyValue.hotkey) {
    return typeof hotkeyValue.hotkey === "string" ? hotkeyValue.hotkey : undefined;
  }

  return undefined;
}

export default i18n;
