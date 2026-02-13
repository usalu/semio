// #region 🔖Header

// [⚙️semio/js/tailwind.config.ts](semiorepo://file/semio/js/tailwind.config.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Tailwind CSS configuration for the JavaScript workspace styling.

// #endregion 🔖Header

// #region 🔖Configuration

// [🔖semio/js/tailwind.config.ts#Configuration](semiorepo://section/semio/js/tailwind.config.ts/CONFIGURATION)
// Tailwind CSS configuration with typography plugin and custom prose styles.
// Configuration MUST define content paths, dark mode, and typography theme extensions.

import typography from "@tailwindcss/typography";
import type { Config } from "tailwindcss";

// Tailwind CSS configuration with prose color mappings for light and dark modes.
// Export MUST satisfy the Tailwind Config type.
export default {
  content: ["./**/*.{ts,tsx,mdx}"],
  darkMode: "media",
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            "--tw-prose-body": "var(--color-foreground)",
            "--tw-prose-headings": "var(--color-foreground)",
            "--tw-prose-lead": "var(--color-foreground)",
            "--tw-prose-links": "var(--color-active-base)",
            "--tw-prose-bold": "var(--color-foreground)",
            "--tw-prose-counters": "var(--color-foreground)",
            "--tw-prose-bullets": "var(--color-foreground)",
            "--tw-prose-hr": "var(--color-border)",
            "--tw-prose-quotes": "var(--color-foreground)",
            "--tw-prose-quote-borders": "var(--color-border)",
            "--tw-prose-captions": "var(--color-muted-foreground)",
            "--tw-prose-code": "var(--color-foreground)",
            "--tw-prose-pre-code": "var(--color-foreground)",
            "--tw-prose-pre-bg": "var(--color-temporary)",
            "--tw-prose-th-borders": "var(--color-border)",
            "--tw-prose-td-borders": "var(--color-border)",
          },
        },
        invert: {
          css: {
            "--tw-prose-body": "var(--color-foreground)",
            "--tw-prose-headings": "var(--color-foreground)",
            "--tw-prose-lead": "var(--color-foreground)",
            "--tw-prose-links": "var(--color-active-foreground)",
            "--tw-prose-bold": "var(--color-foreground)",
            "--tw-prose-counters": "var(--color-foreground)",
            "--tw-prose-bullets": "var(--color-foreground)",
            "--tw-prose-hr": "var(--color-border)",
            "--tw-prose-quotes": "var(--color-foreground)",
            "--tw-prose-quote-borders": "var(--color-border)",
            "--tw-prose-captions": "var(--color-muted-foreground)",
            "--tw-prose-code": "var(--color-foreground)",
            "--tw-prose-pre-code": "var(--color-foreground)",
            "--tw-prose-pre-bg": "var(--color-temporary)",
            "--tw-prose-th-borders": "var(--color-border)",
            "--tw-prose-td-borders": "var(--color-border)",
          },
        },
      },
    },
  },
  plugins: [typography],
} satisfies Config;

// #endregion 🔖Configuration
