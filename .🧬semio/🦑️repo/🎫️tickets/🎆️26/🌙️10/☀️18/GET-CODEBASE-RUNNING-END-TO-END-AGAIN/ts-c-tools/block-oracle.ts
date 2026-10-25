import { resolve } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
const plugin = resolve("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block");
const fixture = await Bun.file(resolve(plugin, "🧪️tests/🧫️fixtures/🧪️publication-authority/🔣️.json")).json() as any;
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
const ANCHORS = ["ToolExecutionContract::bounded_first_step","semio_framework_plugin::bounded_first_step_tool_proofs!","factory_type:","build_artifact_store_one_item_preparation_factory","register_tool_job_factories","build_tool_job","request.operation != request.authority.operation()","request.generation != request.authority.generation()","request.base_revision != request.authority.base_revision()","authority.prepare_one_item","fn cancel(&mut self)","fn begin_close(&mut self)","self.owners.close_step(grant.retained_grant())","fn terminal_is_empty(&self)","LocalizedLabel::native","SelectionMode::Multiple",".default_layout(edit_mode::layout())"];
for (const app of fixture.apps) {
  const source = readModuleTreeSource(resolve(plugin, app.source));
  const ids = [...source.match(new RegExp(`${app.toolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((m: any) => m[1]);
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((m) => `${m[1]}:${[...m[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((l) => l[1]).sort().join("+")}`);
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((m) => m[1]!);
  const expected = app.routes.map((r: any) => r.id);
  console.log(app.owner, "len", source.length, "laws", Object.entries(app.laws).filter(([, v]) => !v).map(([k]) => k), "ids", JSON.stringify(ids.slice(0,3)), ids.length, "expected", expected.length, "contracts", contracts.length, "classifications", classifications.length, "missing anchors", ANCHORS.filter((a) => !source.includes(a)));
}
