#!/usr/bin/env bun
/** 📕️ Norm TypeScript package. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region 🔖️Types
type RetainedRoute = { id: string; emittedLanes: string[]; publicationLanes: string[]; execution: string; admission: string; reducer: string };
type RetainedContract = { toolId: string; lanes: string[] };
type RetainedApp = { variant: string; controller: string; documentSchema: string };
type RetainedFixture = {
  schemaVersion: number;
  factory: { type: string; payloadSchema: string; maximumRawBytes: number; shared: boolean };
  publicationContracts: RetainedContract[];
  routes: RetainedRoute[];
  apps: RetainedApp[];
  expected: { apps: number; routesPerApp: number; identities: number; retained: number; batchOnlyPendingRewrite: number };
};
type DescriptorAction = { id: string; semantics?: { execution?: { interactiveJob?: string } } };
type DescriptorApp = { id: string; role: string; windowKinds?: { id: string; actions?: DescriptorAction[] }[] };
type Descriptor = { manifest: { apps: DescriptorApp[] } };
//#endregion 🔖️Types

//#region 🔖️Test
/** 🧵️ Independent TypeScript check of the same language-neutral cohort fixture the Rust oracle reads. */
class TestScript extends BundleScript {
  run(): void {
    const plugin = join(this.root, "..", "..");
    const fixture = JSON.parse(readFileSync(join(plugin, "🧪️fixtures", "🧫️retained-command-dispositions", "🔣️.json"), "utf8")) as RetainedFixture;
    const descriptor = JSON.parse(readFileSync(join(plugin, "🔣️.json"), "utf8")) as Descriptor;
    const failures: string[] = [];
    const expect = (ok: boolean, message: string) => {
      if (!ok) failures.push(message);
    };

    expect(fixture.schemaVersion === 2, "fixture schemaVersion must be 2");
    expect(fixture.factory.shared, "the norm cohort must declare one shared factory");
    expect(fixture.apps.length === fixture.expected.apps, `fixture declares ${fixture.apps.length} apps, expected ${fixture.expected.apps}`);
    expect(fixture.routes.length === fixture.expected.routesPerApp, `fixture declares ${fixture.routes.length} routes per app, expected ${fixture.expected.routesPerApp}`);
    expect(fixture.expected.identities === fixture.expected.apps * fixture.expected.routesPerApp, "identities must be apps × routesPerApp");
    expect(fixture.expected.retained === fixture.expected.identities, "every identity must be retained (migrated)");
    expect(fixture.expected.batchOnlyPendingRewrite === 0, "no norm identity may remain batchOnlyPendingRewrite");
    expect(fixture.publicationContracts.length === fixture.routes.length, "one publication contract per route");
    for (const [index, route] of fixture.routes.entries()) {
      const contract = fixture.publicationContracts[index];
      expect(route.admission === "migrated", `route ${route.id} admission is ${route.admission}`);
      expect(contract?.toolId === route.id, `publication contract ${index} is ${contract?.toolId}, expected ${route.id}`);
      expect(JSON.stringify(contract?.lanes) === JSON.stringify(route.publicationLanes), `route ${route.id} publication lanes diverge from its contract`);
      expect(route.publicationLanes.length > 0, `route ${route.id} declares no publication lane`);
      expect(!route.publicationLanes.includes("HostOnly") || route.publicationLanes.length === 1, `route ${route.id} pairs HostOnly with another lane`);
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
      for (const window of editor.windowKinds ?? []) {
        for (const route of fixture.routes) {
          const action = (window.actions ?? []).find((entry) => entry.id === route.id);
          if (!action) {
            failures.push(`descriptor ${app.controller} window ${window.id} does not declare ${route.id}`);
            continue;
          }
          const admission = action.semantics?.execution?.interactiveJob;
          if (admission === undefined) continue;
          classified += 1;
          expect(admission === route.admission, `descriptor ${app.controller} ${route.id} is ${admission}, fixture says ${route.admission}`);
        }
      }
      identities += fixture.routes.length;
    }
    expect(identities === fixture.expected.identities, `walked ${identities} identities, expected ${fixture.expected.identities}`);

    if (failures.length > 0) {
      for (const failure of failures) console.error(`norm retained cohort: ${failure}`);
      process.exit(1);
    }
    console.log(`norm retained cohort ok: ${fixture.apps.length} editors × ${fixture.routes.length} migrated routes, ${classified} descriptor rows carried a classification`);
    if (classified === 0) console.warn("norm retained cohort: the committed 🔣️.json predates interactiveJob — re-run describe to make the drift check bite");
    const examples = [
      ["🌬️din16798", "🎬️demo"],
      ["⚡️din18599", "🎬️demo"],
      ["🧱️din4108", "🎬️demo"],
      ["⚖️en1990", "🏢️high-consequence-office"],
      ["🏋️en1991", "🔥️retail-hydrocarbon-fire"],
      ["🏛️en1992", "🛢️liquid-retaining-fem-anchor"],
      ["🔩️en1993", "🔩️high-strength-connection"],
      ["🧩️en1994", "🌉️composite-bridge-girder"],
      ["🪵️en1995", "🌉️glulam-footbridge"],
      ["🪨️en1996", "🧱️loadbearing-wall"],
      ["🌍️en1997", "🎬️demo"],
      ["🫨️en1998", "🏢️seismic-rc-frame"],
      ["🪶️en1999", "🏠️aluminium-roof-purlin"],
      ["📇️iso16757", "🎬️demo"],
      ["🏭️vdi3805", "🎬️demo"],
    ] as const;
    const tests = examples.flatMap(([artifact, example]) => {
      const subset = join(plugin, "🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any");
      return [join(subset, "📚️examples", example, "🧪️tests/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🟦️.ts")];
    });
    runCmd(process.execPath, ["test", ...tests]);
  }
}
//#endregion 🔖️Test

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
