// #region 🔖Header

// 💻 .elements/ui/.storybook/preview.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header
import type { Preview } from "@storybook/react-vite";
import { withLevel } from "./withLevel";
import { withTheme } from "./withTheme";

import "../globals.css";

enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

enum Level {
  BASE = "base",
  WINDOW = "window",
  PANEL = "panel",
  OVERLAY = "overlay",
  TEMPORARY = "temporary",
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
    level: {
      description: "UI level for components",
      toolbar: {
        title: "Level",
        icon: "component",
        items: [
          { value: Level.BASE, title: "Base" },
          { value: Level.WINDOW, title: "Window" },
          { value: Level.PANEL, title: "Panel" },
          { value: Level.OVERLAY, title: "Overlay" },
          { value: Level.TEMPORARY, title: "Temporary" },
        ],
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: {
    theme: Theme.SYSTEM,
    level: Level.BASE,
  },
  decorators: [withLevel, withTheme],
  tags: ["autodocs"],
};

export default preview;
