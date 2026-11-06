process.env.SEMIO_TICKET_TIMEOUT_MS="7200000";
process.argv=[process.execPath,`${import.meta.dirname}/repo-mcp.ts`,"tools/call",`@${import.meta.dirname}/native-borrow-guard-test.json`];
await import("./repo-mcp.ts");
