// #region Header

// vitest.config.ts

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

/// <reference types="vitest" />
import react from "@vitejs/plugin-react";
import path from "path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [react()],
  test: {
    // Use projects instead of workspace
    projects: [
      {
        name: "unit-tests",
        environment: "node",
        globals: true,
        setupFiles: ["./vitest.setup.ts"],
        include: ["**/*.test.{js,ts,jsx,tsx}"],
        exclude: [
          "**/node_modules/**",
          "**/dist/**",
          "**/storybook-static/**",
          "**/.storybook/**",
          "**/*.stories.{js,jsx,ts,tsx}",
          "**/.{idea,git,cache,output,temp}/**",
          "**/{karma,rollup,webpack,vite,vitest,jest,ava,babel,nyc,cypress,tsup,build,storybook}.config.*",
        ],
        pool: "forks",
      },
    ],
  },
  resolve: {
    alias: {
      "@semio/js": path.resolve(__dirname, "."),
      "@semio/assets": path.resolve(__dirname, "../../assets"),
    },
  },
});
