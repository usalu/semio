const no_operation = () => {};
const i18n = { use: () => i18n, init: () => i18n, t: (k) => k, language: "en", isInitialized: true, changeLanguage: no_operation, hasResourceBundle: () => true, addResourceBundle: no_operation, resolvedLanguage: "en" };
export default i18n;
export const initReactI18next = { type: "3rdParty", init: no_operation };
