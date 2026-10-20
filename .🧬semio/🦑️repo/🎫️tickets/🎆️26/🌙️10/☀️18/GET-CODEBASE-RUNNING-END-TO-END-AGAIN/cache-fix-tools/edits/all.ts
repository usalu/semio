import { readFileSync } from "node:fs";
import { join } from "node:path";

export type Edit = { old: string; new: string };
const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const I = `${L}/🏗️build/🕸️graph/📥️inference`;
const B = `${L}/🏗️build/🔧️tools/🏗️bazel`;
const rows = JSON.parse(readFileSync(join(import.meta.dir, "../rows.json"), "utf8"));

const list = (items: string[], indent: string) => `[\n${items.map((item) => `${indent}  ${JSON.stringify(item)}`).join(",\n")}\n${indent}]`;
const renderRow = (row: any) => {
  const parts = [`      "decision": { "class": ${JSON.stringify(row.decision.class)}, "reason": ${JSON.stringify(row.decision.reason)} }`];
  for (const key of ["exact", "heads", "tails", "tokens"]) if (row[key]) parts.push(`      ${JSON.stringify(key)}: ${row[key].length > 6 ? list(row[key], "      ") : JSON.stringify(row[key]).replaceAll(",", ", ")}`);
  return `    {\n${parts.join(",\n")}\n    }`;
};
const renderedRows = `  "targetClasses": [\n${rows.map(renderRow).join(",\n")}\n  ],\n`;

const targetPolicyOldFamily = `export const cacheableFamily = (name) => /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) || /(?:^|-)contract(?:-|$)/.test(name) || /^generator(?:-|$)/.test(name) && !name.includes("generator-inputs");
`;
const targetPolicyNewFamily = `const hasHead = (name, head) => name === head || name.startsWith(\`\${head}-\`);
const hasTail = (name, tail) => name === tail || name.endsWith(\`-\${tail}\`);
const hasToken = (name, token) => hasHead(name, token) || hasTail(name, token) || name.includes(\`-\${token}-\`);

/** 🔤️ Strips the leading symbols (an emoji prefix) of a target name, so every name rule reads \`🏭️generator-build\` exactly as \`generator-build\`. */
export const normalizedTargetName = (name) => name.replace(/^[^A-Za-z0-9]+/u, "");

/** 🗂️ Resolves the cache class the workspace policy declares for one target name: a row that names the name exactly decides first, then the first row with a matching head, tail or token; a name no row classifies has none and never replays.
 * @see ../../../../⚡️caching/🧬️schema/🔣️.json */
export function targetClassV1(name, rows = POLICY.targetClasses) {
  const bare = normalizedTargetName(name);
  const row = rows.find((row) => row.exact?.includes(bare)) ?? rows.find((row) => row.heads?.some((head) => hasHead(bare, head)) || row.tails?.some((tail) => hasTail(bare, tail)) || row.tokens?.some((token) => hasToken(bare, token)));
  return row && cacheDecisionV1(row.decision.class, row.decision.reason);
}

export const cacheableFamily = (name, rows) => targetClassV1(name, rows)?.class === "deterministic";

/** 📌️ Admits the cache decision a target declares itself in \`metadata.semio.cache\`; a malformed or open record is refused instead of ignored. */
function declaredCacheDecisionV1(value) {
  if (value === undefined) return undefined;
  const decision = cacheDecisionV1(value?.class, value?.reason);
  if (Object.keys(value).length !== Object.keys(decision).length || value.version !== decision.version) throw Error(\`Repository target cache decision refused: \${value?.class}/\${value?.reason}\`);
  return decision;
}
`;
const oldTargetPolicy = `/** 🛡️ Side effects and live processes cannot be replayed as completed task results. */
export function targetPolicy(name, target, policy = POLICY) {
  if (matchesCommand(name, policy.continuous)) return withCacheDecision({ ...target, cache: false, continuous: true }, "continuous", "serves-until-stopped");
  if (target.cache === false) return { inputs: ["default"], outputs: [], ...target, cache: false };
  if (matchesUncached(name, policy)) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(name));
  if (policy.cachedExact.includes(name)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  if (mutatingName(name) || liveName(name)) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(name));
  if (cacheableFamily(name) || verifyCommand(target)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  return { ...target, cache: target.cache !== false };
}
`;
const newTargetPolicy = `/** 🛡️ Side effects and live processes cannot be replayed as completed task results, and neither can a target nothing classifies: a target replays only when the owner declared it deterministic, the policy classes its name as deterministic or its command verifies. */
export function targetPolicy(name, target, policy = POLICY) {
  const bare = normalizedTargetName(name);
  if (matchesCommand(bare, policy.continuous)) return withCacheDecision({ ...target, cache: false, continuous: true }, "continuous", "serves-until-stopped");
  if (target.cache === false) { declaredCacheDecisionV1(target.metadata?.semio?.cache); return { inputs: ["default"], outputs: [], ...target, cache: false }; }
  if (matchesUncached(bare, policy)) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(bare));
  const exact = policy.cachedExact.includes(bare);
  if (!exact && (mutatingName(bare) || liveName(bare))) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(bare));
  const decision = declaredCacheDecisionV1(target.metadata?.semio?.cache) ?? targetClassV1(bare, policy.targetClasses) ?? (exact || verifyCommand(target) ? cacheDecisionV1("deterministic", "check-without-outputs") : undefined);
  if (!decision) return withCacheDecision({ ...target, cache: false }, "unclassified", "no-cache-classification");
  if (decision.class !== "deterministic") return withCacheDecision({ ...target, cache: false }, decision.class, decision.reason);
  return { inputs: ["default", "^default"], outputs: [], ...withCacheDecision(target, decision.class, decision.reason), cache: true };
}

/** 🚧️ A replayed producer must declare what it writes: a target classed \`pure-function-of-inputs\` that declares no output (and is neither a Rust build intent, a host-platform output declaration nor a pure ordering task) would replay as an empty success on a clean checkout, so it stops being replayable instead. */
export function cachedOutputsAdmittedV1(target) {
  const decision = target.metadata?.semio?.cache;
  if (target.cache !== true || decision?.class !== "deterministic" || decision.reason !== "pure-function-of-inputs") return target;
  const semio = target.metadata.semio;
  if ((Array.isArray(target.outputs) && target.outputs.length > 0) || semio.rustBuildIntent !== undefined || semio.rustUnitAggregate !== undefined || semio.hostPlatformOutputsV1 !== undefined || target.executor === "repo:noop") return target;
  return withCacheDecision({ ...target, cache: false }, "side-effecting", "mutates-workspace");
}
`;

export const EDITS: Record<string, Edit[]> = {
  [`${I}/🎯️target-policy/🟨️.mjs`]: [
    { old: targetPolicyOldFamily, new: targetPolicyNewFamily },
    { old: oldTargetPolicy, new: newTargetPolicy },
  ],
  [`${I}/🧱️project-assembly/🟨️.mjs`]: [
    { old: "withWasmTooling, withCacheDecision, rootCommandTargets, cargoTargets,", new: "withWasmTooling, withCacheDecision, cachedOutputsAdmittedV1, rootCommandTargets, cargoTargets," },
    { old: "({ withWasmTooling, withCacheDecision, targetWithDefaults, targetPolicy, targetExecutionKindV1 } = registry.targetPolicy);", new: "({ withWasmTooling, withCacheDecision, cachedOutputsAdmittedV1, targetWithDefaults, targetPolicy, targetExecutionKindV1 } = registry.targetPolicy);" },
    { old: "normalized[name] = loaded.targetBuildModes(name, policy);", new: "normalized[name] = loaded.targetBuildModes(name, cachedOutputsAdmittedV1(policy));" },
  ],
  [`${I}/🧰️internals/🟨️.mjs`]: [
    { old: "moduleSourceRecordCacheRootV1, buildTrackedSourceFile, walkCargoToml;\nexport let cacheInternals;", new: "moduleSourceRecordCacheRootV1, buildTrackedSourceFile, walkCargoToml, targetClassV1, normalizedTargetName, cachedOutputsAdmittedV1;\nexport let cacheInternals;" },
    { old: "matchesUncached, cacheableFamily, mutatingName, liveName, verifyCommand } = registry.targetPolicy);", new: "matchesUncached, cacheableFamily, mutatingName, liveName, verifyCommand, targetClassV1, normalizedTargetName, cachedOutputsAdmittedV1 } = registry.targetPolicy);" },
    { old: "moduleSourceRecordCacheRootV1, buildTrackedSourceFile, walkCargoToml };\n}", new: "moduleSourceRecordCacheRootV1, buildTrackedSourceFile, walkCargoToml, targetClassV1, normalizedTargetName, cachedOutputsAdmittedV1 };\n}" },
  ],
  [`${I}/🟨️.d.mts`]: [
    { old: " cacheDecisionV1(decisionClass:string,reason:string):Readonly<{version:1;class:string;reason:string}>;\n", new: " cacheDecisionV1(decisionClass:string,reason:string):Readonly<{version:1;class:string;reason:string}>;\n targetClassV1(name:string,rows?:InferencePolicyV1[\"targetClasses\"]):Readonly<{version:1;class:string;reason:string}>|undefined;\n normalizedTargetName(name:string):string;\n cachedOutputsAdmittedV1<T extends object>(target:T):T;\n" },
  ],
  [`${L}/🏗️build/🏃️execution/🧬️schema/💾️cache.json`]: [
    { old: `    { "$ref": "#/definitions/Diagnostic" }\n  ],`, new: `    { "$ref": "#/definitions/Diagnostic" },\n    { "$ref": "#/definitions/Unclassified" }\n  ],` },
    { old: `        "reason": { "enum": ["admission-refusal", "measures-environment"] }\n      }\n    }\n  }\n}`, new: `        "reason": { "enum": ["admission-refusal", "measures-environment"] }\n      }\n    },\n    "Unclassified": {\n      "description": "No policy row, declaration or verifying command classifies the target, so it never replays until the workspace policy names its kind.",\n      "type": "object",\n      "additionalProperties": false,\n      "required": ["version", "class", "reason"],\n      "properties": {\n        "version": { "const": 1 },\n        "class": { "const": "unclassified" },\n        "reason": { "enum": ["no-cache-classification"] }\n      }\n    }\n  }\n}` },
  ],
  [`${L}/⚡️caching/🔣️policy.json`]: [
    { old: `    "hub-live-catalog-oracle"\n  ],\n  "continuous": [`, new: `    "hub-live-catalog-oracle"\n  ],\n${renderedRows}  "continuous": [` },
  ],
  [`${L}/⚡️caching/🧬️schema/🔣️.json`]: [
    { old: `    "cachedExact",\n    "continuous",\n    "generatedDirectories",`, new: `    "cachedExact",\n    "targetClasses",\n    "continuous",\n    "generatedDirectories",` },
    { old: `    "continuous": {\n      "type": "array",`, new: `    "targetClasses": {
      "description": "Ordered, closed classification of target names into cache classes; a leading emoji or other symbol prefix never hides a name from its row. A row that lists the name in \`exact\` decides before any head, tail or token of any row, then the first row with a matching head (\`<head>\` or \`<head>-…\`), tail (\`<tail>\` or \`…-<tail>\`) or token (a hyphen-delimited segment) decides. Only \`deterministic\` rows replay; a target no row, declaration or verifying command classifies is \`unclassified\` and never replays.",
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["decision"],
        "properties": {
          "decision": {
            "type": "object",
            "additionalProperties": false,
            "required": ["class", "reason"],
            "properties": { "class": { "type": "string" }, "reason": { "type": "string" } }
          },
          "exact": { "type": "array", "items": { "type": "string", "pattern": "^[A-Za-z0-9][A-Za-z0-9._-]*$" }, "uniqueItems": true },
          "heads": { "type": "array", "items": { "type": "string", "pattern": "^[A-Za-z0-9][A-Za-z0-9._-]*$" }, "uniqueItems": true },
          "tails": { "type": "array", "items": { "type": "string", "pattern": "^[A-Za-z0-9][A-Za-z0-9._-]*$" }, "uniqueItems": true },
          "tokens": { "type": "array", "items": { "type": "string", "pattern": "^[A-Za-z0-9][A-Za-z0-9._-]*$" }, "uniqueItems": true }
        },
        "anyOf": [{ "required": ["exact"] }, { "required": ["heads"] }, { "required": ["tails"] }, { "required": ["tokens"] }]
      }
    },
    "continuous": {\n      "type": "array",` },
  ],
};


const cacheTests = `${I}/🧪️tests/💾️cache`;
const decisionRow = (id: string, klass: string, reason: string, valid: boolean) => `    {\n      "id": ${JSON.stringify(id)},\n      "value": {\n        "version": 1,\n        "class": ${JSON.stringify(klass)},\n        "reason": ${JSON.stringify(reason)}\n      },\n      "valid": ${valid}\n    }`;
const policyRow = (name: string, decision: [string, string] | null, cache: boolean, extra: { authored?: false; continuous?: true; target?: object } = {}) =>
  `    {\n      "name": ${JSON.stringify(name)},\n${extra.authored === false ? '      "authored": false,\n' : ""}${extra.target ? `      "target": ${JSON.stringify(extra.target)},\n` : ""}      "decision": ${decision ? `[\n        ${JSON.stringify(decision[0])},\n        ${JSON.stringify(decision[1])}\n      ]` : "null"},\n      "cache": ${cache}${extra.continuous ? ',\n      "continuous": true' : ""}\n    }`;
const gateRow = (id: string, name: string, target: object, cache: boolean, decision: [string, string] | null) =>
  `    {\n      "id": ${JSON.stringify(id)},\n      "name": ${JSON.stringify(name)},\n      "target": ${JSON.stringify(target)},\n      "expect": {\n        "cache": ${cache},\n        "decision": ${decision ? `[\n          ${JSON.stringify(decision[0])},\n          ${JSON.stringify(decision[1])}\n        ]` : "null"}\n      }\n    }`;
const CHECK: [string, string] = ["deterministic", "check-without-outputs"], PURE: [string, string] = ["deterministic", "pure-function-of-inputs"], MUTATES: [string, string] = ["side-effecting", "mutates-workspace"], UNCLASSIFIED: [string, string] = ["unclassified", "no-cache-classification"];
const declared = (klass: string, reason: string) => ({ metadata: { semio: { cache: { version: 1, class: klass, reason } } } });

EDITS[`${cacheTests}/🔣️.json`] = [
  { old: `    {\n      "id": "missing-reason",\n      "value": {\n        "version": 1,\n        "class": "continuous"\n      },\n      "valid": false\n    }\n  ],\n  "policy": [`, new: `    {\n      "id": "missing-reason",\n      "value": {\n        "version": 1,\n        "class": "continuous"\n      },\n      "valid": false\n    },\n${decisionRow("unclassified", "unclassified", "no-cache-classification", true)},\n${decisionRow("unclassified-with-a-foreign-reason", "unclassified", "measures-environment", false)}\n  ],\n  "policy": [` },
  { old: `    {\n      "name": "test",\n      "decision": null,\n      "cache": true\n    },\n    {\n      "name": "build",\n      "decision": null,\n      "cache": true\n    },\n    {\n      "name": "check",\n      "authored": false,\n      "decision": null,\n      "cache": false\n    }\n  ],\n  "nativeRunsTests": [`, new: [
    policyRow("test", CHECK, true), policyRow("build", PURE, true), policyRow("check", null, false, { authored: false }),
    policyRow("🧪️test", CHECK, true), policyRow("🏭️generator-build", MUTATES, false), policyRow("⚙️generator-manifests", MUTATES, false),
    policyRow("🚀️serve", ["continuous", "serves-until-stopped"], false, { continuous: true }), policyRow("🧹️clean", MUTATES, false),
    policyRow("lockfile-check", CHECK, true), policyRow("forms-document-contract", CHECK, true), policyRow("generator-inputs", PURE, true),
    policyRow("prepare-prototype", ["driver-owned", "freshness-owned-by-driver"], false), policyRow("acceptance-goal", ["live", "drives-running-process"], false),
    policyRow("unheard-of-kind", UNCLASSIFIED, false), policyRow("🦄️unheard-of-kind", UNCLASSIFIED, false),
    policyRow("unheard-of-kind", UNCLASSIFIED, false, { target: { cache: true } }),
    policyRow("catalog-smoke", CHECK, true, { target: { options: { command: "bun ./📜️script.ts verify catalog" } } }),
    policyRow("unheard-of-kind", PURE, true, { target: declared("deterministic", "pure-function-of-inputs") }),
    policyRow("test", ["driver-owned", "freshness-owned-by-driver"], false, { target: declared("driver-owned", "freshness-owned-by-driver") }),
  ].join(",\n").replace(/^/, "") + `\n  ],\n  "outputsGate": [\n${[
    gateRow("producer-with-outputs", "build", { outputs: ["{projectRoot}/dist"] }, true, PURE),
    gateRow("producer-with-empty-outputs", "build", { outputs: [] }, false, MUTATES),
    gateRow("producer-without-outputs", "generate", {}, false, MUTATES),
    gateRow("producer-with-rust-build-intent", "build", { outputs: [], metadata: { semio: { rustBuildIntent: { version: 1 } } } }, true, PURE),
    gateRow("producer-with-host-platform-outputs", "build", { outputs: [], metadata: { semio: { hostPlatformOutputsV1: { version: 1 } } } }, true, PURE),
    gateRow("ordering-task", "build", { executor: "repo:noop", outputs: [] }, true, PURE),
    gateRow("check-without-outputs", "test", { outputs: [] }, true, CHECK),
    gateRow("declared-producer-without-outputs", "unheard-of-kind", { ...declared("deterministic", "pure-function-of-inputs") }, false, MUTATES),
    gateRow("declared-producer-with-outputs", "unheard-of-kind", { outputs: ["{projectRoot}/out"], ...declared("deterministic", "pure-function-of-inputs") }, true, PURE),
    gateRow("unclassified-stays-uncached", "unheard-of-kind", { cache: true, outputs: ["{projectRoot}/out"] }, false, UNCLASSIFIED),
  ].join(",\n")}\n  ],\n  "nativeRunsTests": [` },
];
EDITS[`${cacheTests}/🧬️schema/🔣️.json`] = [
  { old: `    "policy",\n    "nativeRunsTests",`, new: `    "policy",\n    "outputsGate",\n    "nativeRunsTests",` },
  { old: `          "continuous": {\n            "type": "boolean"\n          }\n        }\n      }\n    },\n    "nativeRunsTests": {`, new: `          "continuous": {\n            "type": "boolean"\n          },\n          "target": {\n            "type": "object"\n          }\n        }\n      }\n    },\n    "outputsGate": {\n      "type": "array",\n      "minItems": 1,\n      "items": {\n        "type": "object",\n        "additionalProperties": false,\n        "required": ["id", "name", "target", "expect"],\n        "properties": {\n          "id": { "type": "string" },\n          "name": { "type": "string" },\n          "target": { "type": "object" },\n          "expect": {\n            "type": "object",\n            "additionalProperties": false,\n            "required": ["cache", "decision"],\n            "properties": {\n              "cache": { "type": "boolean" },\n              "decision": { "anyOf": [{ "type": "null" }, { "type": "array", "minItems": 2, "maxItems": 2, "items": { "type": "string" } }] }\n            }\n          }\n        }\n      }\n    },\n    "nativeRunsTests": {` },
];
EDITS[`${cacheTests}/🟦️.ts`] = [
  { old: `const target: any = api.targetPolicy(row.name, { cache: row.authored === false ? false : undefined, options: { command: "bun ./📜️script.ts " + row.name } });`, new: `const target: any = api.targetPolicy(row.name, { cache: row.authored === false ? false : undefined, ...structuredClone(row.target ?? {}), options: { command: "bun ./📜️script.ts " + row.name, ...(row.target as any)?.options } });` },
  { old: `test("native targets run tests by contract, by name or through the Cargo test driver", () => {`, new: `test("a replayed producer declares its outputs and a target nothing classifies never replays", () => {
  for (const row of corpus.outputsGate) {
    const target: any = api.cachedOutputsAdmittedV1(api.targetPolicy(row.name, structuredClone(row.target)));
    expect(target.cache, row.id).toBe(row.expect.cache);
    expect(decisionOf(target), row.id).toEqual(row.expect.decision);
    if (row.expect.decision) expect(validateDecision(target.metadata.semio.cache), row.id).toBe(true);
  }
  expect(() => api.targetPolicy("unheard-of-kind", { metadata: { semio: { cache: { version: 1, class: "cacheable", reason: "pure-function-of-inputs" } } } }), "an open declared class").toThrow("Repository target cache decision refused");
  expect(() => api.targetPolicy("unheard-of-kind", { metadata: { semio: { cache: { version: 1, class: "deterministic", reason: "pure-function-of-inputs", note: "x" } } } }), "a declared decision with an extra key").toThrow("Repository target cache decision refused");
});

test("the emoji prefix of a target name never hides it from its class", () => {
  for (const [name, bare] of [["🏭️generator-build", "generator-build"], ["⚙️generator", "generator"], ["🧪️test", "test"], ["plain", "plain"]]) expect(api.normalizedTargetName(name), name).toBe(bare);
  for (const name of ["generator-build", "🏭️generator-build", "⚙️generator-manifests"]) expect(decisionOf({ metadata: { semio: { cache: api.targetClassV1(name) } } }), name).toEqual(["side-effecting", "mutates-workspace"]);
  expect(api.targetClassV1("totally-unheard-of")).toBeUndefined();
  expect(api.targetClassV1("a-contract-b")?.reason).toBe("check-without-outputs");
});

test("native targets run tests by contract, by name or through the Cargo test driver", () => {` },
];

EDITS[`${B}/🧬️schema/🔣️.json`] = [
  { old: `                "aggregate",\n                "diagnostic"\n              ]`, new: `                "aggregate",\n                "diagnostic",\n                "unclassified"\n              ]` },
  { old: `            "diagnostic": {\n              "type": "integer",\n              "minimum": 1\n            }\n          }\n        },\n        "unbounded": {`, new: `            "diagnostic": {\n              "type": "integer",\n              "minimum": 1\n            },\n            "unclassified": {\n              "type": "integer",\n              "minimum": 1\n            }\n          }\n        },\n        "unbounded": {` },
];
EDITS[`${B}/📜️script.ts`] = [
  { old: `|"live"|"aggregate"|"diagnostic";`, new: `|"live"|"aggregate"|"diagnostic"|"unclassified";` },
];
EDITS[`${B}/🧪️tests/📥️actions/🔣️.json`] = [
  { old: `      "expected": {\n        "cache": false,\n        "reason": "diagnostic"\n      }\n    },`, new: `      "expected": {\n        "cache": false,\n        "reason": "diagnostic"\n      }\n    },\n    {\n      "name": "unclassified-uncached",\n      "task": {\n        "metadata": {\n          "semio": {\n            "cache": {\n              "version": 1,\n              "class": "unclassified",\n              "reason": "no-cache-classification"\n            }\n          }\n        }\n      },\n      "skipCache": false,\n      "valid": true,\n      "expected": {\n        "cache": false,\n        "reason": "unclassified"\n      }\n    },` },
];


const vector = (target: string, cache: boolean) => `    {\n      "target": ${JSON.stringify(target)},\n      "cache": ${cache},\n      "continuous": false,\n      "disabledCache": false\n    }`;
EDITS[`${L}/⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json`] = [
  { old: `      "command": "bun ./📜️script.ts verify portable",\n      "cache": true,\n      "disabledCache": false,\n      "continuous": false\n    }\n  ],\n  "pluginGraph": {`, new: `      "command": "bun ./📜️script.ts verify portable",\n      "cache": true,\n      "disabledCache": false,\n      "continuous": false\n    },\n${[vector("🧪️test", true), vector("🏭️generator-build", false), vector("⚙️generator-manifests", false), vector("unheard-of-kind", false), vector("prepare-prototype", false), vector("acceptance-goal", false)].join(",\n")}\n  ],\n  "pluginGraph": {` },
];
EDITS[`${L}/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`] = [
  { old: `    assert.equal(cacheInternals.cacheableFamily("generator-inputs"), false);`, new: `    assert.equal(cacheInternals.targetClassV1("generator-inputs").reason, "pure-function-of-inputs", "the generator input registry is a declared producer, not a fixture generator");\n    assert.equal(cacheInternals.cacheableFamily("🏭️generator-build"), false, "an emoji-prefixed fixture generator is classed like its bare name and never replays");` },
];

EDITS[`${L}/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts`] = [
  { old: `["wire","targetWithDefaults","targetPolicy","targetExecutionKindV1","withWasmTooling","withCacheDecision"]`, new: `["wire","targetWithDefaults","targetPolicy","targetExecutionKindV1","withWasmTooling","withCacheDecision","cachedOutputsAdmittedV1"]` },
];
export const CREATES: Record<string, string> = {
  [`${L}/⚖️laws/🔒️cache-correctness/🟦️.ts`]: readFileSync(join(import.meta.dir, "../law/🟦️.ts"), "utf8"),
  [`${L}/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts`]: readFileSync(join(import.meta.dir, "../law/🧪️tests/🔒️rules/🟦️.ts"), "utf8"),
  [`${L}/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🔣️.json`]: readFileSync(join(import.meta.dir, "../law/🧪️tests/🔒️rules/🔣️.json"), "utf8"),
  [`${L}/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🧬️schema/🔣️.json`]: readFileSync(join(import.meta.dir, "../law/🧪️tests/🔒️rules/🧬️schema/🔣️.json"), "utf8"),
};
