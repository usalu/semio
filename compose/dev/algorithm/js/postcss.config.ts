// #region 🧲Header
// 💻 compose/algorithm/postcss.config.ts
// Specs: PostCSS configuration for the algorithms bundle using @tailwindcss/postcss.
// Summary: Enables Tailwind CSS v4 processing for the algorithms storybook.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

// #region 🗄️Configuration
// PostCSS program configuration for the algorithms bundle.
// Configuration MUST use the @tailwindcss/postcss program.

import { Config } from "postcss-load-config";

const config: Config = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

export default config;

// #endregion 🗄️Configuration
