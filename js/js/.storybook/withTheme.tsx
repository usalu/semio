// #region Header

// js/js/.storybook/withTheme.tsx

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

// #endregion Header

// #region Header

// withTheme.tsx

// 2025 Ueli Saluz

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

// #endregion
import type { Decorator } from "@storybook/react";
import { useEffect } from "react";

enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

export const withTheme: Decorator = (Story, context) => {
  const theme = context.globals.theme as Theme;
  useEffect(() => {
    const root = window.document.documentElement;
    const body = window.document.body;
    root.classList.remove(Theme.DARK);
    if (theme === Theme.DARK) {
      root.classList.add(Theme.DARK);
      body.style.backgroundColor = "var(--background)";
      body.style.color = "var(--foreground)";
    } else if (theme === Theme.SYSTEM) {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      if (prefersDark) {
        root.classList.add(Theme.DARK);
        body.style.backgroundColor = "var(--background)";
        body.style.color = "var(--foreground)";
      } else {
        body.style.backgroundColor = "var(--background)";
        body.style.color = "var(--foreground)";
      }
    } else {
      body.style.backgroundColor = "var(--background)";
      body.style.color = "var(--foreground)";
    }
  }, [theme]);
  return <Story />;
};
