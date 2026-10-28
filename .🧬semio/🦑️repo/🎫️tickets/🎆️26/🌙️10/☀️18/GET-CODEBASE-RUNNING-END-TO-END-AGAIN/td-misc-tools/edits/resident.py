DOC = """🔭 Staged batch `resident` (lane td-misc): (1) REAL BUG: the resident service answered `dirty` from its event journal as complete although its watcher was not exact (backend none/poll, portable fallback, failed fence), so a generation host could skip its stat pass on an empty journal; the journal now proves nothing (`complete:false, reason:"trust"`) unless the events are exact. (2) The resident service and watch laws get their native watcher the way `repo service` does (configured binary, else the repository's provisioned `repo-watch`, built once from the pinned source) instead of silently degrading to backend `none` in a bare run; the process crash-safety laws that need the fixture tools directory stay gated on SEMIO_REPO_NATIVE_DIR."""
R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/"
EDITS = [
 (R + "🗃️state/🟨️.mjs", [
  ('/** 📖 Net membership changes after a cursor, folded in order; incomplete when the cursor is another instance, predates the journal, or a loss happened since. */\n const since=from=>{\n  if(from.instance!==instance||from.seq<journalFloor)return{complete:false,reason:"cursor"};\n',
   '/** 📖 Net membership changes after a cursor, folded in order; incomplete when the cursor is another instance, predates the journal, a loss happened since, or the events are not exact (no watcher, a polling or degraded one): only an exact journal can prove that nothing changed. */\n const since=from=>{\n  if(from.instance!==instance||from.seq<journalFloor)return{complete:false,reason:"cursor"};\n  if(trust!=="exact")return{complete:false,reason:"trust"};\n'),
 ]),
 (R + "🗃️state/🟨️.d.mts", [
  ('{complete:false;reason:"cursor"|"lost";scope?:string}', '{complete:false;reason:"cursor"|"lost"|"trust";scope?:string}'),
 ]),
 (R + "🧪️tests/🗃️state/🟦️.ts", [
  ('  state.setTrust("verify");\n  const key=await state.taskKey("o1:build"),hashed=state.stats.hashCalls;\n',
   '  state.setTrust("verify");\n  expect(state.since(state.cursor())).toMatchObject({complete:false,reason:"trust"});\n  const key=await state.taskKey("o1:build"),hashed=state.stats.hashCalls;\n'),
 ]),
 (R + "🧪️tests/🟦️.ts", [
  ('export const WATCHER=process.env.SEMIO_REPO_WATCH_BINARY??(NATIVE?NATIVE+"/semio-repo-watch":"");\n',
   'export const WATCHER=process.env.SEMIO_REPO_WATCH_BINARY??(NATIVE?NATIVE+"/semio-repo-watch":"");\n/** 🔭 The native watcher of these laws: the configured binary, else the one the repository provisions for itself (built once from the pinned source), exactly as `repo service` resolves it. */\nexport async function residentWatcherV1():Promise<string>{if(WATCHER)return WATCHER;const{ensureResidentWatcherV1}=await import("../🔭️watch/🏗️provision/🔌️tools/🟨️.mjs");return ensureResidentWatcherV1(join(HERE,"..","..","..","..","..","..","..",".."));}\n'),
 ]),
 (R + "🧪️tests/🔭️watch/🟦️.ts", [
  ('import {WATCHER,cleanup,haveBinary,sleep,snapshot,tempRoot,waitFor,writeTree} from "../🟦️.ts";', 'import {cleanup,haveBinary,residentWatcherV1,sleep,snapshot,tempRoot,waitFor,writeTree} from "../🟦️.ts";'),
  ('const native=haveBinary(WATCHER);', 'const WATCHER=await residentWatcherV1(),native=haveBinary(WATCHER);'),
 ]),
 (R + "🧪️tests/🏛️service/🟦️.ts", [
  ('import {WATCHER,cleanup,haveBinary,pick,rng,shortTempDirectory,sleep,snapshot,tempRoot,waitFor,writeTree} from "../🟦️.ts";', 'import {NATIVE,cleanup,haveBinary,pick,residentWatcherV1,rng,shortTempDirectory,sleep,snapshot,tempRoot,waitFor,writeTree} from "../🟦️.ts";'),
  ('const native=haveBinary(WATCHER);', 'const WATCHER=await residentWatcherV1(),native=haveBinary(WATCHER);'),
  ('  const s=await startService({backend,binary:undefined});\n  await s.run("run",["c:build"]);expect((await s.run("run",["c:build"])).served).toBe("memo");\n  await sleep(30);writeFileSync(s.root+"/pkgs/b/src/x.ts","changed silently and longer");\n',
   '  const s=await startService({backend,binary:undefined}),cursor=JSON.parse((await s.run("cursor",[])).out);\n  await s.run("run",["c:build"]);expect((await s.run("run",["c:build"])).served).toBe("memo");\n  await sleep(30);writeFileSync(s.root+"/pkgs/b/src/x.ts","changed silently and longer");\n  expect(JSON.parse((await s.run("dirty",["--since",cursor.instance+":"+cursor.seq])).out)).toMatchObject({complete:false,reason:"trust"});\n'),
  ('describe.skipIf(!native)("process level crash safety",()=>{', 'describe.skipIf(!native||!NATIVE)("process level crash safety",()=>{'),
 ]),
]
