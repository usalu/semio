export const useTranslation = () => ({ t: (k) => k, i18n: { language: "en", changeLanguage: () => {} } });
export const initReactI18next = { type: "3rdParty", init: () => {} };
export const I18nextProvider = ({ children }) => children;
