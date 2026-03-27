const noop = () => {};
const i18n = { use: () => i18n, init: () => i18n, t: (k) => k, language: "en", isInitialized: true, changeLanguage: noop, hasResourceBundle: () => true, addResourceBundle: noop, resolvedLanguage: "en" };
export default i18n;
export const initReactI18next = { type: "3rdParty", init: noop };
