// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — pg-boss worker process entry (library lives in index.ts).
// #endregion 🧲Header

import { runRepoServerWorker } from "./index.ts";

runRepoServerWorker().catch((err) => {
  console.error("[worker] fatal error:", err);
  process.exit(1);
});
