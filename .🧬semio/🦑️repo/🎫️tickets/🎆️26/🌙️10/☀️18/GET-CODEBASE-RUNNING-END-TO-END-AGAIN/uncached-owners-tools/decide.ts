/** 🧭 Decides, for every authored `cache: false` target that records no reason, whether it is a hermetic verification (it becomes cached) or genuinely host-state or interactive (it keeps `cache: false` and records the decision the neutral cache schema names).
 *
 *   bun decide.ts   # reads 🗑️generated/uncached-owners/rows.json (bun scan.ts), writes classification.json next to this file: the recorded decision, its basis and its note for every target
 *
 * Every row cites its basis: `read` (the handler was read), `analysis` (the handler call graph shows no workspace-anchored write, spawn of a live process or host store), `name` (the command word is a mutating, provisioning, publishing or measuring verb).
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const G = join(import.meta.dir, "../🗑️generated/uncached-owners");
export type Decision = { class: string; reason: string };
export type Row = { id: string; manifest: string; target: string; disposition: "cached" | "uncached" | "interactive"; decision: Decision; basis: string; note: string };

const D = {
  check: { class: "deterministic", reason: "check-without-outputs" },
  host: { class: "side-effecting", reason: "mutates-host-store" },
  workspace: { class: "side-effecting", reason: "mutates-workspace" },
  remote: { class: "side-effecting", reason: "publishes-remote" },
  live: { class: "live", reason: "drives-running-process" },
  resources: { class: "live", reason: "exercises-host-resources" },
  measure: { class: "diagnostic", reason: "measures-environment" },
  caller: { class: "diagnostic", reason: "reads-caller-input" },
  reenter: { class: "aggregate", reason: "reenters-build-tool" },
  order: { class: "aggregate", reason: "orders-prerequisites" },
  store: { class: "managed-output", reason: "outputs-in-managed-store" },
  driver: { class: "driver-owned", reason: "freshness-owned-by-driver" },
  serve: { class: "continuous", reason: "serves-until-stopped" },
} as const satisfies Record<string, Decision>;

type Group = { treatment: keyof typeof D; basis: "read" | "analysis" | "name"; note: string; ids: string[] };
const g = (treatment: keyof typeof D, basis: Group["basis"], note: string, ids: string): Group => ({ treatment, basis, note, ids: ids.split(/\s+/).filter(Boolean) });

const GROUPS: Group[] = [
  g("check", "analysis", "contract oracle or twin of the owning component; bun, vitest or cargo-law verdict over repository sources; zero workspace-anchored writes (test-artifact sink only)", `
    mit-bestand-praesentation-projektetage:canonical-architecture mit-bestand-demonstrator:canonical-architecture procedural-js:canonical-architecture
    procedural-generation3d-rs:canonical-architecture flow-extension-brep-rust:canonical-architecture sequence-sequence-rs:canonical-architecture cad-cad-rs:canonical-architecture
    playbook-extension-procedural-rust:canonical-architecture energy-model-rs:canonical-architecture raster-js:canonical-architecture stdio-artifact-contract-rs:canonical-architecture
    stdio-assembly-rs:canonical-architecture stdio-wav-rs:canonical-architecture stdio-gltf-rs:canonical-architecture stdio-semio-rs:canonical-architecture stdio-bmp-rs:canonical-architecture
    note-note-js:canonical-architecture s-spatial-kernel-semio-session-rs:canonical-architecture s-flow-composition-rs:canonical-architecture s-composition-laws-rs:canonical-architecture
    s-fixture-sweep-rs:canonical-architecture os-hub:canonical-architecture puzzle-plugin:canonical-architecture framework-actor:canonical-architecture ui-react:canonical-architecture
    semio-framework-3d:canonical-architecture framework-job-rs:canonical-architecture framework-os:canonical-architecture framework-os-kernel:canonical-architecture
    framework-renderer-wgpu:canonical-architecture framework-plugin:canonical-architecture framework-os-dev:canonical-architecture repo-lib:canonical-architecture`),
  g("check", "read", "standard TypeScript artifact test (runArtifactTypeScriptPackageMain); 77 sibling artifact tests are cached with outputs []", "demonstrator-playground:test trinity-jack:test trinity-rewriting:test dag-dag:test"),
  g("check", "read", "vitest or bun test over the project's own sources and fixtures; the reached files show no spawn, signal, server, browser, network, home or store marker", `
    mit-bestand-demonstrator:test mit-bestand-demonstrator:test-runtime-catalog semio-tech-play:test framework-source-workspace:test-source ui-rs:test-text-editor-source
    repo-cli-go:test-entrypoint-source framework-os-mcp:test-native-staging framework-os-kernel:test-directory-lease-fixture repo-next-storage:test-nx-cache
    repo:test-bootstrap-original-invocation-source framework-async:twin repo-dashboard-rs:test-dispatch repo-dashboard-rs:test-source-freshness`),
  g("check", "read", "the selected tests run on temporary directories, fixtures or the parsed source text only", "framework-process:test-cargo-source-admission framework-process:test-runtime-environment framework-process:test-budget framework-process:test-native-cache-descriptors framework-process:test-native-image-observation"),
  g("check", "read", "owner-declared inputs cover the read domain; the suite writes only the test-artifact sink and spawns nothing live", "repo-lib:test-cargo-workspace-ownership repo-lib:test-dependency-policy-bootstrap repo-lib:test-fixture-ownership repo-lib:go-test"),
  g("check", "read", "validates the course directory with the proctor binary built from the checked-in sources; reads the course, the proctor sources and PROCTOR_COURSE only", "architecture-course:check proctor:check"),

  g("store", "read", "wasm package, site, storybook or cmake build whose product lives in the leased products store behind a receipt (outputs-in-managed-store)", "mit-bestand-demonstrator:build semio-tech-play:build architecture-course:build workspace:build-storybook workspace:cpp-configure workspace:cpp-build s-spatial-kernel-semio-session-rs:wasm s-cad-composition-rs:wasm framework-os-node-graph-rs:wasm"),
  g("driver", "name", "native dashboard binary compiled through the build driver", "repo-dashboard-rs:build"),

  g("live", "read", "drives a real hub, MCP gateway, browser, dev server or docker stack; the verdict is a property of the running process", `
    mit-bestand-demonstrator:test-e2e semio-tech-play:test-e2e semio-tech-play:test-e2e-enduser semio-tech-play:test-release semio-tech-play:test-release-routing architecture-course:test-e2e
    s-spatial-kernel-semio-session-rs:browser-test s-cad-composition-rs:browser-test
    s-gis-services-native:live-agent-loop-check s-gis-services-native:hub-agent-participant-check s-gis-services-native:plugin-coverage-check s-gis-services-native:user-path-check
    s-gis-services-native:inference-quartet-check s-gis-services-native:security-check s-gis-services-native:agent-reply-check
    s-fem-composition-tests:verify-fem-live-visual-publication s-dev-composition:mcp-client-e2e gis-plugin:hub-edit-durability-check
    os-hub-ts:two-client-e2e os-hub-ts:document-growth-e2e os-hub-ts:backup-restore-drill os-hub-ts:agent-ceiling-check os-hub-ts:shutdown-drill os-hub-ts:docker-image-check os-hub-ts:hub-freshness os-hub-ts:backend-run
    os-hub:directory-live-lanes os-hub:live-sign-in-check os-hub:admin-live-journey-check os-hub:browser-actor-child-worker-containment-check os-hub:browser-actor-gis-describe-check
    os-hub:secure-local-smoke os-hub:hub-document-sweep
    hub-renderer-wgpu:journey-check hub-renderer-wgpu:creation-check hub-renderer-wgpu:collaboration-check hub-renderer-wgpu:collaboration-acceptance hub-renderer-wgpu:native-guest-journey-check
    architecture-course:docker-image-check architecture-course:docker-stack-check architecture-course:docker-stack-bundle architecture-course:deploy-check
    proctor:health proctor:capacity
    framework-os-kernel:wal-writer-fence-live framework-renderer-wgpu:browser-dock-acceptance framework-renderer-wgpu:browser-media-acceptance framework-renderer-wgpu:browser-embedded-acceptance
    framework-os-dev:program-matrix framework-os-dev:tool-run-matrix framework-os-dev:two-human framework-os-dev:two-human-viewer framework-os-dev:two-human-cross-undo
    framework-os-dev:connection-budget framework-os-dev:idle-budget framework-os-dev:interaction-latency framework-os-dev:memory-soak framework-os-dev:boot-budget framework-os-dev:time-travel
    framework-os-dev:parity-journey framework-os-dev:cold-boot-check-s framework-os-dev:s-host-foreign-kind-s framework-os-dev:s-host-pinch-diagram-contrast-s
    repo-dashboard-rs:test repo-dashboard-rs:test-quick repo-dashboard-rs:test-long repo-dashboard-rs:test-exhaustive repo-dashboard-rs:test-coverage repo-dashboard-rs:test-journeys
    repo-dashboard-rs:test-load repo-dashboard-rs:test-smoke repo-dashboard-rs:test-authority repo-dashboard-rs:test-execution`),
  g("resources", "read", "exercises the machine's real locks, lease queues, storage volumes, process tree, descriptors, clock or installed toolchain; the verdict is a property of this host and moment", `
    framework-process:test-descriptor-lifetime framework-process:test-process-progress framework-process:test-process-tree-termination framework-process:test-execution-budget
    framework-process:test-resource-leases framework-process:test-native-storage-platform framework-process:test-cargo-test-leases framework-process:test-cargo-build-lease
    framework-process:test-native-artifact-publication framework-process:test-compiler-capacity framework-process:test-compiler-capacity-inheritance framework-process:test-compiler-capacity-custodian
    framework-process:test-exact-cargo-storage framework-process:test-resource-lease-waits framework-process:test-resource-lease-fairness
    repo:test-cargo-build-lease repo:test-next-runtime repo:test-next-dev-runtime repo:test-graph-cache-waiter repo:test-bazel-storage-runtime repo:test-bazel-storage repo:test-tools-partition
    repo:test-daemon-journal repo:test-daemon-journal-workspace repo:test-storage-budget repo:test-leases-partition repo:test-lease-bootstrap repo:test-lease-queue repo:test-javascript-storage
    repo:test-javascript-installed-paths repo:test-javascript-endpoint-fence repo:test-javascript-provider-walk repo:test-runtime-verification-storage repo:test-runtime-verification-storage-runtime
    repo:test-rustup-toolchain repo:test-runtime-bootstrap repo-dashboard-rs:test-process-deadline workspace:cpp-test repo-lib:test-build-caller-birth repo-lib:test-original-default-preparation-control repo-lib:test-go-executable-acceptance
    stdio-assembly-rs:deletion-proof repo-mcp:test-install workspace:test-build-tool-bazel workspace:test-build-tool-bazel-bootstrap`),

  g("measure", "name", "measures or reports the environment: benchmark, report, doctor, probe or repository-state check", `
    semio-framework-3d:bench framework-os-dev:bench framework-os-dev:bench-plugins-native framework-os-dev:bench-plugins-react framework-os-dev:bench-plugins-wgpu framework-os-dev:plugin-size
    framework-renderer-wgpu:native-scale workspace:bench-plugins workspace:test-doctor repo:ci-baseline repo:audit repo:doctor repo:disk-report repo:cache-report repo:cache-verify
    repo-test-domain:test-doctor repo-test-domain:test-inventory repo-test-domain:test-probe repo-lib:probe-nx-owner-arguments repo-lib:test-cross-platform-bootstrap repo-lib:test-preflight-reference-basis
    repo-lib:test-registry-catalog-gitlink-boundary architecture-course:preflight semio-tech-play:release-verify energy-oracle-py:oracle-status os-hub-ts:backend-status`),
  g("caller", "read", "the output is a function of arguments or standard input the caller supplies, which are not declared inputs", `
    bim-model-rs:sqlite-oracle repo:debug-bun-bundle repo:digest-source-inventory repo:digest-source-closure repo:digest-cargo-lock repo:emit-typescript-declarations repo-lib:owner-command
    s-dev-composition:preview-generated energy-oracle-py:oracle-run energy-oracle-py:oracle-native energy-oracle-py:oracle-epjson energy-oracle-py:oracle-emit
    repo-developer-tools:git-show repo-developer-tools:git-blame repo-developer-tools:git-diff repo-developer-tools:git-history repo-developer-tools:git-status`),
  g("reenter", "read", "re-enters the build tool (bun repo run / run-many --skip-cache) for other targets instead of declaring them as dependencies", `
    workspace:verify-gate workspace:verify-layering workspace:verify-canonical-architecture workspace:verify-build-owners-sync s-gis-services-native:fixture-ownership-check
    s-gis-services-native:canonical-pair-composition-check hub-puzzle-retained-test:test-native hub-space-document-sources-test:test-native os-hub:trusted-catalog-preflight os-hub:runtime-fixture-graph
    s-rebuild:rebuild-all`),
  g("order", "name", "pure ordering task over its dependencies", "workspace:build workspace:cpp"),

  g("host", "name", "provisions, installs, repairs or prunes tools, dependencies, caches or images outside the declared outputs", `
    energy-oracle-py:oracle-setup energy-oracle-py:deps  ui-styling-py:deps ui-styling-dotnet:deps repo-test-dotnet:deps ui-styling-tokens:fonts
        workspace:setup-devcontainer-start workspace:setup-devcontainer-attach
    workspace:deps-javascript workspace:deps-python workspace:deps-cargo workspace:deps-go workspace:deps-dotnet workspace:deps-cpp workspace:deps-browsers workspace:deps-wasm workspace:deps-trunk
    workspace:deps-wasm-opt workspace:deps-tools workspace:deps-cargo-lock workspace:deps-js-lock workspace:deps-js-contract workspace:deps-js-all workspace:deps-build-tools workspace:cargo-prepare
    workspace:cpp-setup print:deps-tectonic print:deps-tex os-hub:setup os-hub:trusted-catalog-bootstrap os-hub-ts:backend-up os-hub-ts:backend-down os-hub-ts:docker-image-build
    s-rebuild:guest-framework-check architecture-course:ci-browser-prepare architecture-course:docker-image-build mit-bestand-demonstrator:prepare-e2e semio-tech-play:prepare-e2e gis-gismap-js:tiles-prefetch
    repo:cache-prune repo:cache-prune-stale-incremental repo:cargo-provenance-repair repo:cargo-prepare repo:cargo-synchronize repo-mcp:install repo-dashboard-rs:install repo-developer-tools:setup-debugger
        `),
  g("workspace", "name", "regenerates, formats, applies or scaffolds tracked files the target does not declare as outputs", `
     norm-en1996-rs:regenerate-example-assets remodel-remodeling-rs:composition-prepare energy-model-rs:lock 
     cad-js:fixture remodel-plugin:regenerate-example   
        
       os-hub:hostile-input-check 
    plugin-registry:rebuild-all plugin-registry:surface-schema plugin-registry:new os-plugin-describe-rs:describe print-rs:produce-logical-owner repo-lib:workspaces-write 
    cargo-workspaces:members-write workspace:agents-write workspace:generate-config   workspace:format workspace:test-report
    workspace:clean-test  workspace:clean-marker-only-folders workspace:clean-taxonomy-inventory workspace:clean-taxonomy-plan workspace:clean-taxonomy-apply
    workspace:clean-taxonomy-verify  workspace:stdio-fuzz workspace:verify-typescript-project  workspace:build-canvas-font-assets
    workspace:components repo-test-domain:test-clean repo-test-domain:test-fixture-generate
    repo-test-domain:test-fixture-reproduce repo-test-domain:test-gc`),
  g("workspace", "analysis", "the handler call graph writes tracked or generated workspace files (strong, workspace-anchored write sites) the target does not declare as outputs", `
    stdio-plugin:artifact-directory-wiring-generate stdio-plugin:subset-directory-wiring-generate stdio-plugin:native-codec-projection stdio-plugin:catalog-root
    stdio-plugin:home-io-surface-check stdio-plugin:editor-component-check stdio-plugin:composition-prepare stdio-plugin:parent-removal-test-check
    stdio-plugin:parent-removal-check stdio-semio-rs:composition-prepare cad-js:generate fixture-generator-stdio-jpg-jfif-1-01-document:🧪️witness
    os-hub:io-matrix architect-program-rs:generate-register-catalog hub-stdio-publication:publish workspace:schema-parsers-apply
    workspace:schema-parsers-restate workspace:new-taxonomy-mutation workspace:clean-coverage workspace:runtime-fixture-graph
    repo-lib:schema-read-inventory`),
  g("host", "analysis", "the handler call graph writes host stores, installs tools or rebuilds the pinned binary (workspace-anchored write sites outside the declared outputs)", `
    workspace:setup workspace:setup-storybook workspace:setup-git workspace:setup-native
    leutwiler-realparts-of-powers-z-n:deps proctor:rebuild proctor:backup proctor:restore
    proctor:erase proctor:prune`),
  g("remote", "name", "publishes a release, image or package to a remote", "os-hub:publish architecture-course:publish architecture-course:docker-image-publish framework-os-mcp-rs:publish workspace:publish workspace:release"),
  g("serve", "name", "runs until stopped: a server, watcher, shell, debugger or terminal user interface", `
    trinity-jack-shell:run repo-mcp-rs:run repo-coordinator-rs:run repo-cli-rs:repo repo-cli-rs:mcp repo-dashboard-rs:run repo-dashboard-rs:preferences repo-developer-tools:terminal repo-developer-tools:debug
    repo-developer-tools:debuggers repo-lib:ui-storybook-dev assets:logo-dev workspace:os workspace:semio os-hub:scoped-presence-browser-serve os-hub:local-hub-owner os-hub-ts:residency-watch os-hub-ts:boot-watch`),
];

const norm = (id: string) => id.replace(/^@semio-tech\//, "").replace(/^@teaching\//, "").replace(/^@leutwiler\//, "leutwiler-");
const SKIP = new Set(["energy-model-rs:test-bestest-regenerate"]);

if (import.meta.main) {
  const rows: any[] = JSON.parse(readFileSync(join(G, "rows.json"), "utf8"));
  const todo = rows.filter((r) => !r.newCache && !r.newDecision && !SKIP.has(norm(r.id)));
  const byId = new Map(todo.map((r) => [norm(r.id), r]));
  const assigned = new Map<string, Group>(), problems: string[] = [];
  for (const group of GROUPS) for (const id of group.ids) {
    if (!byId.has(id)) problems.push(`no such target: ${id}`);
    else if (assigned.has(id)) problems.push(`assigned twice: ${id}`);
    else assigned.set(id, group);
  }
  for (const id of byId.keys()) if (!assigned.has(id)) problems.push(`unassigned: ${id}`);
  if (problems.length) { console.error(problems.join("\n")); process.exit(1); }
  const out: Row[] = [];
  for (const [id, group] of assigned) {
    const row = byId.get(id)!;
    out.push({ id: row.id, manifest: row.manifest, target: row.target, disposition: group.treatment === "check" ? "cached" : group.treatment === "serve" ? "interactive" : "uncached", decision: D[group.treatment], basis: group.basis, note: group.note });
  }
  out.sort((a, b) => (a.id < b.id ? -1 : a.id > b.id ? 1 : 0));
  writeFileSync(join(import.meta.dir, "classification.json"), JSON.stringify(out, null, 1) + "\n");
  const count: Record<string, number> = {};
  for (const r of out) { const k = `${r.disposition} ${r.decision.class}/${r.decision.reason}`; count[k] = (count[k] ?? 0) + 1; }
  console.log(out.length, "rows", count);
}
