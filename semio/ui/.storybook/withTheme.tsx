// #region 🔖Header
// 💻 semio/ui/.storybook/withTheme.tsx
// Specs: Use same theme decorator behavior as elements/ui.
// Summary: Applies global Storybook light/dark/system theme handling.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

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
      }
      body.style.backgroundColor = "var(--background)";
      body.style.color = "var(--foreground)";
    } else {
      body.style.backgroundColor = "var(--background)";
      body.style.color = "var(--foreground)";
    }
  }, [theme]);
  return <Story />;
};
