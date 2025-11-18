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
  const { t } = useTranslation();
  const expertise = getExpertiseFunction ? getExpertiseFunction() : Expertise.NORMAL;
  const value = t(id);

  console.log(`[DEBUG] [BREADCRUMB-RENDER] useLabel called with id="${id}"`, { value, expertise, valueType: typeof value, hasLabel: value && typeof value === "object" && "label" in value });

  if (typeof value === "string") {
    console.log(`[DEBUG] [BREADCRUMB-RENDER] Returning string value:`, value);
    return value;
  }

  if (value && typeof value === "object" && value.label) {
    console.log(`[DEBUG] [BREADCRUMB-RENDER] value.label exists:`, { label: value.label, labelType: typeof value.label });
    if (typeof value.label === "string") {
      console.log(`[DEBUG] [BREADCRUMB-RENDER] Returning value.label (string):`, value.label);
      return value.label;
    }
    if (value.label && typeof value.label === "object") {
      console.log(`[DEBUG] [BREADCRUMB-RENDER] value.label is object:`, value.label);
      if (expertise === Expertise.BEGINNER && value.label.beginner !== undefined) {
        const result = String(value.label.beginner);
        console.log(`[DEBUG] [BREADCRUMB-RENDER] Returning beginner label:`, result);
        return result;
      }
      if (value.label.normal !== undefined) {
        const result = String(value.label.normal);
        console.log(`[DEBUG] [BREADCRUMB-RENDER] Returning normal label:`, result);
        return result;
      }
      if (value.label.beginner !== undefined) {
        const result = String(value.label.beginner);
        console.log(`[DEBUG] [BREADCRUMB-RENDER] Returning beginner label (fallback):`, result);
        return result;
      }
    }
  }

  console.log(`[DEBUG] [BREADCRUMB-RENDER] Returning default/id:`, defaultValue ?? id);
  return defaultValue ?? id;
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
