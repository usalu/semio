// #region 🔖Header
// [👤semio📚engine💻mcpapp](repo://p/u/semio/b/l/engine/f/mcp-app.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite entry point that mounts the McpDesignViewer React component from @semio/ui.
// Summary: MCP App entry point for the semio engine design viewer.

// #endregion 🔖Header

import { McpDesignViewer } from "@semio/ui";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <McpDesignViewer />
  </StrictMode>,
);
