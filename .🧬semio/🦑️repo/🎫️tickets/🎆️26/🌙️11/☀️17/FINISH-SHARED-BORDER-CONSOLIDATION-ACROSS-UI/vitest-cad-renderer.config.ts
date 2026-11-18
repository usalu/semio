import { defineConfig } from "vitest/config";

/** 🧪️ Ticket-local config: runs the CAD aside-frame source test without the wasm-dependent renderer runtime. */
export default defineConfig({ root: import.meta.dirname, test: { environment: "node", include: ["cad-aside-frame.test.ts"] } });
