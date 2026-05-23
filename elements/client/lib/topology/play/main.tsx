// #region 🧲Header
// 💻 elements/client/lib/topology/play/main.tsx — Vite entry: mounts topology play shell (React-only).
// #endregion 🧲Header

import { mountReactApp } from "@elements/ui";

import { createTopologyPlayElement } from "./react.tsx";

mountReactApp(createTopologyPlayElement());
