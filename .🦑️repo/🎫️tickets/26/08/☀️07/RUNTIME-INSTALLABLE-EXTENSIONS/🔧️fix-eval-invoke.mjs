import fs from "fs";

const files = process.argv.slice(2);
for (const file of files) {
  let src = fs.readFileSync(file, "utf8");
  const re =
    /        if let Some\(pending\) = host\.take_pending_extension_eval\(\) \{\n            if let Some\(plugin_id\) = flow_core::flow_extension_plugin_id\(&pending\.extension_id\) \{\n                let request_json = serde_json::json!\(\{\n                    "operatorId": pending\.operator_id,\n                    "inputJson": pending\.input_json,\n                    "nodeHash": pending\.node_hash,\n                \}\)\n                \.to_string\(\);\n                effects\.push\(((?:semio_framework_core::kernel::)?HostEffect)::InvokeExtension \{\n                    extension_id: pending\.extension_id\.clone\(\),\n                    capability: "evaluate"\.into\(\),\n                    request_json: request_json,\n                    response_action: "flowEvalResolve"\.into\(\),\n                \}\);\n            \}\n        \}/;

  if (!re.test(src)) {
    console.error("pattern not found in", file);
    continue;
  }
  src = src.replace(
    re,
    `        if let Some(pending) = host.take_pending_extension_eval() {
            let request_json = serde_json::json!({
                "operatorId": pending.operator_id,
                "inputJson": pending.input_json,
                "nodeHash": pending.node_hash,
            })
            .to_string();
            effects.push($1::InvokeExtension {
                extension_id: pending.extension_id,
                capability: "evaluate".into(),
                request_json,
                response_action: "flowEvalResolve".into(),
            });
        }`,
  );
  fs.writeFileSync(file, src);
  console.log("updated", file);
}
