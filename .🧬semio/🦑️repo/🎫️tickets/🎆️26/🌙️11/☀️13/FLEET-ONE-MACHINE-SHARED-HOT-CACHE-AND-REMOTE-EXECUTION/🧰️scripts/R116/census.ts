// R116 census: is the fleet store converging? Read-only. Run with the repo's bun, niced:
//   cd /Users/ueli/Documents/semio && nice -n 10 .cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R116/census.ts"
// Env: R116_BUDGET seconds (default 118; the sample is cut when it ends), R116_N judged owners (default 300), R116_KEYED keyed owners (default 310), R116_WAVE wave directory name.
// About 60 s keying plus 50-75 s judging at load 16-25; at load 40 the budget cuts the sample (the header says how many were judged).
import { readFileSync, readdirSync, statSync, mkdtempSync, realpathSync, rmSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve, dirname } from "node:path";
import { deserialize } from "node:v8";

const root = "/Users/ueli/Documents/semio";
const B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const PN = B + "/🕸️graph/📥️inference/📦️project-nodes";
const LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const TICKET13 = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION";
const BUDGET = Number(process.env.R116_BUDGET ?? 118) * 1000, MAXJ = Number(process.env.R116_N ?? 300), KEYN = Number(process.env.R116_KEYED ?? 310);
const T0 = performance.now(), left = () => BUDGET - (performance.now() - T0), NOW = Date.now();
const iso = (ms: number) => new Date(ms).toISOString().slice(5, 19).replace("T", " ");
const out: string[] = [], say = (s = "") => { out.push(s); console.log(s); };
const pct = (a: number, b: number) => b === 0 ? "-" : (100 * a / b).toFixed(1) + "%";
const rel = (p: string) => p.startsWith(root + "/") ? p.slice(root.length + 1) : p;

const { libraryBootstrap } = await import(B + "/🕸️graph/📥️inference/🟨️.mjs"); await libraryBootstrap;
const { remoteCacheFetchV1 } = await import(B + "/🌐️remote-cache/🛰️fleet/🟦️.ts"); const { graphFleetSettingsV1 } = await import(B + "/🪪️graph-state/🧶️nx/🟦️.ts");
const X: any = await import(B + "/🪪️graph-state/🛰️exchange/🟦️.ts"); const R: any = await import(PN + "/🟨️.mjs");
const PR: any = await import(B + "/🪪️graph-state/📦️project-records/🟦️.ts"); const P: any = await import(B + "/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const OB: any = await import(B + "/👁️observation/🟨️.mjs"); const SI: any = await import(B + "/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const SA: any = await import(B + "/🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs");
const env: any = { ...process.env }; for (const k of Object.keys(env)) if (k.startsWith("SEMIO_REMOTE_CACHE")) delete env[k];
const client = X.graphFleetClientV1(graphFleetSettingsV1(root, env), { fetch: remoteCacheFetchV1(env, { root }) });

const wavesDir = root + "/.cache/f/owner-pool-waves";
const waves = readdirSync(wavesDir).filter(n => existsSync(join(wavesDir, n, "units.json")) && existsSync(join(wavesDir, n, "session.bin")) && existsSync(join(wavesDir, n, "head.json")))
  .map(n => ({ n, head: JSON.parse(readFileSync(join(wavesDir, n, "head.json"), "utf8")) })).sort((a, b) => b.head.createdMs - a.head.createdMs);
const sizeOf = (n: string) => statSync(join(wavesDir, n, "units.json")).size, biggest = Math.max(...waves.map(w => sizeOf(w.n)));
const wave = process.env.R116_WAVE ? waves.find(w => w.n === process.env.R116_WAVE)! : waves.find(w => sizeOf(w.n) >= 0.8 * biggest)!;
const smallWaves = waves.filter(w => sizeOf(w.n) < 0.8 * biggest).length;
const units: any[] = JSON.parse(readFileSync(join(wavesDir, wave.n, "units.json"), "utf8")), session = deserialize(readFileSync(join(wavesDir, wave.n, "session.bin")));
const base = mkdtempSync(join(realpathSync(tmpdir()), "semio-r116-"));
const service: any = await import(PN + "/🏊️project-farm/👷️service/🟨️.mjs");
await service.open({ ...session, workspaceRoot: root, closureStore: undefined, stagingBase: base, authority: undefined }, { slot: 0, generation: 1 });
const envr = service.sessionEnvironmentV1();
SI.bindClosureStoreV1(root, join(base, "ce"), true); SI.bindClosureScriptsV1(root, SI.createScriptInputCache({ lateClosures: true }));
const host = P.hostOfRootV1(root);
const providers = async () => [...Object.values(SA.prepareSourceAnalysesV1()).map((p: any) => p.id), ...SI.scriptProviderIdsV1()];
const generation = P.dependencyGenerationV1(root);

const landingsText = readFileSync(TICKET13 + "/landings.md", "utf8");
const landings: { id: string; start: number; end: number; label: string }[] = [];
for (const m of landingsText.matchAll(/^## (R\d+) landing \((\d{4}-\d\d-\d\d) (\d\d:\d\d(?::\d\d)?)(?: UTC)? to (?:about )?(\d\d:\d\d(?::\d\d)?)/gm)) {
  const t = (s: string) => s.length === 5 ? s + ":00" : s;
  const start = Date.parse(`${m[2]}T${t(m[3]!)}Z`); let end = Date.parse(`${m[2]}T${t(m[4]!)}Z`); if (end < start) end += 864e5;
  landings.push({ id: m[1]!, start, end, label: `${m[1]} ${iso(start).slice(6)}-${iso(end).slice(6)} UTC` });
}
landings.sort((a, b) => a.start - b.start); const last3 = landings.filter(l => l.start <= NOW).slice(-3);

const group = (u: any) => /language:(cargo|rust)/.test(u.kind) ? "rust/cargo" : u.kind === "language:typescript" ? "typescript" : "other";
const GROUPS = ["rust/cargo", "typescript", "other"];
let seed = 116; const rnd = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };
const lists: Record<string, any[]> = { "rust/cargo": [], typescript: [], other: [] }; for (const u of units) lists[group(u)]!.push(u);
const total: Record<string, number> = Object.fromEntries(GROUPS.map(g => [g, lists[g]!.length]));
for (const g of GROUPS) { const a = lists[g]!; for (let i = a.length - 1; i > 0; i--) { const j = Math.floor(rnd() * (i + 1)); [a[i], a[j]] = [a[j], a[i]]; } }
const order: any[] = []; { const cur: Record<string, number> = { "rust/cargo": 0, typescript: 0, other: 0 }; const pat = ["rust/cargo", "rust/cargo", "rust/cargo", "typescript", "other"];
  for (let live = true; live;) { live = false; for (const g of pat) { const a = lists[g]!; if (cur[g]! < a.length) { order.push(a[cur[g]!++]); live = true; } } } }

const claimsByAddress = new Map<string, { grantedMs: number; origin: string }[]>();
try { for (const line of readFileSync(root + "/.cache/f/cache-server/v1/claims.jsonl", "utf8").split("\n")) { if (!line.includes('"set"')) continue; let d: any; try { d = JSON.parse(line); } catch { continue; }
  if (d.t !== "set") continue; const a = claimsByAddress.get(d.address) ?? []; if (!a.some(x => x.grantedMs === d.grantedMs)) a.push({ grantedMs: d.grantedMs, origin: String(d.owner).split("|")[0]! }); claimsByAddress.set(d.address, a); } } catch {}
const acRoot = root + "/.cache/f/cache-server/v1/main/ac";

type Loc = { unit: any; grp: string; state: any; entryKey: string; mtimeMs?: number; claimOrigin?: string };
const locs: Loc[] = []; let keyErrors = 0;
const phaseAEnd = BUDGET * 0.55;
for (const unit of order) {
  if (locs.length >= KEYN || performance.now() - T0 > phaseAEnd) break;
  try {
    const job = R.ownerJobV1(envr, wave.head.globals, unit);
    const keyText = R.ownerRecordKeyTextV1(R.projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs));
    const state = PR.projectRecordStateV1({ root, fleet: job.root, keyText, platform: "darwin-arm64" });
    const entryKey = X.graphEntryKeyV1(state); let mtimeMs: number | undefined; try { mtimeMs = statSync(`${acRoot}/${entryKey.slice(0, 2)}/${entryKey}`).mtimeMs; } catch {}
    locs.push({ unit, grp: group(unit), state, entryKey, mtimeMs });
  } catch { keyErrors++; }
}
const phaseAMs = performance.now() - T0;

type Verdict = { holds: boolean; partial: boolean; rows: number; failing: Map<string, { kind: string }>; unlocalizable?: boolean; ms: number };
const rowMemo = new Map<string, boolean>(); let memoHits = 0, memoMiss = 0;
const rowFresh = (row: any): boolean => { let k: string; try { k = row.kind + "\0" + row.path + "\0" + JSON.stringify(row.fact); } catch { k = ""; } if (k !== "") { const m = rowMemo.get(k); if (m !== undefined) { memoHits++; return m; } } let ok = true; try { ok = OB.observationRowFreshV1(row); } catch { ok = false; } memoMiss++; if (k !== "") rowMemo.set(k, ok); return ok; };
const bundleVerdicts = new Map<string, Verdict>(), recipeMemo = new Map<string, any>();
const failFiles = new Map<string, { kinds: Set<string>; blobs: Set<string>; owners: number; mtimeMs?: number; missing: boolean; src: Set<string> }>();
const fileOf = (path: string): string | undefined => { if (path.startsWith("/")) return path; try { const j = JSON.parse(path); const f = Array.isArray(j) ? j.flat(3).find((x: any) => typeof x === "string" && x.startsWith("/")) : undefined; return f; } catch { return undefined; } };

async function judgeBundle(name: string, entry: any, all: Map<string, string>, context: any): Promise<Verdict> {
  const known = entry.hash === undefined ? undefined : bundleVerdicts.get(entry.hash); if (known) return known;
  const at = performance.now(); const doc = await entry.read({ keep: true });
  if (!doc) { const v = { holds: false, partial: false, rows: 0, failing: new Map(), unlocalizable: true, ms: 0 }; return v; }
  const packed = JSON.parse(Buffer.from(doc.bytes).toString("utf8")); const bundle = P.unpackBundleV1(packed);
  const moved = P.localBundleV1({ recipeKey: name.slice(8), bundle }, host, all, context);
  const v: Verdict = { holds: true, partial: false, rows: 0, failing: new Map(), ms: 0 };
  if (!moved) { v.holds = false; v.unlocalizable = true; } else {
    const seen = new Set<any>(), cap = at + 7000; let stop = false;
    const probe = (row: any) => { if (seen.has(row)) return; seen.add(row); v.rows++; const ok = rowFresh(row);
      if (!ok) { v.holds = false; const f = fileOf(String(row.path)) ?? String(row.path).slice(0, 160); if (!v.failing.has(f)) v.failing.set(f, { kind: row.kind }); if (v.failing.size >= 300) stop = true; }
      if ((v.rows & 1023) === 0 && performance.now() > cap) { v.partial = true; stop = true; } };
    for (const row of moved.bundle.rows) { probe(row); if (stop) break; }
    if (!stop) for (const layer of moved.bundle.layers) { for (const row of layer) { probe(row); if (stop) break; } if (stop) break; }
  }
  v.ms = performance.now() - at; if (entry.hash !== undefined) bundleVerdicts.set(entry.hash, v); return v;
}

type Judged = { grp: string; loc: Loc; status: "accepted" | "rejected" | "none" | "unreadable" | "unjudged"; reasons: string[]; createdMs?: number; origin?: string; blobs: string[]; staleBlobs: string[]; failFiles: string[]; partial: boolean };
const diag: string[] = []; const judged: Judged[] = []; const blobRefs = new Map<string, { name: string; owners: number; latestCreated: number; earliestCreated: number }>();
const withAc = locs.filter(l => l.mtimeMs !== undefined), noAc = locs.filter(l => l.mtimeMs === undefined);
const pending = withAc.slice(); let unjudged = 0;
for (const loc of pending) {
  if (judged.length >= MAXJ) break;
  if (left() < 4000) { unjudged = pending.length - judged.length; break; }
  const j: Judged = { grp: loc.grp, loc, status: "accepted", reasons: [], blobs: [], staleBlobs: [], failFiles: [], partial: false };
  try {
    const found = await X.fetchGraphEntryV1(client, loc.state, () => false, (n: string) => n.startsWith("closure-"));
    if (found === undefined) { j.status = "none"; judged.push(j); continue; }
    if (found === "fault") { j.status = "unreadable"; j.reasons.push("cache answered a fault"); judged.push(j); continue; }
    j.createdMs = found.note.createdMs; j.origin = String(found.note.producer).split(":")[0];
    const rec = found.documents.find((d: any) => d.name === "record"); if (!rec) { j.status = "rejected"; j.reasons.push("record document missing"); judged.push(j); continue; }
    const text = Buffer.from(rec.bytes).toString("utf8"), owned = text.includes(P.portableRowsPolicyV1.ownerToken);
    const taken = P.localRecordV1(P.unpackRecordV1(JSON.parse(text)), host, { providers: owned ? await providers() : [], generation });
    const bad = new Set<string>(); const badFiles: string[] = [];
    for (const row of taken.record.observations) { if (row.kind === "closure") continue; const ok = rowFresh(row);
      if (!ok) { bad.add("record row: " + row.kind); if (row.kind === "readdirNames" && diag.length < 5) { let now = ""; try { now = JSON.stringify(readdirSync(String(row.path)).sort()).slice(0, 260); } catch (e: any) { now = "unreadable " + e?.code; } diag.push(`${loc.unit.name} ${rel(String(row.path)).slice(-60)} recorded=${JSON.stringify(row.fact).slice(0, 260)} now=${now}`); } const f = fileOf(String(row.path)); if (f) { badFiles.push(f); const ff = failFiles.get(f) ?? { kinds: new Set(), blobs: new Set(), owners: 0, missing: false, src: new Set(["record"]) }; ff.kinds.add(row.kind); ff.owners++; ff.src.add("record"); failFiles.set(f, ff); } } }
    const lane = new Map<string, any>(found.deferred); const names = [...lane.keys()].sort();
    j.blobs = names.map(n => lane.get(n).hash ?? n);
    for (const n of names) { const e = lane.get(n); const h = e.hash ?? n; const b = blobRefs.get(h) ?? { name: n, owners: 0, latestCreated: 0, earliestCreated: Infinity }; b.owners++; b.latestCreated = Math.max(b.latestCreated, j.createdMs); b.earliestCreated = Math.min(b.earliestCreated, j.createdMs); blobRefs.set(h, b); }
    if (names.some(n => !bundleVerdicts.has(lane.get(n).hash))) {
      const recipes = new Map<string, unknown>();
      for (const n of names) { const e = lane.get(n); let r = e.hash !== undefined ? recipeMemo.get(e.hash) : undefined; if (r === undefined) { const d = await e.read({ keep: true }); if (d) { try { r = JSON.parse(Buffer.from(d.bytes).toString("utf8")).recipe; if (e.hash !== undefined) recipeMemo.set(e.hash, r); } catch {} } } if (r !== undefined) recipes.set(n.slice(8), r); }
      var all = new Map<string, string>([...taken.keys, ...P.bundleKeysV1([...recipes].map(([recipeKey, recipe]) => ({ recipeKey, bundle: { recipe } })), host, "local")]);
    } else var all = new Map();
    const context = { providers: owned ? await providers() : [], generation };
    for (const n of names) { if (left() < 1500) { j.partial = true; break; } const e = lane.get(n); const v = await judgeBundle(n, e, all, context);
      if (v.partial) j.partial = true;
      if (!v.holds) { j.staleBlobs.push(e.hash ?? n); bad.add(v.unlocalizable ? "closure entry unlocalizable" : "closure entry stale (" + [...new Set([...v.failing.values()].map(x => x.kind))].sort().join("+") + ")");
        for (const f of v.failing.keys()) { badFiles.push(f); const ff = failFiles.get(f) ?? { kinds: new Set(), blobs: new Set(), owners: 0, missing: false, src: new Set(["closure"]) }; ff.kinds.add(v.failing.get(f)!.kind); if (!ff.blobs.has(e.hash ?? n)) ff.blobs.add(e.hash ?? n); ff.owners++; failFiles.set(f, ff); } } }
        j.failFiles = [...new Set(badFiles)];
    if (bad.size > 0) { j.status = "rejected"; j.reasons = [...bad].sort(); }
  } catch (e: any) { j.status = "unreadable"; j.reasons.push("error: " + String(e?.message ?? e).slice(0, 80)); }
  judged.push(j);
}
for (const [f, ff] of failFiles) { try { ff.mtimeMs = statSync(f).mtimeMs; } catch { ff.missing = true; } }
const phaseBMs = performance.now() - T0 - phaseAMs;

// ---------- report ----------
const stamp = new Date(NOW).toISOString();
say(`R116 census ${stamp} UTC; host ${process.env.HOSTNAME ?? "ueli-mac"}; elapsed budget ${(BUDGET / 1000)} s`);
say(`wave ${wave.n} created ${iso(wave.head.createdMs)} UTC, ${units.length} owners (rust/cargo ${total["rust/cargo"]}, typescript ${total.typescript}, other ${total.other}); record revision ${OB.projectRecordKeyTextV1(null)}; globals ${JSON.stringify(Object.fromEntries(Object.entries(wave.head.globals).map(([k, v]) => [k, typeof v === "string" ? v.slice(0, 8) : v])))}`);
say(`phase A (key, state digest, store file time) covered ${locs.length} of ${units.length} owners in ${(phaseAMs / 1000).toFixed(1)} s (${keyErrors} key errors); phase B judged ${judged.length} owners in ${(phaseBMs / 1000).toFixed(1)} s; ${unjudged} stored owners left unjudged by the time budget`);
say(`waves on disk ${waves.length}; ${smallWaves} newer or older partial waves (under 80% of the largest units.json) skipped; the chosen wave is the newest full one`);
say(`landings used: ${last3.map(l => l.label).join("; ")}`);
say();
say("== 1. Owner records (verdict = rows of the record probed against the live tree + every closure entry it carries probed row by row, the verdict a taker gives) ==");
const cov: Record<string, { a: number; ac: number; j: number; acc: number; rej: number; none: number; unr: number }> = {};
for (const g of GROUPS) cov[g] = { a: 0, ac: 0, j: 0, acc: 0, rej: 0, none: 0, unr: 0 };
for (const l of locs) { const c = cov[l.grp]!; c.a++; if (l.mtimeMs !== undefined) c.ac++; }
for (const j of judged) { const c = cov[j.grp]!; c.j++; if (j.status === "accepted") c.acc++; else if (j.status === "rejected") c.rej++; else if (j.status === "none") c.none++; else c.unr++; }
say("kind        owners  keyed  stored(file)  none(file)  judged  accepted  rejected  unreadable");
let estAccepted = 0, estRejected = 0, estNone = 0, estUnr = 0, estStored = 0;
for (const g of GROUPS) { const c = cov[g]!, tot = total[g]!; const fracStored = c.a ? c.ac / c.a : 0, fa = c.j ? c.acc / c.j : 0, fr = c.j ? c.rej / c.j : 0, fu = c.j ? (c.unr + c.none) / c.j : 0;
  const stored = fracStored * tot; estStored += stored; estAccepted += stored * fa; estRejected += stored * fr; estUnr += stored * fu; estNone += tot - stored;
  say(`${g.padEnd(11)} ${String(tot).padStart(6)} ${String(c.a).padStart(6)} ${String(c.ac).padStart(12)} ${String(c.a - c.ac).padStart(11)} ${String(c.j).padStart(7)} ${String(c.acc).padStart(9)} ${String(c.rej).padStart(9)} ${String(c.unr + c.none).padStart(11)}`); }
say(`estimate for all ${units.length} owners (stratified by kind from the sample): accepted ${Math.round(estAccepted)} (${pct(estAccepted, units.length)}), rejected ${Math.round(estRejected)} (${pct(estRejected, units.length)}), stored but unreadable ${Math.round(estUnr)}, no store file ${Math.round(estNone)} (${pct(estNone, units.length)})`);
const rclass = (j: Judged) => { const rr = j.reasons.some(r => r.startsWith("record row")), cc = j.reasons.some(r => r.startsWith("closure entry")); return rr && cc ? "both" : rr ? "record-row only" : cc ? "closure-entry only" : "other"; };
const classEst: Record<string, number> = {}; for (const g of GROUPS) { const js = judged.filter(j => j.grp === g); if (!js.length) continue; for (const j of js.filter(j => j.status === "rejected")) classEst[rclass(j)] = (classEst[rclass(j)] ?? 0) + total[g]! / js.length; }
say("estimated rejected owners by reason class (stratified): " + Object.entries(classEst).map(([k, v]) => `${k} ${Math.round(v)} (${pct(v, units.length)})`).join("; ") + "  [record-row = the record's own rows, mostly readdirNames of a project directory that lists machine-local build output; closure-entry = a shared closure entry the record carries walked a file that changed]");
const reasonCount = new Map<string, number>(); for (const j of judged) if (j.status === "rejected" || j.status === "unreadable") { const k = j.reasons.join(" | "); reasonCount.set(k, (reasonCount.get(k) ?? 0) + 1); }
say("top rejection reasons (judged owners):"); for (const [k, v] of [...reasonCount].sort((a, b) => b[1] - a[1]).slice(0, 8)) say(`  ${String(v).padStart(4)}  ${k}`);
const bg: Record<string, Map<string, number>> = {}; for (const j of judged) if (j.status === "rejected") { const m = bg[j.grp] ??= new Map(); for (const r of j.reasons) m.set(r, (m.get(r) ?? 0) + 1); }
for (const g of GROUPS) if (bg[g]) say(`  ${g}: ` + [...bg[g]!].sort((a, b) => b[1] - a[1]).slice(0, 4).map(([k, v]) => `${v}x ${k}`).join("; "));
for (const d of diag) say("  readdirNames diagnosis: " + d);
say(`partial verdicts (a closure entry over 7 s or the budget ended): ${judged.filter(j => j.partial).length}`);

say(); say("== 2. Closure entries referenced by the judged owners ==");
const refd = [...blobRefs.keys()]; const staleH = refd.filter(h => bundleVerdicts.get(h) && !bundleVerdicts.get(h)!.holds), freshH = refd.filter(h => bundleVerdicts.get(h)?.holds), unj = refd.length - staleH.length - freshH.length;
say(`distinct shared closure entries referenced (by blob digest): ${refd.length}; distinct recipes ${new Set([...blobRefs.values()].map(b => b.name)).size}; fresh now ${freshH.length}; stale ${staleH.length}; not probed ${unj}; entry probe time ${[...bundleVerdicts.values()].reduce((a, v) => a + v.ms, 0) / 1000 | 0} s, rows probed ${[...bundleVerdicts.values()].reduce((a, v) => a + v.rows, 0)}`);
const toolsJson = JSON.parse(readFileSync(B + "/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🔣️.json", "utf8")); const toolDir = B + "/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures";
const toolEntries = new Map<string, string>(); for (const [n, t] of Object.entries<any>(toolsJson.tools.closures ?? {})) for (const e of t.entries ?? []) toolEntries.set(resolve(toolDir, e), n);
const cls = (f: string) => toolEntries.has(f) ? `declared-tool entry (${toolEntries.get(f)})` : f.includes(LIB + "🏗️build/") ? "build library (outside tool entries)" : f.includes("/🧰️framework/") ? "framework other" : f.includes("/node_modules/") || f.includes("/.cache/") ? "dependency/cache" : "project/other";
const top = [...failFiles].filter(([, ff]) => ff.blobs.size > 0).sort((a, b) => b[1].blobs.size - a[1].blobs.size || b[1].owners - a[1].owners).slice(0, 15);
say("top 15 files that staled closure entries (entries = distinct stale shared entries containing a failing row of the file; owners = judged records carrying them):");
say("entries owners born-stale  mtime(UTC)         age-min  kinds                   class                              file   [born-stale = judged records carrying the failing file that were created AFTER the file changed, i.e. the record shipped an entry that was already stale when it was written]");
const born = (f: string, ff: any) => { const rs = judged.filter(j => j.failFiles.includes(f)); return `${rs.filter(j => j.createdMs !== undefined && ff.mtimeMs !== undefined && j.createdMs > ff.mtimeMs).length}/${rs.length}`; };
for (const [f, ff] of top) say(`${String(ff.blobs.size).padStart(7)} ${String(ff.owners).padStart(6)} ${born(f, ff).padStart(10)}  ${(ff.mtimeMs ? iso(ff.mtimeMs) : "missing").padEnd(16)} ${(ff.mtimeMs ? String(Math.round((NOW - ff.mtimeMs) / 60000)) : "-").padStart(7)}  ${[...ff.kinds].join("+").padEnd(23)} ${cls(f).padEnd(34)} ${rel(f)}`);
const inLib = (f: string) => f.includes(LIB); const libFiles = [...failFiles].filter(([f, ff]) => ff.blobs.size > 0 && inLib(f)); const allStaleFiles = [...failFiles].filter(([, ff]) => ff.blobs.size > 0);
say(`failing files of closure entries: ${allStaleFiles.length}; in the build library ${libFiles.length}; the rest ${allStaleFiles.length - libFiles.length}; stale entries with at least one build-library failing file ${staleH.filter(h => [...failFiles].some(([f, ff]) => ff.blobs.has(h) && inLib(f))).length} of ${staleH.length}`);
const age = (ms: number) => (NOW - ms) / 60000; const bucket = new Map<number, number>();
for (const [, ff] of allStaleFiles) if (ff.mtimeMs) { const b = Math.floor(age(ff.mtimeMs) / 30); bucket.set(b, (bucket.get(b) ?? 0) + 1); }
say("mtime of failing closure-entry files, 30-minute buckets back from now (files): " + [...bucket].sort((a, b) => a[0] - b[0]).slice(0, 12).map(([b, n]) => `${b * 30}-${b * 30 + 30}min:${n}`).join("  "));
const inWin = (ms: number | undefined, l: { start: number; end: number }) => ms !== undefined && ms >= l.start - 60000 && ms <= l.end + 300000;
say("failing closure-entry files by landing window (start-1 min to end+5 min): " + last3.map(l => `${l.id}:${allStaleFiles.filter(([, ff]) => inWin(ff.mtimeMs, l)).length}`).join("  ") + `  none of the three:${allStaleFiles.filter(([, ff]) => !last3.some(l => inWin(ff.mtimeMs, l))).length}`);
say("failing library files with mtime inside a landing window: " + (libFiles.filter(([, ff]) => last3.some(l => inWin(ff.mtimeMs, l))).slice(0, 8).map(([f, ff]) => `${rel(f).replace(LIB + "🏗️build/", "")} ${ff.mtimeMs ? iso(ff.mtimeMs).slice(6) : ""}`).join("; ") || "none"));

const topRec = [...failFiles].filter(([, ff]) => ff.src.has("record") && ff.blobs.size === 0).sort((a, b) => b[1].owners - a[1].owners).slice(0, 8);
say("top files/directories failing rows of the RECORD itself (not of a closure entry): owners  mtime  kinds  file"); for (const [f, ff] of topRec) say(`  ${String(ff.owners).padStart(4)}  ${(ff.mtimeMs ? iso(ff.mtimeMs) : "missing").padEnd(14)} ${[...ff.kinds].join("+").padEnd(14)} ${rel(f)}`);
say(`row memo: ${memoHits} hits, ${memoMiss} probes`);
say(); say("== 3. Rate ==");
const pools = new Map<string, any>(); try { for (const o of readdirSync("/Users/Shared/semio-store/v1/origins")) { try { pools.set(o, JSON.parse(readFileSync(`/Users/Shared/semio-store/v1/origins/${o}/pool.json`, "utf8"))); } catch {} } } catch {}
const hostOrigin = new Map<string, string>(); for (const [o, p] of pools) if (p.host) hostOrigin.set(String(p.host), o);
const originOf = (producer?: string) => producer === undefined ? "?" : hostOrigin.get(producer) ?? (/^g1-/.test(producer) ? producer : producer.replace(/\.fritz\.box$/, "").slice(-22));
const windows = [10, 30, 60]; const frac = locs.length ? units.length / locs.length : 1; const scale = judged.length ? withAc.length / judged.length : 0;
const lag = judged.filter(j => j.createdMs !== undefined && j.loc.mtimeMs !== undefined).map(j => (j.loc.mtimeMs! - j.createdMs!) / 60000).sort((x, y) => x - y);
say(`sample: ${locs.length} keyed owners (scaled x${frac.toFixed(2)} to ${units.length}), ${judged.length} judged (accepted rates scaled x${(scale * frac).toFixed(2)}). Store file time vs the note's createdMs for ${lag.length} judged owners: median lag ${lag.length ? lag[lag.length >> 1]!.toFixed(1) : "-"} min, ${lag.filter(x => x > 2).length} of ${lag.length} files touched more than 2 min after the record was created (a store file time is NOT a creation time for those)`);
const stats: Record<number, { written: number; accepted: number; created: number }> = {};
for (const w of windows) { const lo = NOW - w * 60000; const wr = withAc.filter(l => l.mtimeMs! >= lo).length * frac; const created = judged.filter(j => (j.createdMs ?? 0) >= lo).length * scale * frac; const ac = judged.filter(j => j.status === "accepted" && (j.createdMs ?? 0) >= lo).length * scale * frac; stats[w] = { written: wr, accepted: ac, created };
  say(`last ${String(w).padStart(2)} min: owners with a store file touched ${wr.toFixed(0).padStart(4)}; records created (note createdMs) ${created.toFixed(0).padStart(4)} (${(created / w).toFixed(1)}/min); of those accepted now ${ac.toFixed(0).padStart(4)} (${(ac / w).toFixed(1)}/min)`); }
const origins = new Set<string>([...judged.map(j => originOf(j.origin)), ...pools.keys()]);
const grants = new Map<string, Set<string>[]>(); for (const [addr, cl] of claimsByAddress) for (const c of cl) { const a = grants.get(c.origin) ?? [new Set(), new Set(), new Set()]; windows.forEach((w, i) => { if (c.grantedMs >= NOW - w * 60000) a[i]!.add(addr + c.grantedMs); }); grants.set(c.origin, a); }
say("per origin: records created in the judged sample by note producer (scaled to all owners) w10/w30/w60 | accepted now w60 | claims granted in the claim log w10/w30/w60 | pool.json (ownersPerMinute, claimed, published, stored, children, wave)");
for (const o of [...origins].filter(o => o !== "?").sort()) { const c = (w: number) => judged.filter(j => originOf(j.origin) === o && (j.createdMs ?? 0) >= NOW - w * 60000).length * scale * frac; const a = judged.filter(j => originOf(j.origin) === o && j.status === "accepted" && (j.createdMs ?? 0) >= NOW - 3600000).length * scale * frac; const p = pools.get(o); const g = grants.get(o) ?? [new Set(), new Set(), new Set()];
  say(`  ${o.padEnd(11)} ${c(10).toFixed(0).padStart(4)} ${c(30).toFixed(0).padStart(4)} ${c(60).toFixed(0).padStart(4)} | ${a.toFixed(0).padStart(4)} | ${g.map(x => String(x.size).padStart(4)).join(" ")} | ${p ? `${p.ownersPerMinute}/min claimed ${p.claimed} published ${p.published} stored ${p.stored} children ${p.children} reason ${p.reason} wave ${String(p.wave).slice(0, 8)} updated ${iso(p.updatedMs).slice(6)}` : "no pool.json"}`); }
{ const m = (xs: number[]) => xs.length ? Math.round(xs.reduce((a, b) => a + b, 0) / xs.length) : 0; const by = (f: (j: Judged) => boolean) => judged.filter(f);
  const line = (n: string, js: Judged[]) => `${n}: ${js.length} owners, mean ${m(js.map(j => Number(j.loc.unit.megabytes) || 0))} MB expected memory, mean ${m(js.map(j => j.loc.unit.expectedMilliseconds / 1000))} s expected derivation`;
  say("size of the owners the pools must refresh (wave units): " + [line("accepted", by(j => j.status === "accepted")), line("rejected, closure-entry class", by(j => j.status === "rejected" && rclass(j) !== "record-row only")), line("rejected, record-row only", by(j => j.status === "rejected" && rclass(j) === "record-row only"))].join(" | ")); }
const pw = [...pools.values()].map(p => p.wave); say(`waves the pools follow right now: ${[...new Set(pw)].map(w => String(w).slice(0, 8)).join(", ")}; pool reasons: ${[...pools.entries()].map(([o, p]) => o.replace("g1-", "") + "=" + p.reason).join(" ")}; waves on disk created in the last 30 min: ${waves.filter(w => NOW - w.head.createdMs < 1800e3).length}`);
const remaining = Math.max(0, units.length - estAccepted); const r30 = stats[30]!.accepted / 30, r60 = stats[60]!.accepted / 60;
const hrs = (r: number) => r > 0 ? (remaining / r / 60).toFixed(1) + " h" : "never (rate 0)";
say(`accepted now (estimate) ${Math.round(estAccepted)} of ${units.length} = ${pct(estAccepted, units.length)}; remaining ${Math.round(remaining)}`);
const closureLeft = (classEst["closure-entry only"] ?? 0) + (classEst["both"] ?? 0); say(`projection under a landing freeze: closure-entry class ${Math.round(closureLeft)} owners / ${r60.toFixed(1)}-${r30.toFixed(1)} per min = ${r60 > 0 ? (closureLeft / Math.max(r30, r60) / 60).toFixed(1) : "?"} to ${r30 > 0 ? (closureLeft / Math.min(r30, r60 || r30) / 60).toFixed(1) : "?"} h; record-row-only class ${Math.round(classEst["record-row only"] ?? 0)} owners does not converge by refreshing (a refresh on a machine without the build output is rejected on a machine with it)`);
say(`projected time to all acceptable, nothing re-staled: at the 30-min rate (${r30.toFixed(2)}/min) ${hrs(r30)}; at the 60-min rate (${r60.toFixed(2)}/min) ${hrs(r60)}`);

say(); say("== 4. Re-staling by landings ==");
say("per landing: judged records written before the landing start; of those rejected now; rejected now with a failing file (record row or closure entry) whose mtime is within [start-1 min, end+5 min] (= staled by that landing, they were acceptable before it if their rows held then); scaled to all owners");
for (const l of last3) { const before = judged.filter(j => j.createdMs !== undefined && j.createdMs < l.start); const rej = before.filter(j => j.status === "rejected"); const byL = rej.filter(j => j.failFiles.some(f => inWin(failFiles.get(f)?.mtimeMs, l)));
  const onlyL = byL.filter(j => !j.failFiles.some(f => !last3.some(x => inWin(failFiles.get(f)?.mtimeMs, x)) && failFiles.get(f)?.mtimeMs));
  say(`  ${l.label}: written before ${before.length}; rejected now ${rej.length}; staled by files changed in this window ${byL.length} (${pct(byL.length, before.length)} of those before; ~${Math.round(byL.length * scale * frac)} of all owners); staled by this window alone ${onlyL.length}`); }
const stampsBefore = (l: { start: number }) => judged.filter(j => j.createdMs !== undefined && j.createdMs < l.start).length; void stampsBefore;
const writtenAfter = last3.length ? judged.filter(j => (j.createdMs ?? 0) >= last3[0]!.start) : []; say(`records written since ${last3[0]?.id ?? "-"} start: ${writtenAfter.length} judged, ${writtenAfter.filter(j => j.status === "accepted").length} still accepted, ${writtenAfter.filter(j => j.status === "rejected").length} already rejected`);
const rejectedAny = judged.filter(j => j.status === "rejected"); const anyLanding = rejectedAny.filter(j => j.failFiles.some(f => last3.some(l => inWin(failFiles.get(f)?.mtimeMs, l)))).length;
say(`of ${rejectedAny.length} rejected judged records: ${anyLanding} fail on a file changed in one of the three landing windows, ${rejectedAny.filter(j => j.failFiles.length > 0).length - anyLanding} only on files changed outside them (peer edits), ${rejectedAny.filter(j => j.failFiles.length === 0).length} without a located file`);
const allFailRecord = [...failFiles].filter(([, ff]) => ff.src.has("record") && ff.blobs.size === 0).sort((a, b) => 0).slice(0, 0); void allFailRecord;
{ let born = 0, went = 0, mixed = 0; for (const j of rejectedAny) { const ms = j.failFiles.map(f => failFiles.get(f)?.mtimeMs).filter((x): x is number => x !== undefined); if (ms.length === 0 || j.createdMs === undefined) continue; const after = ms.filter(m => m > j.createdMs!).length; if (after === 0) born++; else if (after === ms.length) went++; else mixed++; }
  say(`rejected judged records by when their failing files changed relative to the record's createdMs: born stale (every failing file changed BEFORE the record was written) ${born}; went stale (every failing file changed AFTER) ${went}; mixed ${mixed}`);
  const young = judged.filter(j => j.createdMs !== undefined && NOW - j.createdMs < 3600000); say(`records created in the last 60 min: ${young.length} judged, ${young.filter(j => j.status === "accepted").length} accepted, ${young.filter(j => j.status === "rejected").length} rejected`); }
{ const bk: [string, number, number][] = [["<1 h", 0, 3600e3], ["1-3 h", 3600e3, 3 * 3600e3], ["3-6 h", 3 * 3600e3, 6 * 3600e3], ["6-12 h", 6 * 3600e3, 12 * 3600e3], [">12 h", 12 * 3600e3, 1e15]];
  say("age of the judged records (note createdMs): " + bk.map(([n, a, b]) => { const r = judged.filter(j => j.createdMs !== undefined && NOW - j.createdMs >= a && NOW - j.createdMs < b); return `${n}: ${r.filter(j => j.status === "accepted").length} accepted / ${r.filter(j => j.status === "rejected").length} rejected`; }).join("; "));
  const accN = judged.filter(j => j.status === "accepted").length, p = judged.length ? accN / judged.length : 0, ci = judged.length ? 1.96 * Math.sqrt(p * (1 - p) / judged.length) : 0; say(`judged ${judged.length}: accepted ${accN} = ${(100 * p).toFixed(1)}% +/- ${(100 * ci).toFixed(1)} points (95%, unstratified)`);
  say("records created in the last 90 min and rejected now (name, kind, created UTC, origin, up to 3 failing files with mtime UTC):");
  for (const j of judged.filter(j => j.status === "rejected" && j.createdMs !== undefined && NOW - j.createdMs < 5400e3).slice(0, 14)) say(`  ${j.loc.unit.name.slice(0, 44).padEnd(44)} ${j.grp.padEnd(10)} ${iso(j.createdMs!).slice(6)} ${originOf(j.origin).padEnd(8)} ${j.failFiles.slice(0, 3).map(f => rel(f).replace(LIB + "🏗️build/", "L:").slice(-70) + " " + (failFiles.get(f)?.mtimeMs ? iso(failFiles.get(f)!.mtimeMs!).slice(6) : "?")).join(" | ")}`); }
{ const cl = (j: Judged) => { const rr = j.reasons.some(r => r.startsWith("record row")), cc = j.reasons.some(r => r.startsWith("closure entry")); return rr && cc ? "both" : rr ? "record-row" : "closure-entry"; };
  const bk: [string, number, number][] = [["<1 h", 0, 3600e3], ["1-3 h", 3600e3, 3 * 3600e3], ["3-6 h", 3 * 3600e3, 6 * 3600e3], ["6-12 h", 6 * 3600e3, 12 * 3600e3], [">12 h", 12 * 3600e3, 1e15]];
  say("rejected judged records by age and reason class (record-row only / closure-entry only / both): " + bk.map(([n, a, b]) => { const r = judged.filter(j => j.status === "rejected" && j.createdMs !== undefined && NOW - j.createdMs >= a && NOW - j.createdMs < b); return `${n}: ${r.filter(j => cl(j) === "record-row").length}/${r.filter(j => cl(j) === "closure-entry").length}/${r.filter(j => cl(j) === "both").length}`; }).join("; "));
  const recs = judged.filter(j => j.status === "rejected" && j.createdMs !== undefined && NOW - j.createdMs < 6 * 3600e3 && cl(j) !== "record-row"); say(`rejected records created in the last 6 h with a stale closure entry: ${recs.length}; failing build-library files among them: ${new Set(recs.flatMap(j => j.failFiles).filter(f => f.includes(LIB))).size}`); }
const libTable = [...failFiles].filter(([f, ff]) => ff.blobs.size > 0 && f.includes(LIB)).sort((a, b) => b[1].blobs.size - a[1].blobs.size);
say("all failing build-library files of closure entries (entries owners mtime): " + libTable.slice(0, 40).map(([f, ff]) => `${f.replace(root + "/" + LIB + "🏗️build/", "")} ${ff.blobs.size} ${ff.owners} ${ff.mtimeMs ? iso(ff.mtimeMs).slice(6) : "?"}`).join(" ;; "));
say("recipes of the 8 stale shared entries carried by most judged records (entry name, owners, recipe start):"); for (const h of staleH.sort((a, b) => blobRefs.get(b)!.owners - blobRefs.get(a)!.owners).slice(0, 8)) { const b = blobRefs.get(h)!; say(`  ${b.name.slice(0, 20)} ${b.owners} ${JSON.stringify(recipeMemo.get(h) ?? null).slice(0, 220)}`); }
say(`total seconds ${((performance.now() - T0) / 1000).toFixed(1)}`);
rmSync(base, { recursive: true, force: true }); process.exit(0);
