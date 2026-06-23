// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Electron Forge configuration for building and packaging the coda desktop app.

// #endregion 🧲Header

// #region 🔌Adapters
// Electron Forge build configuration for the coda desktop application.
// Configuration MUST define packager, makers, and plugins for Electron Forge.

import type { ForgeConfig } from "@electron-forge/shared-types";
import { MakerSquirrel } from "@electron-forge/maker-squirrel";
import { MakerZIP } from "@electron-forge/maker-zip";
import { MakerDeb } from "@electron-forge/maker-deb";
import { MakerRpm } from "@electron-forge/maker-rpm";
import { VitePlugin } from "@electron-forge/plugin-vite";
import { FusesPlugin } from "@electron-forge/plugin-fuses";
import { FuseV1Options, FuseVersion } from "@electron/fuses";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const assistantBundleRoot = path.resolve(__dirname, "..", "assistant");
// #endregion 🔌Adapters

// #region 🗄️Configuration
/**
 * Electron Forge configuration with Vite plugin and security fuses.
 * Config MUST include VitePlugin with main, preload, and renderer entries.
 **/
const config: ForgeConfig = {
  packagerConfig: {
    asar: true,
    executableName: "coda-desktop",
    name: "coda-desktop",
    extraResource: [assistantBundleRoot],
  },
  rebuildConfig: {},
  makers: [
    new MakerSquirrel({
      authors: "Ueli Saluz",
      name: "coda_desktop",
      setupExe: "coda-desktop-installer.exe",
    }),
    new MakerZIP({}, ["darwin"]),
    new MakerRpm({}),
    new MakerDeb({}),
  ],
  plugins: [
    new VitePlugin({
      build: [
        {
          entry: "index.ts",
          config: "vite.main.config.ts",
          target: "main",
        },
        {
          entry: "preload.ts",
          config: "vite.preload.config.ts",
          target: "preload",
        },
      ],
      renderer: [
        {
          name: "main_window",
          config: "vite.renderer.config.ts",
        },
      ],
    }),

    new FusesPlugin({
      version: FuseVersion.V1,
      [FuseV1Options.RunAsNode]: false,
      [FuseV1Options.EnableCookieEncryption]: true,
      [FuseV1Options.EnableNodeOptionsEnvironmentVariable]: false,
      [FuseV1Options.EnableNodeCliInspectArguments]: false,
      [FuseV1Options.EnableEmbeddedAsarIntegrityValidation]: true,
      [FuseV1Options.OnlyLoadAppFromAsar]: true,
    }),
  ],
};

export default config;
// #endregion 🗄️Configuration
