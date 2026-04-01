// #region 🔖Header
// [🧰repo⌨️server⚙️nextconfig](repo://p/i/repo/b/b/server/f/next.config.ts)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Next.js configuration for the repo server app.
// #endregion 🔖Header

import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "standalone",
  serverExternalPackages: ["pg", "pg-boss"],
};

export default nextConfig;
