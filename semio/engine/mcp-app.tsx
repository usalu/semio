// #region 🔖Header
// [👤semio📚engine💻mcpapp](repo://p/u/semio/b/l/engine/f/mcp-app.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: MCP App entry point. Exclusively uses @semio/ui components.
// Summary: MCP App entry point that mounts McpDesignViewer from @semio/ui.

// #endregion 🔖Header

import "@semio/ui/globals.css";
import { createRoot } from "react-dom/client";
import { mountMcpDesignViewer } from "@semio/ui";

mountMcpDesignViewer(createRoot);
