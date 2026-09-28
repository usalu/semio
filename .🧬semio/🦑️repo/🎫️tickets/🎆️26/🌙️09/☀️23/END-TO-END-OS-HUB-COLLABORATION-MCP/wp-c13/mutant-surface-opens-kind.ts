/** 🧬️ C13: the pairing-rule law must refuse the rule it replaced — replays both hub fixtures against the pre-fix browser rule
 * (the app must declare the kind, or a plugin-level kind its dialect names), a viewer-blind hub rule and the live twin.
 * usage: bun mutant-surface-opens-kind.ts */
import { readFileSync } from "node:fs";
import { surfaceOpensArtifactKindV1, type SurfaceArtifactKindV1, type SurfaceKindAppV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
type Rule = (plugin: readonly SurfaceArtifactKindV1[], apps: readonly SurfaceKindAppV1[], app: SurfaceKindAppV1, artifact: { kind: string; schema: string }) => boolean;
const declares = (kinds: readonly SurfaceArtifactKindV1[], artifact: { kind: string; schema: string }) => kinds.some((kind) => kind.id === artifact.kind && kind.schema === artifact.schema);
const rules: Record<string, Rule> = {
  "pre-fix browser rule": (plugin, _apps, app, artifact) => declares(app.artifactKinds, artifact) || (declares(plugin, artifact) && app.dialect.artifactKind === artifact.kind),
  "viewer-blind hub rule": (plugin, _apps, app, artifact) => (declares(plugin, artifact) ? app.dialect.artifactKind === artifact.kind : declares(app.artifactKinds, artifact)),
  "viewer opens any editor's kinds": (plugin, apps, app, artifact) => (declares(plugin, artifact) ? app.dialect.artifactKind === artifact.kind : declares(app.artifactKinds, artifact) || (app.role === "viewer" && apps.some((editor) => editor.role === "editor" && declares(editor.artifactKinds, artifact)))),
  fix: surfaceOpensArtifactKindV1,
};
const root = "/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/";
const pairing = JSON.parse(readFileSync(`${root}🗂️surface-opens-kind/🔣️.json`, "utf8")).cases as any[];
const targets = JSON.parse(readFileSync(`${root}🎯️descriptor-open-targets/🔣️.json`, "utf8")).cases as any[];
const dialect = { artifactKind: "s.fixture.document", standard: "1", subset: "*" };
for (const [name, rule] of Object.entries(rules)) {
  const failing = pairing.filter((c) => rule(c.pluginArtifactKinds, c.apps, c.apps[c.app], c.artifact) !== c.opens).map((c) => c.name);
  for (const c of targets) {
    const kind = { id: c.kindId, schema: "fixture.document@1" };
    const apps: SurfaceKindAppV1[] = [{ role: "editor", dialect, artifactKinds: c.declaredOn === "plugin" ? [] : [kind] }, { role: "viewer", dialect, artifactKinds: [] }];
    const opened = c.execution !== "isolated" ? [] : apps.filter((app) => rule(c.declaredOn === "editor" ? [] : [kind], apps, app, { kind: kind.id, schema: kind.schema })).map((app) => app.role);
    if (JSON.stringify(opened) !== JSON.stringify(c.targets.map((t: any) => t.role))) failing.push(`open-targets: ${c.name}`);
  }
  console.log(`${name}: ${pairing.length + targets.length - failing.length}/${pairing.length + targets.length} pass; failing ${JSON.stringify(failing)}`);
}
