// #region 🔖Header
// 💻 semio/algorithms/.storybook/preview.ts
// Specs: Match .elements/ui Storybook globals and decorators.
// Summary: Defines global Storybook preview parameters for algorithms.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Preview } from "@storybook/react-vite";
import { withLevel } from "./withLevel";
import { withTheme } from "./withTheme";
import { AlgorithmLanguage, withLanguage } from "./withLanguage";

import "../../ui/globals.css";

enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
  globalTypes: {
    theme: {
      description: "Global theme for components",
      toolbar: {
        title: "Theme",
        icon: "circlehollow",
        items: [
          { value: Theme.SYSTEM, title: "System", icon: "browser" },
          { value: Theme.LIGHT, title: "Light", icon: "sun" },
          { value: Theme.DARK, title: "Dark", icon: "moon" },
        ],
        dynamicTitle: true,
      },
    },
    language: {
      description: "Global implementation language for algorithm execution",
      toolbar: {
        title: "Language",
        icon: "circlehollow",
        items: [
          { value: AlgorithmLanguage.TS, title: "TypeScript", icon: "code" },
          { value: AlgorithmLanguage.PYTHON, title: "Python", icon: "python" },
          { value: AlgorithmLanguage.RUST, title: "Rust", icon: "code-fork" },
          { value: AlgorithmLanguage.GO, title: "Go", icon: "terminal" },
        ],
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: {
    theme: Theme.SYSTEM,
    language: AlgorithmLanguage.TS,
  },
  decorators: [withLevel, withLanguage, withTheme],
  tags: ["autodocs"],
};

export default preview;
