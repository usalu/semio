process.env.SEMIO_TICKET_TIMEOUT_MS="7200000";
process.argv=[process.argv[0]!,process.argv[1]!,"tools/call","@"+import.meta.dirname+"/escrow-publisher-native-request.json"];
await import("./repo-mcp.ts");
