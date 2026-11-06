process.env.SEMIO_TICKET_TIMEOUT_MS="14400000";
process.argv=[process.execPath,import.meta.dirname+"/repo-mcp.ts","tools/call","@"+import.meta.dirname+"/actor-context-native-qualification.json"];
await import("./repo-mcp.ts");
