import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { runCmd } from "@semio-tech/framework/process/execution";
import plan from "../🧫️fixtures/🏃️package-runner/🔣️.json" with { type: "json" };
import { testNormDocumentContractOracle } from "../🪪️document-contract/🟦️.ts";
import { deleteEntitiesFailuresV1 } from "../🗑️delete-entities/🟦️.ts";

type RetainedRoute = { id: string; emittedLanes: string[]; publicationLanes: string[]; execution: string; admission: string; reducer: string };
type RetainedContract = { toolId: string; lanes: string[] };
type RetainedApp = { variant: string; controller: string; artifactSchema: string };
type RetainedFixture = {
  schemaVersion: number;
  factory: { type: string; payloadSchema: string; maximumRawBytes: number; shared: boolean };
  publicationContracts: RetainedContract[];
  routes: RetainedRoute[];
  apps: RetainedApp[];
  expected: { apps: number; routesPerApp: number; identities: number; retained: number; batchOnlyPendingRewrite: number };
  live?: { routes: { id: string; executes?: string }[] };
};
type DescriptorAction = { id: string; semantics?: { execution?: { interactiveJob?: string } } };
type DescriptorApp = { id: string; role: string; actions?: DescriptorAction[]; windowKinds?: { id: string; actions?: DescriptorAction[] }[] };
type Descriptor = { manifest: { apps: DescriptorApp[] } };

/** 🧭️ Resolves the committed test plan against the source owner and deployed component. */
export function normPackageTestPlan(plugin: string, repoRoot: string): { descriptor: string; suites: string[] } {
  return { descriptor: join(repoRoot, plan.descriptor), suites: plan.suites.map(path => join(plugin, path)) };
}

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): Promise<void> {
  const plugin = join(context.root, "..", "..");
  const fixture = JSON.parse(readFileSync(join(plugin, "📇️registry", "🧬️contract", "🧪️tests", "🧫️fixtures", "🧫️retained-command-dispositions", "🔣️.json"), "utf8")) as RetainedFixture;
  const testPlan = normPackageTestPlan(plugin, context.repoRoot);
  const descriptor = JSON.parse(readFileSync(testPlan.descriptor, "utf8")) as Descriptor;
  const failures: string[] = [];
  const expect = (ok: boolean, message: string) => {
    if (!ok) failures.push(message);
  };

  expect(fixture.schemaVersion === 2, "fixture schemaVersion must be 2");
  expect(fixture.factory.shared, "the norm cohort must declare one shared factory");
  expect(fixture.apps.length === fixture.expected.apps, `fixture declares ${fixture.apps.length} apps, expected ${fixture.expected.apps}`);
  expect(fixture.routes.length === fixture.expected.routesPerApp, `fixture declares ${fixture.routes.length} routes per app, expected ${fixture.expected.routesPerApp}`);
  expect(fixture.expected.identities === fixture.expected.apps * fixture.expected.routesPerApp, "identities must be apps × routesPerApp");
  expect(fixture.expected.retained === fixture.expected.identities, "every identity must be retained");
  expect(fixture.expected.batchOnlyPendingRewrite === 0, "no norm identity may remain batchOnlyPendingRewrite");
  expect(fixture.publicationContracts.length === fixture.routes.length, "one publication contract per route");
  for (const [index, route] of fixture.routes.entries()) {
    const contract = fixture.publicationContracts[index];
    expect(route.admission === "migrated", `route ${route.id} admission is ${route.admission}`);
    expect(contract?.toolId === route.id, `publication contract ${index} is ${contract?.toolId}, expected ${route.id}`);
    expect(JSON.stringify(contract?.lanes) === JSON.stringify(route.publicationLanes), `route ${route.id} publication lanes diverge from its contract`);
    expect(route.publicationLanes.length > 0, `route ${route.id} declares no publication lane`);
    expect(!route.publicationLanes.includes("host-only") || route.publicationLanes.length === 1, `route ${route.id} pairs host-only with another lane`);
  }

  const editors = new Map(descriptor.manifest.apps.filter((app) => app.role === "editor").map((app) => [app.id, app]));
  let classified = 0;
  let identities = 0;
  for (const app of fixture.apps) {
    const editor = editors.get(app.controller);
    if (!editor) {
      failures.push(`descriptor has no editor ${app.controller}`);
      continue;
    }
    for (const route of fixture.routes) {
      const action = editor.actions?.find(entry => entry.id === route.id);
      if (!action) {
        failures.push(`descriptor ${app.controller} does not declare ${route.id}`);
        continue;
      }
      const admission = action.semantics?.execution?.interactiveJob;
      classified += 1;
      expect(admission === route.admission, `descriptor ${app.controller} ${route.id} is ${admission}, fixture says ${route.admission}`);
    }
    identities += fixture.routes.length;
  }
  expect(identities === fixture.expected.identities, `walked ${identities} identities, expected ${fixture.expected.identities}`);

  const deleteRoute = fixture.live?.routes.find((route) => route.id === "deleteEntities");
  expect(deleteRoute?.executes === "🗑️delete-entities", "the deleteEntities route names the fixture it executes");
  for (const failure of deleteEntitiesFailuresV1()) failures.push(`deleteEntities executed: ${failure}`);

  expect(classified === fixture.expected.identities, "every app route carries its declared classification");

  if (failures.length > 0) {
    for (const failure of failures) console.error(`norm retained cohort: ${failure}`);
    process.exit(1);
  }
  console.log(`norm retained cohort ok: ${fixture.apps.length} editors × ${fixture.routes.length} retained routes, ${classified} descriptor rows carried a classification`);
  const artifacts = join(plugin, "🗿️artifacts");
  const tests = readdirSync(artifacts, { withFileTypes: true }).filter((entry) => entry.isDirectory()).flatMap((artifact) => {
    const subset = join(artifacts, artifact.name, "🏅️standards/🔖️1/🪆️subsets/✳️any");
    const examples = existsSync(join(subset, "📚️examples")) ? readdirSync(join(subset, "📚️examples")).map((example) => join(subset, "📚️examples", example, "🧪️tests/🧩️example/🟦️.ts")) : [];
    return [...examples, join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")].filter((path) => existsSync(path));
  });
  await testNormDocumentContractOracle();
  tests.push(...testPlan.suites);
  runCmd(process.execPath, ["test", ...tests]);
}
