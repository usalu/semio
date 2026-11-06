process.argv=[process.argv[0]!,process.argv[1]!,"tools/call","@"+import.meta.dirname+"/capacity-status-input.json"];
await import("./repo-mcp.ts");
