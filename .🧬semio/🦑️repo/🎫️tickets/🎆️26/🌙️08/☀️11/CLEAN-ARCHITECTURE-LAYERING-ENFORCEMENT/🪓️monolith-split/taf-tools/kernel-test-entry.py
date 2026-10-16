import re
p = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts"
t = open(p, encoding="utf8", newline="").read()
head_old = 'import { fixtureOriginalWorkStock } from "../../../🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";\ntype TestSource = { readonly directory: string; readonly url: string };\n'
assert t.count(head_old) == 1
head_new = '''/** 📡️ The kernel children's behaviour suites: the turn outcome broadcast, app router, activation registry, plugin registry, io router and plugin sources. This module is a vitest test file that imports each child and registers its suite; no child imports it back. */
import * as vitest from "vitest";
import { fileURLToPath } from "node:url";
import { fixtureOriginalWorkStock } from "../../../🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
import { ShardClient } from "../../../🎭️actor/📮️shard-client/🟦️.ts";
import { dialectCoordinate } from "../../../🚪️io/🧬️schema/🟦️.ts";
import { OwnedResidentLedger } from "../../../🌱️value/💾️resident/🟦️.ts";
import { ActivationRegistry, intersectCapabilityGrants } from "../../🎛️activation-registry/🟦️.ts";
import { AppRouter } from "../../🧭️app-router/🟦️.ts";
import { IoEntryGraph, ioIdentify, ioRun } from "../../🚪️io-router/🟦️.ts";
import { DEFAULT_MAX_RESIDENT_ACTORS, residentActorCapFromMemory } from "../../🧯️memory-pressure/🟦️.ts";
import { createBundledPluginSource, createDevPluginSource, createExtensionSource } from "../../🔌️plugin-source/🟦️.ts";
import { expandPluginRegistry } from "../../🗂️plugin-registry/🟦️.ts";
import { RUNTIME_METRICS_PUBLISH_INTERVAL_MS, runtimeMetricsDue } from "../../⏱️runtime-metrics/🟦️.ts";
import { createTurnOutcomeBroadcast } from "../../📡️turn-outcome-broadcast/🟦️.ts";

type Vitest = typeof import("vitest");
type TestSource = { readonly directory: string; readonly url: string };
'''
t = t.replace(head_old, head_new)
n = t.count('export async function registerTests')
assert n == 6, n
t = t.replace('export async function registerTests', 'async function registerTests')
m = t.count('vitest: NonNullable<ImportMeta["vitest"]>')
assert m == 6, m
t = t.replace('vitest: NonNullable<ImportMeta["vitest"]>', 'vitest: Vitest')
assert t.endswith("}\n") or t.endswith("}")
if not t.endswith("\n"): t += "\n"
tail = '''
const source: TestSource = { directory: fileURLToPath(new URL("../..", import.meta.url)), url: import.meta.url };
await registerTests1(vitest, { createTurnOutcomeBroadcast }, source);
await registerTests2(vitest, { AppRouter, dialectCoordinate }, source);
await registerTests3(vitest, { ActivationRegistry, DEFAULT_MAX_RESIDENT_ACTORS, OwnedResidentLedger, RUNTIME_METRICS_PUBLISH_INTERVAL_MS, ShardClient, intersectCapabilityGrants, residentActorCapFromMemory, runtimeMetricsDue }, source);
await registerTests4(vitest, { expandPluginRegistry }, source);
await registerTests5(vitest, { IoEntryGraph, dialectCoordinate, ioIdentify, ioRun }, source);
await registerTests6(vitest, { createBundledPluginSource, createDevPluginSource, createExtensionSource }, source);
'''
t += tail
open(p, "w", encoding="utf8", newline="").write(t)
print("ok")
