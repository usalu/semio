import { defineConfig } from "vitest/config";

/** 🧪️ Ticket-local runner for the contract self-tests the broken package runner would call. */
export default defineConfig({ root: import.meta.dirname, test: { include: ["card-roles-selftests.test.ts"], environment: "node", testTimeout: 60000 } });
