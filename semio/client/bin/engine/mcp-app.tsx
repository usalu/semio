// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: MCP App entry point. Mounts the @semio/sketchpad viewer named by #root data-mcp-viewer (kit, design, scene, diagram).
// Summary: MCP App entry mounting the sketchpad MCP viewers.

// #endregion 🧲Header

import { mountMcpDesignViewer, mountMcpDiagramViewer, mountMcpKitViewer, mountMcpSceneViewer } from "@semio/sketchpad/boot";

const root = document.getElementById("root");
if (!root) throw new Error("Missing #root element");
const mode = root.getAttribute("data-mcp-viewer") ?? "design";
try {
  if (mode === "kit") {
    mountMcpKitViewer(root);
  } else if (mode === "scene") {
    mountMcpSceneViewer(root);
  } else if (mode === "diagram") {
    mountMcpDiagramViewer(root);
  } else {
    mountMcpDesignViewer(root);
  }
} catch (e) {
  const msg = e instanceof Error ? e.message : String(e);
  root.innerHTML = `<div style="padding:16px; font-family: ui-sans-serif, system-ui, sans-serif; min-height:100dvh; display:flex; align-items:center; justify-content:center;">
    <div style="max-width:720px; text-align:center;">
      <div style="font-weight:700; margin-bottom:8px;">semio MCP viewer crashed (mode: ${mode})</div>
      <div style="white-space:pre-wrap; opacity:0.95;">${msg}</div>
    </div>
  </div>`;
}
