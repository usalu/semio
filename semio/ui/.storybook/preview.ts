// #region 🧲Header
// 💻 semio/ui/.storybook/preview.ts
// Specs: Match elements/ui Storybook globals and decorators.
// Summary: Defines global Storybook preview parameters for semio ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Preview } from "@storybook/react-vite";
import { withLevel } from "./withLevel";
import { withTheme } from "./withTheme";

import "../globals.css";

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
  },
  initialGlobals: {
    theme: Theme.SYSTEM,
  },
  decorators: [withLevel, withTheme],
  tags: ["autodocs"],
};

export default preview;
