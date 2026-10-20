/** 📐 End states of the manifest targets this set converts to cached tasks (or records a reason for): each spec derives the `after` target from the live `before` target, so a peer's unrelated edit of the same target survives. `bun plan.ts` writes `ops.json` (before and after per target) that `apply.ts` verifies and applies.
 * @see ../apply.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = process.env.SEMIO_ROOT ?? resolve(import.meta.dir, "../../../../../../../../..");
type Json = Record<string, any>;
type Spec = { id: string; manifest: string; target: string; note: string; group: string; requires?: { target: string; key: string; contains: string; by: string }[]; derive: (before: Json) => Json };

const clone = <T,>(v: T): T => structuredClone(v);
const decision = (klass: string, reason: string) => ({ version: 1, class: klass, reason });
const dropDecision = (t: Json) => {
  if (t.metadata?.semio?.cache !== undefined) delete t.metadata.semio.cache;
  if (t.metadata?.semio && !Object.keys(t.metadata.semio).length) delete t.metadata.semio;
  if (t.metadata && !Object.keys(t.metadata).length) delete t.metadata;
};
const setDecision = (t: Json, d: Json) => { t.metadata ??= {}; t.metadata.semio ??= {}; t.metadata.semio.cache = d; };
const cachedReceipt = (t: Json, receipt: string) => { t.cache = true; t.outputs = [receipt]; setDecision(t, decision("deterministic", "pure-function-of-inputs")); };
const noop = (t: Json, dependsOn: unknown[]) => {
  for (const key of Object.keys(t)) if (!["metadata"].includes(key)) delete t[key];
  t.executor = "repo:noop"; t.dependsOn = dependsOn; setDecision(t, decision("aggregate", "orders-prerequisites"));
};

const R = "🧰️framework/🛍️products/🦑️repo/🔨️modules", L = `${R}/📚️library`;
const specs: Spec[] = [
  {
    group: "wasm-receipt", id: "@semio-tech/framework-os-node-graph-rs:wasm", target: "wasm", manifest: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/📋️project.json",
    note: "same receipted wasm package producer (`buildAdmittedWasmWebV1` + `verifyProducedWasmPackageV1`) as the cached siblings surface, editor, puzzle, flow; only the authored `cache: false` kept it uncached",
    derive: (t) => { t.cache = true; dropDecision(t); return t; },
  },
  {
    group: "wasm-receipt", id: "@semio-tech/s-spatial-kernel-semio-session-rs:wasm", target: "wasm", manifest: "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📋️project.json",
    note: "receipted wasm package producer (`runWasmScript`, the shared rust-package-commands wasm lane); `WasmScript.run()` ignores arguments, so `forwardAllArgs` is dropped",
    derive: (t) => { t.cache = true; dropDecision(t); delete t.options.forwardAllArgs; return t; },
  },
  {
    group: "wasm-receipt", id: "@semio-tech/s-cad-composition-rs:wasm", target: "wasm", manifest: "✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/📋️project.json",
    note: "receipted wasm package producer (`buildRepositoryWasmWebV1` + `verifyProducedWasmPackageV1`); `WasmScript.run()` ignores arguments, so `forwardAllArgs` is dropped",
    derive: (t) => { t.cache = true; dropDecision(t); delete t.options.forwardAllArgs; return t; },
  },
  {
    group: "course", id: "@teaching/architecture-course:build", target: "build", manifest: "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json",
    note: "Vite release build published through the repository artifact destination (`withRepositoryArtifactDestinationV1` records the store receipt), declared exactly like the cached deck build; extra caller arguments no longer flow into the compiler; inputs name the whole course tree (builder, deploy, content) and the dependency projects, because the Vite configuration path is a string the command closure does not follow",
    derive: (t) => {
      cachedReceipt(t, "{projectRoot}/.🧾️store/build.json");
      delete t.options.forwardAllArgs;
      t.inputs = ["default", "^default", "{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/**/*", { env: "PROCTOR_URL" }, { dependentTasksOutputFiles: "**/*", transitive: false }];
      return t;
    },
  },
  {
    group: "managed-store-receipt", id: "semio-framework-canvas:stage-runtime-assets", target: "stage-runtime-assets", manifest: "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/📋️project.json",
    note: "publishes its two trees through `stageAdmittedArtifactTreesV1`, whose provider uses `withRepositoryArtifactDestinationV1` (receipt recorded); the owner already declares the exact inputs",
    derive: (t) => { cachedReceipt(t, "{projectRoot}/.🧾️store/stage-runtime-assets.json"); return t; },
  },
  {
    group: "managed-store-receipt", id: "workspace:build-storybook", target: "build-storybook", manifest: "📋️project.json",
    note: "`buildRepositoryStorybookV1` publishes through `withRepositoryArtifactDestinationV1` (receipt recorded); the script accepts only its declared intent, so caller arguments are no longer forwarded",
    derive: (t) => { cachedReceipt(t, "{projectRoot}/.🧾️store/build-storybook.json"); delete t.options.forwardAllArgs; return t; },
  },
  ...[
    ["@semio-tech/repo-test-runner-go:build", `${R}/🏃️test-runner/📦️packages/🐹️go/📋️project.json`],
    ["@semio-tech/repo-metrics-go:build", `${R}/📊️metrics/📦️packages/🐹️go/📋️project.json`],
  ].map(([id, manifest]): Spec => ({
    group: "verdict", id: id!, target: "build", manifest: manifest!,
    note: "`runCanonicalGoBuild(PACKAGE_DIR, [\"./...\"])` passes no `-o`, so `go build ./...` publishes nothing: a compile verdict over the module and its declared Go toolchain inputs, exactly like the cached `test` sibling",
    derive: (t) => { t.cache = true; t.outputs = []; setDecision(t, decision("deterministic", "check-without-outputs")); return t; },
  })),
  {
    group: "noop", id: "@semio-tech/ui-styling:generate", target: "generate", manifest: "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📋️project.json",
    note: "`GenerateScript.run()` only prints `[nx-generate] styling artifacts ready`; the artifacts come from the declared dependency `ui-styling-tokens:generate`, so the target is a pure ordering node",
    derive: (t) => { noop(t, t.dependsOn ?? []); return t; },
  },
  {
    group: "noop", id: "@semio-tech/ui-styling-py:generate", target: "generate", manifest: "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/📋️project.json",
    note: "`runStylingPythonGenerateScript` only prints `[nx-generate] styling artifacts ready`; pure ordering node over `ui-styling-tokens:generate`",
    derive: (t) => { noop(t, t.dependsOn ?? []); return t; },
  },
  ...[
    ["workspace:build", "build", "📋️project.json", "`BuildScript.run` without a slice only logs `Completed the <mode> workspace build graph`; the work is the declared `build` dependencies"],
    ["workspace:cpp", "cpp", "📋️project.json", "`CppScript` with the default command `all` only logs `Completed the <mode> native graph`; the work is the declared `cpp-test` dependency"],
    ["workspace:prepare", "prepare", "📋️project.json", "`setup prepare` only logs `Build prerequisites completed`; the work is the five declared generator dependencies"],
    ["@semio-tech/framework-os-dev:build", "build", "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json", "`CompleteScript` only logs `Production build complete: s/react/release`; the work is the declared `build-s-react` dependency"],
  ].map(([id, target, manifest, note]): Spec => ({
    group: "noop", id: id!, target: target!, manifest: manifest!, note: note!,
    derive: (t) => { noop(t, t.dependsOn ?? []); return t; },
  })),
  {
    group: "noop", id: "@semio-tech/repo-vscode:build", target: "build", manifest: "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📋️project.json",
    note: "`BuildScript extends CompileExtensionScript {}`: the same command as the cached `compile-extension`, which already owns the `📤️dist/<mode>` output; `build` becomes its ordering alias (two cached owners of one output directory were the overlap cache-fix found)",
    derive: (t) => { noop(t, ["compile-extension"]); return t; },
  },
  {
    group: "noop", id: "@semio-tech/repo-vscode:build-vsix", target: "build-vsix", manifest: "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📋️project.json",
    note: "keeps its direct dependency on the compiler now that `build` is an ordering alias without outputs; stays uncached (writes the vsix inside the directory `compile-extension` owns)",
    derive: (t) => { t.dependsOn = ["compile-extension"]; return t; },
  },
  {
    group: "verdict", id: "@semio-tech/print-viz-inference:build", target: "build", manifest: "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📋️project.json",
    note: "`runBuildScript` runs `tsc --noEmit` twice and loads the barrel: a type-check verdict; the program reads files across the whole print tree (worker, io, schema, native-chart-grammar tests), which `production` excludes, so the print tree is declared",
    derive: (t) => {
      t.cache = true; t.outputs = []; delete t.options.forwardAllArgs;
      t.inputs = ["production", "^production", "{workspaceRoot}/🧰️framework/🛍️products/📓️print/**/*"];
      setDecision(t, decision("deterministic", "check-without-outputs"));
      return t;
    },
  },
  {
    group: "verdict", id: "@semio-tech/mit-bestand-bericht:build", target: "build", manifest: "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json",
    note: "`CompleteScript` only checks the `%PDF-` header of the three published report PDFs, which are the declared outputs of the three cached `build-*` dependencies; a verdict over those outputs",
    requires: ["zwischenbericht", "forschungsbericht", "kompaktbericht"].map((d) => ({ target: `build-${d}`, key: "outputs", contains: ".cache/b/p/m/print", by: "demo-bzl (report PDFs are published into the products store slot, not into dist/documents)" })),
    derive: (t) => {
      t.cache = true; t.outputs = [];
      t.inputs = ["sharedGlobals", "{workspaceRoot}/♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/**/*", "{workspaceRoot}/🧰️framework/🛍️products/📓️print/📦️artifacts/**/*", { dependentTasksOutputFiles: "**/*", transitive: false }];
      setDecision(t, decision("deterministic", "check-without-outputs"));
      return t;
    },
  },
  ...[
    ["repo:cache-retire-inactive-preview", "diagnostic", "measures-environment"],
    ["repo:cache-retire-inactive", "side-effecting", "mutates-host-store"],
    ["repo:test-cargo-retirement", "live", "exercises-host-resources"],
  ].map(([id, klass, reason]): Spec => ({
    group: "reason", id: id!, target: id!.split(":")[1]!, manifest: `${L}/⚡️caching/📋️project.json`,
    note: `stays uncached; records the reason (${klass}/${reason}) that the 2026-10-20 owner decisions did not cover because the target was added afterwards`,
    derive: (t) => { setDecision(t, decision(klass!, reason!)); return t; },
  })),
];

export type Op = { id: string; manifest: string; target: string; group: string; note: string; requires: NonNullable<Spec["requires"]>; before: Json; after: Json };
const ops: Op[] = specs.map((s) => {
  const json = JSON.parse(readFileSync(join(root, s.manifest), "utf8"));
  const current = json.targets?.[s.target];
  if (!current) throw new Error(`missing ${s.id} in ${s.manifest}`);
  return { id: s.id, manifest: s.manifest, target: s.target, group: s.group, note: s.note, requires: s.requires ?? [], before: clone(current), after: s.derive(clone(current)) };
});
writeFileSync(join(import.meta.dir, "ops.json"), JSON.stringify(ops, null, 1) + "\n");
console.log(`${ops.length} ops: ${Object.entries(ops.reduce<Record<string, number>>((m, o) => ({ ...m, [o.group]: (m[o.group] ?? 0) + 1 }), {})).map(([k, v]) => `${k}=${v}`).join(" ")}`);
