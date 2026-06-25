// #region 🧲Header

// 2026 Ueli Saluz <ueli@compose-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite build configuration for the Electron renderer process.
// Pre-bundles all dependencies from elements.tsx to prevent module waterfall
// stalls in Electron's HTTP/1.1 connection-limited renderer.

// #endregion 🧲Header

// #region 🗄️Configuration
// Vite configuration for the Electron renderer process with React and Tailwind.
// Configuration MUST enable the React and Tailwind CSS plugins.
// Configuration MUST pre-bundle heavy dependencies to avoid white screen in Electron.

// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "path";
import type { UserConfig } from "vite";
import { semioFaviconVitePlugin } from "../../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const repoRoot = path.resolve(__dirname, "../../../..");
const configuration: UserConfig = {
  server: {
    // Pre-transform the renderer entry so all modules are cached before
    // Electron requests them. Without this, cold starts produce a white screen
    // because Electron's Chromium has a 6-connection-per-origin HTTP/1.1 limit
    // and the 80+ ESM module waterfall from elements.tsx stalls.
    warmup: {
      clientFiles: ["./renderer.tsx", "../../../../ui/react/index.tsx"],
    },
    // Allow serving files from the entire monorepo since elements.tsx and
    // other dependencies live outside the desktop project root.
    fs: {
      allow: [path.resolve(__dirname, "../../../..")],
    },
  },
  optimizeDeps: {
    // Force Vite to skip runtime dep discovery. The dep scanner races with
    // electron-forge's server lifecycle causing "server is being restarted"
    // errors that skip pre-bundling. All deps are listed in include[].
    noDiscovery: true,
    // Pre-bundle all third-party dependencies imported by elements.tsx into
    // single chunks. This reduces the number of HTTP requests from 80+ to ~35
    // pre-bundled files, preventing connection pool exhaustion in Electron.
    // List ALL third-party dependencies to prevent Vite from discovering new
    // deps at runtime, which triggers re-optimization and invalidates the
    // module graph mid-load — causing the white screen in Electron.
    include: [
      "@semio-tech/framework-core",
      "@semio-tech/framework-platform-core",
      "@semio-tech/framework-platform-renderer-react",
      "react",
      "react-dom",
      "react-dom/client",
      "react/jsx-runtime",
      "react/jsx-dev-runtime",
      "@dnd-kit/core",
      "@dnd-kit/sortable",
      "@dnd-kit/utilities",
      "@mdx-js/react",
      "@radix-ui/react-accordion",
      "@radix-ui/react-avatar",
      "@radix-ui/react-collapsible",
      "@radix-ui/react-dialog",
      "@radix-ui/react-dropdown-menu",
      "@radix-ui/react-hover-card",
      "@radix-ui/react-popover",
      "@radix-ui/react-scroll-area",
      "@radix-ui/react-select",
      "@radix-ui/react-slider",
      "@radix-ui/react-slot",
      "@radix-ui/react-tabs",
      "@radix-ui/react-toggle",
      "@radix-ui/react-toggle-group",
      "@radix-ui/react-tooltip",
      "@react-three/drei",
      "@react-three/fiber",
      "@xstate/react",
      "@xyflow/react",
      "class-variance-authority",
      "clsx",
      "cmdk",
      "d3-force",
      "dagre",
      "date-fns",
      "date-fns/locale",
      "fuse.js",
      "i18next",
      "i18next-browser-languagedetector",
      "lucide-react",
      "react-hotkeys-hook",
      "react-i18next",
      "react-resizable-panels",
      "react-router",
      "tailwind-merge",
      "three",
      "three/addons/loaders/OBJLoader.js",
      "xstate"
    ],
  },
  resolve: {
    alias: {
      "@semio-tech/compose-js": path.resolve(__dirname, "../../../../compose/client/lib/js"),
      "@semio-tech/semio-asset": path.resolve(__dirname, "../../../../asset"),
      "@semio-tech/ui-react": path.resolve(__dirname, "../../../../ui/react"),
      "@semio-tech/ui-styling": path.resolve(__dirname, "../../../../ui/styling/js/index.ts"),
      "@semio-tech/framework-core": path.resolve(__dirname, "../../../../framework/core/index.ts"),
      "@semio-tech/framework-platform-core": path.resolve(__dirname, "../../../../framework/product/platform/core/index.ts"),
      "@semio-tech/framework-platform-renderer-react": path.resolve(__dirname, "../../../../framework/product/platform/renderer/react/index.tsx"),
      "@semio-tech/framework-playground-core": path.resolve(__dirname, "../../../../framework/product/playground/core/index.ts"),
      "@semio-tech/framework-playground-renderer-react": path.resolve(__dirname, "../../../../framework/product/playground/renderer/react/index.tsx"),
      "@semio-tech/puzzle-2d-react": path.resolve(__dirname, "../../../../puzzle/2d/react/index.tsx"),
      "@semio-tech/puzzle-3d-react": path.resolve(__dirname, "../../../../puzzle/3d/react/index.tsx"),
      "@semio-tech/puzzle-5d-react": path.resolve(__dirname, "../../../../puzzle/5d/react/index.tsx"),
      "@semio-tech/infinite-world-r3f": path.resolve(__dirname, "../../../../infinite/world/r3f/index.tsx"),
      "@semio-tech/coda-desktop": path.resolve(__dirname, "."),
    },
  },
  plugins: [
    ...semioFaviconVitePlugin(repoRoot),
    ...(tailwindcss() as unknown as NonNullable<UserConfig["plugins"]>),
    react() as unknown as NonNullable<UserConfig["plugins"]>[number],
  ],
};

export default configuration;

// #endregion 🗄️Configuration
