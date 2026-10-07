import { resolve } from "node:path";
import Ajv from "ajv";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

type Lane = "Artifact" | "WindowConfig" | "HostOnly";
type Fixture = { schema: string; owner: "EquationPlayApp"; source: string; routes: { id: string; lane: Lane }[]; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

function oracle(fixture: Fixture, source: string): boolean {
  const ids = [...source.match(/EQUATION_TOOL_IDS: &\[&str\] = &\[([^\]]*)\]/s)?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((match) => match[1]!);
  const contracts = new Map([...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[ArtifactToolPublicationLane::(Artifact|WindowConfig|HostOnly)\] \}/g)].map((match) => [match[1]!, match[2]! as Lane]));
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = fixture.routes.map(({ id }) => id);
  return fixture.schema === "semio.app.publication-authority.v1" && Object.values(fixture.laws).every(Boolean)
    && fixture.ui.locales.join(",") === "en,de" && fixture.ui.accessibleLabels && fixture.ui.customizableUi
    && exact(ids, expected) && exact(classifications, expected) && exact([...contracts.keys()], expected)
    && fixture.routes.every(({ id, lane }) => contracts.get(id) === lane)
    && ["ToolExecutionContract::resumable", "semio_framework_plugin::bounded_first_step_tool_proofs!", "build_artifact_store_one_item_preparation_factory", "register_window_config_owners", "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()", "authority.prepare_one_item", "fn cancel(&mut self)", "fn begin_close(&mut self)", "base.return_to_registry()", "fn terminal_is_empty(&self)", "LocalizedLabel::native", ".default_layout(edit::layout())"].every((anchor) => source.includes(anchor));
}

/** 📣️ Verifies the Equation app's eight literal publication routes and their authority laws. */
export async function verifyMathematicalPublicationAuthority(repoRoot: string, packageRoot: string): Promise<void> {
  const manifest = await Bun.file(resolve(packageRoot, "package.json")).json() as Record<string, unknown>;
  const scripts = manifest.scripts as Record<string, unknown>;
  const devDependencies = manifest.devDependencies as Record<string, unknown> | undefined;
  if (manifest.name !== "@semio-tech/mathematical-js" || typeof manifest.description !== "string" || !manifest.description.startsWith("@semio-tech/mathematical-js ")) throw new Error("Mathematical package identity is not domain-scoped");
  if (JSON.stringify(scripts) !== JSON.stringify({ test: "bun nx run @semio-tech/mathematical-js:test" })) throw new Error("Mathematical package scripts do not match its Nx targets");
  if (manifest.dependencies !== undefined || devDependencies?.ajv !== "^8.20.0") throw new Error("Mathematical package has runtime dependencies or lacks its Ajv test dependency");
  const plugin = resolve(packageRoot, "../..");
  const authority = resolve(plugin, "🧫️fixtures/📣️publication-authority");
  const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
  const source = await Bun.file(resolve(plugin, fixture.source)).text();
  if (!oracle(fixture, source)) throw new Error("Mathematical publication-authority oracle rejected production");
  const hostileSource = [source.replace('ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },', ""), source.replace("            || request.generation != request.authority.generation()\n", ""), source.replace('.action_interactive_job("setPoints", InteractiveJobClassification::Migrated)', "")];
  if (hostileSource.some((candidate) => oracle(fixture, candidate))) throw new Error("Mathematical oracle accepted a hostile source mutation");
  console.error(`validated Mathematical publication authority; routes=${fixture.routes.length}; WindowConfig=1; oracle=owned; hostile=3`);
  const subset = resolve(plugin, "🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any");
  runCmd(process.execPath, ["test", resolve(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), resolve(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")], { cwd: repoRoot });
}
