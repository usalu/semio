import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

/** 🧫️ Joins original caller fixtures to exact current codec and declaration authority. */
const ticket = resolve(import.meta.dir, "../..");
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const repairs = [
  {
    path: `${plugin}/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`,
    before: 'admission.find("protocol::json::to_json_string(&(verb, wire_args))").expect("bounded wire encoder")',
    after: 'admission.find("semio_framework_pack_json::to_json_string(&(verb, wire_args))").expect("bounded wire encoder")',
  },
  {
    path: `${plugin}/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`,
    before: '        app.bind_instance_id(id).await;\n        let child = new_test_child("child-1").await.expect("construct child");\n        app.register_child("slot", "child-1", test_child_dialect().await, child).await.expect("register child");',
    after: '        app.bind_instance_id(id).await;\n        let declared = ArtifactRef { artifact_id: "child-1".into(), dialect: test_child_dialect().await }.to_uri();\n        app.test_store_mut().await.dispatch(store::ArtifactCommand::Apply {\n            mutations: vec![TestMutation::SetSlotChildren(SetSlotChildren { children: vec![declared] })],\n            description: Some("declare the acknowledged child member".into()),\n            transaction: None,\n        }).await.expect("the actual parent declares the exact member before registration");\n        let child = new_test_child("child-1").await.expect("construct child");\n        app.register_child("slot", "child-1", test_child_dialect().await, child).await.expect("register child");',
  },
  {
    path: `${plugin}/🧪️tests/🔬️app-merge-ui-values/🦀️.rs`,
    before: 'assert_eq!(ui_value_to_dsl_retained(&UiValue::List(ui_list([UiValue::Number(1.0), UiValue::Bool(false)]))).await.expect("retained list"), serde_json::json!([1.0, false]));',
    after: 'assert_eq!(ui_value_to_dsl_retained(&UiValue::List(ui_list([UiValue::Number(1.0), UiValue::Bool(false)]))).await.expect("retained list"), DslValue::from(&serde_json::json!([1.0, false])));',
  },
];
const pairs: { path: string; before: string; beforeSha256: string; after: string }[] = [];
for (const repair of repairs) {
  const absolute = resolve(process.cwd(), repair.path);
  const before = readFileSync(absolute, "utf8");
  if (before.split(repair.before).length !== 2) throw Error(`Exact unique fixture guard refused: ${repair.path}`);
  pairs.push({ path: repair.path, before, beforeSha256: createHash("sha256").update(before).digest("hex"), after: before.replace(repair.before, repair.after) });
}
writeFileSync(resolve(ticket, "📥️inputs/child-emission-current-caller-fixture-joins-pairs.json"), JSON.stringify({ state: "CurrentEmissionCallerFixtureJoins", pairs, preserves: "All original assertions and the padded-wire maximum-production-grant demand" }, null, 2) + "\n");
if (process.argv.includes("--mount")) {
  for (const pair of pairs) {
    const absolute = resolve(process.cwd(), pair.path);
    if (readFileSync(absolute, "utf8") !== pair.before) throw Error(`Concurrent source guard refused: ${pair.path}`);
    writeFileSync(absolute, pair.after);
  }
}
console.log(JSON.stringify({ paths: pairs.length, mounted: process.argv.includes("--mount"), originalAssertionsRemoved: 0 }));
