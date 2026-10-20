/** 🧬 The library edits that make an uncached target's reason part of the neutral cache schema and its law: four reasons the owner decisions need, the `uncached-without-reason` rule of the cache-correctness law, and the language-agnostic corpus rows that pin both.
 * Every edit is anchored on text the cache-fix set creates or edits, so the series is: cache-fix `apply.sh`, then this set's `apply.sh`. An edit is idempotent: it is skipped when its result is already present.
 */
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const I = `${L}/🏗️build/🕸️graph/📥️inference`;
const LAW = `${L}/⚖️laws/🔒️cache-correctness`;

export type Edit = { old: string; new: string } | { id: string; fn: (text: string) => string };
const swap = (old: string, next: string): Edit => ({ old, new: next });

const records = (id: string, target: object, authoredCache: boolean | null) => ({
  id, root: "lib/pkg", authoredCache, target,
  namedInputs: { default: ["{projectRoot}/**/*", "sharedGlobals"], sharedGlobals: [{ runtime: "bun --version" }, { env: "SEMIO_BUILD_MODE" }], production: ["{projectRoot}/**/*", "!{projectRoot}/**/🧪️tests/**/*", "sharedGlobals"], testEnv: [{ env: "SEMIO_TEST_LEVEL" }] },
});
const decision = (klass: string, reason: string) => ({ metadata: { semio: { cache: { version: 1, class: klass, reason } } } });

const corpusEdit = (text: string): string => {
  const corpus = JSON.parse(text);
  if (JSON.stringify(corpus, null, 2) + "\n" !== text) throw Error("cache-correctness corpus is not in canonical JSON form");
  const reasoned = (id: string, record: string, mutate: (target: any) => void) => { const row = corpus.cases.find((c: any) => c.id === id)?.records.find((r: any) => r.id === record); if (!row) throw Error(`corpus case missing: ${id}`); mutate(row.target); };
  reasoned("overlap-of-an-uncached-target-is-fine", "b:gen", (target) => { target.metadata ??= decision("side-effecting", "mutates-workspace").metadata; });
  reasoned("uncached-mutating-command-is-fine", "p:fix", (target) => { target.metadata ??= decision("side-effecting", "mutates-workspace").metadata; });
  const cases = [
    { id: "uncached-with-a-reason-is-fine", records: [records("p:install", { cache: false, ...decision("side-effecting", "mutates-host-store") }, false)], expect: [] },
    { id: "uncached-without-a-reason", records: [records("p:install", { cache: false, outputs: [] }, false)], expect: [["p:install", "uncached-without-reason"]] },
    { id: "interactive-with-a-reason-is-fine", records: [records("p:serve", { cache: false, continuous: true, ...decision("continuous", "serves-until-stopped") }, false)], expect: [] },
    { id: "live-resource-test-with-a-reason-is-fine", records: [records("p:test-leases", { cache: false, outputs: [], ...decision("live", "exercises-host-resources") }, false)], expect: [] },
  ];
  for (const row of cases) if (!corpus.cases.some((c: any) => c.id === row.id)) corpus.cases.push(row);
  return JSON.stringify(corpus, null, 2) + "\n";
};

const actionsEdit = (text: string): string => {
  const fixture = JSON.parse(text);
  if (JSON.stringify(fixture, null, 2) + "\n" !== text) throw Error("bazel actions fixture is not in canonical JSON form");
  const row = (name: string, task: object, reason: string) => ({ name, task, skipCache: false, valid: true, expected: { cache: false, reason } });
  const rows = [
    row("host-store-uncached", { cache: false, outputs: [], ...decision("side-effecting", "mutates-host-store") }, "side-effecting"),
    row("host-resources-uncached", { cache: false, outputs: [], ...decision("live", "exercises-host-resources") }, "live"),
    row("caller-input-uncached", { cache: false, outputs: [], ...decision("diagnostic", "reads-caller-input") }, "diagnostic"),
    row("reentering-verb-uncached", { cache: false, outputs: [], ...decision("aggregate", "reenters-build-tool") }, "aggregate"),
    row("interactive-with-a-reason", { cache: false, continuous: true, ...decision("continuous", "serves-until-stopped") }, "continuous"),
  ];
  for (const entry of rows) if (!fixture.classDecisions.some((c: any) => c.name === entry.name)) fixture.classDecisions.push(entry);
  return JSON.stringify(fixture, null, 2) + "\n";
};

export const EDITS: Record<string, Edit[]> = {
  [`${L}/🏗️build/🏃️execution/🧬️schema/💾️cache.json`]: [
    swap('"description": "Mutates state outside its declared outputs.",', '"description": "Mutates state outside its declared outputs: the workspace, a tool, dependency or cache store of this machine, or a remote.",'),
    swap('"reason": { "enum": ["publishes-live-state", "publishes-remote", "mutates-workspace"] }', '"reason": { "enum": ["publishes-live-state", "publishes-remote", "mutates-workspace", "mutates-host-store"] }'),
    swap('"description": "Observes or drives a running process.",', '"description": "Observes or drives a running process, or exercises the real locks, storage, process tree or clock of this machine.",'),
    swap('"reason": { "enum": ["drives-running-process"] }', '"reason": { "enum": ["drives-running-process", "exercises-host-resources"] }'),
    swap('"description": "Orders prerequisites and does no work of its own.",', '"description": "Orders prerequisites and does no work of its own; a verb that re-enters the build tool for other targets orders them the same way and should declare them as dependencies.",'),
    swap('"reason": { "enum": ["orders-prerequisites"] }', '"reason": { "enum": ["orders-prerequisites", "reenters-build-tool"] }'),
    swap('"description": "Reports a refusal or a measurement and is always re-evaluated.",', '"description": "Reports a refusal or a measurement, or answers the arguments and standard input its caller supplies, and is always re-evaluated.",'),
    swap('"reason": { "enum": ["admission-refusal", "measures-environment"] }', '"reason": { "enum": ["admission-refusal", "measures-environment", "reads-caller-input"] }'),
  ],
  [`${LAW}/🟦️.ts`]: [
    swap('"command-mutating", "command-network"] as const;', '"command-mutating", "command-network", "uncached-without-reason"] as const;'),
    swap('  if (decision?.class === "unclassified") found("unclassified-target", "no policy row, declaration or verifying command classifies this target");\n', '  if (decision?.class === "unclassified") found("unclassified-target", "no policy row, declaration or verifying command classifies this target");\n  if (target.cache === false && decision === undefined) found("uncached-without-reason", "cache false records no decision: declare metadata.semio.cache with the class and reason that forbid replay");\n'),
  ],
  [`${LAW}/🧪️tests/🔒️rules/🧬️schema/🔣️.json`]: [
    swap('"command-mutating", "command-network"] }]', '"command-mutating", "command-network", "uncached-without-reason"] }]'),
  ],
  [`${LAW}/🧪️tests/🔒️rules/🔣️.json`]: [{ id: "uncached-without-reason", fn: corpusEdit }],
  [`${L}/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🔣️.json`]: [{ id: "uncached-reasons", fn: actionsEdit }],
  [`${I}/🧪️tests/💾️cache/🔣️.json`]: [
    swap('    }\n  ],\n  "policy": [', `    },\n${[
      ["mutates-host-store", "side-effecting", "mutates-host-store", true], ["exercises-host-resources", "live", "exercises-host-resources", true], ["reenters-build-tool", "aggregate", "reenters-build-tool", true],
      ["reads-caller-input", "diagnostic", "reads-caller-input", true], ["host-resources-under-a-foreign-class", "side-effecting", "exercises-host-resources", false], ["caller-input-under-a-foreign-class", "live", "reads-caller-input", false],
    ].map(([id, klass, reason, valid]) => `    {\n      "id": ${JSON.stringify(id)},\n      "value": {\n        "version": 1,\n        "class": ${JSON.stringify(klass)},\n        "reason": ${JSON.stringify(reason)}\n      },\n      "valid": ${valid}\n    }`).join(",\n")}\n  ],\n  "policy": [`),
  ],
};
