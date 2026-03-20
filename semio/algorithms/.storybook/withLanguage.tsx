// #region 🔖Header
// 💻 semio/algorithms/.storybook/withLanguage.tsx
// Specs: Provide a global Storybook toolbar selector for algorithm implementation language.
// Summary: Wraps stories with a LanguageProvider and exposes `useAlgorithmLanguage()` for story UI.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Decorator } from "@storybook/react";
import * as React from "react";

export enum AlgorithmLanguage {
  TS = "ts",
  PYTHON = "python",
  RUST = "rust",
  GO = "go",
}

export type AlgorithmLanguageValue = (typeof AlgorithmLanguage)[keyof typeof AlgorithmLanguage];

const AlgorithmLanguageContext = React.createContext<AlgorithmLanguageValue>(AlgorithmLanguage.TS);

export function useAlgorithmLanguage(): AlgorithmLanguageValue {
  return React.useContext(AlgorithmLanguageContext);
}

export const AlgorithmLanguageProvider: React.FC<{
  language: AlgorithmLanguageValue;
  children: React.ReactNode;
}> = ({ language, children }) => {
  return <AlgorithmLanguageContext.Provider value={language}>{children}</AlgorithmLanguageContext.Provider>;
};

export const withLanguage: Decorator = (Story, context) => {
  const language = (context.globals.language as AlgorithmLanguageValue | undefined) ?? AlgorithmLanguage.TS;
  return (
    <AlgorithmLanguageProvider language={language}>
      <Story />
    </AlgorithmLanguageProvider>
  );
};
