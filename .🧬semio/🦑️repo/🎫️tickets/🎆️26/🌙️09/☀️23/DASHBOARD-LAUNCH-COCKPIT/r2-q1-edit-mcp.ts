import { readFileSync, writeFileSync } from "node:fs";

const path = "🎮️registry/🦀️.rs";
let text = readFileSync(path, "utf8");
const once = (from: string, to: string): void => {
  if (text.split(from).length !== 2) throw new Error(`not exactly once: ${from.slice(0, 60)}`);
  text = text.replace(from, () => to);
};

once(
  "    fn verb(&mut self, verb: Option<&String>) {",
  `    fn mcp(&mut self, mcp: Option<&Mcp>, parameters: &[ParameterDeclaration]) {
        let Some(mcp) = mcp else { return };
        if !is_slug(&mcp.server) { self.report(format!("mcp server {:?} must be lowercase words joined by \`-\`, \`.\` or \`_\`", mcp.server)); }
        if mcp.clients.is_empty() { self.report("mcp needs at least one client"); }
        for (client, stated) in &mcp.clients {
            if !AGENT_CLIENTS.contains(&client.as_str()) { self.report(format!("mcp client {client:?} is none of {}", AGENT_CLIENTS.join(", "))); }
            for name in stated.parameters.keys().filter(|name| !parameters.iter().any(|declared| &declared.id == *name)) { self.report(format!("mcp client {client} states {name:?}, which the tool does not accept")); }
        }
    }
    fn verb(&mut self, verb: Option<&String>) {`,
);
once(
  "            let own = scope.parameters(&tool.parameters, Origin::Tool);\n            if !valid || tool.command.first()",
  "            let own = scope.parameters(&tool.parameters, Origin::Tool);\n            scope.mcp(tool.mcp.as_ref(), &tool.parameters);\n            if !valid || tool.command.first()",
);
once(
  "cwd: tool.cwd.clone(), env, ready, requires }));",
  "cwd: tool.cwd.clone(), env, ready, requires }));\n            if let Some(entry) = entries.last_mut() { entry.mcp.clone_from(&tool.mcp); }",
);
once("    pub source: String,\n    own: Vec<Parameter>,", "    pub source: String,\n    /// 🔌️ The MCP server the command is for the agent clients that start it, when it declares one.\n    pub mcp: Option<Mcp>,\n    own: Vec<Parameter>,");
once("source: source.to_string(), own, action, haystack: String::new() }", "source: source.to_string(), mcp: None, own, action, haystack: String::new() }");
once(
  "        if let Some(ready) = ready { value[\"ready\"] = serde_json::json!({ \"port\": ready.port, \"path\": ready.path }); if ready.printed { value[\"ready\"][\"printed\"] = true.into(); } }\n        value\n    }\n}\n\nimpl Entry {",
  "        if let Some(ready) = ready { value[\"ready\"] = serde_json::json!({ \"port\": ready.port, \"path\": ready.path }); if ready.printed { value[\"ready\"][\"printed\"] = true.into(); } }\n        if let Some(mcp) = &entry.mcp { value[\"mcp\"] = serde_json::json!(mcp); }\n        value\n    }\n}\n\nimpl Entry {",
);
writeFileSync(path, text);
