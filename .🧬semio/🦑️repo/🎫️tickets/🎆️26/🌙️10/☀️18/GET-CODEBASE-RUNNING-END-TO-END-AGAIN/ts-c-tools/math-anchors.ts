import { dirname, join, resolve } from "node:path";
const plugin = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical";
const fixture = await Bun.file(resolve(plugin, "🧪️tests/🧫️fixtures/📣️publication-authority/🔣️.json")).json() as any;
const sourceDirectory = dirname(fixture.source);
const source = (await Promise.all([fixture.source, ...fixture.mounted.map((m: string) => join(sourceDirectory, m))].map((p: string) => Bun.file(resolve(plugin, p)).text()))).join("\n");
const anchors = ["ToolExecutionContract::resumable", "semio_framework_plugin::bounded_first_step_tool_proofs!", "build_artifact_store_one_item_preparation_factory", "register_window_config_owners", "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()", "authority.prepare_one_item", "fn cancel(&mut self)", "fn begin_close(&mut self)", "base.return_to_registry()", "fn terminal_is_empty(&self)", "LocalizedLabel::native", ".default_layout(edit::layout())"];
console.log("missing", anchors.filter((a) => !source.includes(a)), "laws", Object.entries(fixture.laws).filter(([, v]) => !v));
const ids = [...source.match(/EQUATION_TOOL_IDS: &\[&str\] = &\[([^\]]*)\]/s)?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((m: any) => m[1]);
console.log("ids", ids.length, "routes", fixture.routes.length);
