/** 📏️ GR23 probe: the production path of a pool child for the closure-graph answers (one `closureGraphAnswerV1` per walk question, every question its own pass) over a sample of the wave's project owners, cold (machine state in a scratch directory). Env: WAVE (dir with session.bin), SAMPLE (owners), OUT (summary json). Prints rss/heap/peak, graph nodes and distinct witness bases. */
import { readFileSync, writeFileSync } from "node:fs"; import { join } from "node:path"; import { deserialize } from "node:v8";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/", SI = INF + "🧭️script-inputs/";
const { WAVE, SAMPLE = "80", OUT } = process.env, MB = 1048576, out = { steps: [] };
const peak = () => Math.round(process.resourceUsage().maxRSS / 1024), t0 = performance.now();
const mark = (label, extra = {}) => { const row = { label, s: +((performance.now() - t0) / 1000).toFixed(1), rssMB: Math.round(process.memoryUsage().rss / MB), heapMB: Math.round(process.memoryUsage().heapUsed / MB), peakMB: peak(), ...extra }; out.steps.push(row); console.error("[GR23] " + JSON.stringify(row)); };
const inference = await import(INF + "🟨️.mjs"); await inference.libraryBootstrap; const si = await import(SI + "🟨️.mjs");
const session = deserialize(readFileSync(WAVE + "/session.bin")), registry = session.registry.owners;
let owners = registry.map(owner => { const manifest = String(owner.manifest ?? ""); if (manifest.endsWith("Cargo.toml")) return undefined; try { return { ...owner, json: JSON.parse(readFileSync(join(R, manifest), "utf8")) }; } catch { return undefined; } }).filter(Boolean);
const n = Number(SAMPLE), step = Math.max(1, Math.floor(owners.length / n)); owners = owners.filter((_, at) => at % step === 0).slice(0, n);
const cache = inference.createScriptInputCache({ workspaceRoot: R });
const questions = []; for (const owner of owners) for (const target of Object.values(owner.json.targets ?? {})) { try { const q = await si.targetClosureQuestionV1(target, R, cache, owner.root); if (q) questions.push(q); } catch {} }
mark("questions", { owners: owners.length, questions: questions.length });
let failed = 0; for (const [at, question] of questions.entries()) { try { await si.closureGraphAnswerV1(cache, R, question); } catch { failed++; } if (at % 25 === 24) mark("answers", { done: at + 1 }); }
const graph = cache.closureGraph, bases = new Set(); let nodes = 0, typed = 0; for (const record of graph?.records() ?? []) { nodes++; const w = record.extra?.witness; if (w?.base) bases.add(w.base); typed += w?.typed?.length ?? 0; }
let baseTyped = 0; for (const b of bases) baseTyped += b.typed?.length ?? 0;
mark("end", { failed, nodes, bases: bases.size, ownTyped: typed, baseTyped, walkNodes: cache.walkNodes?.size });
if (OUT) writeFileSync(OUT, JSON.stringify(out, null, 1));
process.exit(0);
