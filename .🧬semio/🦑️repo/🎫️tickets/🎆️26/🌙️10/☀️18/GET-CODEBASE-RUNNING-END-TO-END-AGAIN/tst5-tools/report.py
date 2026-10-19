import json,re
C=open('/private/tmp/claude-501/c.txt').read().strip()
def load(p): return {json.loads(l)['file']:json.loads(l) for l in open(p)}
r1=load(C+'/🗑️generated/tst5-out/r1.jsonl'); r2=load(C+'/🗑️generated/tst5-out/r2.jsonl'); r3=load(C+'/🗑️generated/tst5-out/r3.jsonl')
fin={f:(r3[f] if f in r3 else r2[f]) for f in r2}
ovr={'vite/📥️artifacts':(39,4)}
red1=json.load(open('/private/tmp/claude-501/red1.json'))
N={
0:("red","NO-DIST: package exports ./dist, not built (needs <pkg>:build)"),
1:("red","NO-DIST self-reference of wfc bitmap"),
2:("red","pinned meshes absent (cargo-produced fixtures)"),3:("red","pinned meshes absent (cargo-produced fixtures)"),4:("red","pinned meshes absent (cargo-produced fixtures)"),
5:("red","geometry-service fixture is {} since HEAD 675; needs cargo service run"),
6:("red","Flow retirement law: `retirement::shared` -> `arc_custody`, ToolJobFactory `create_job_from_wire_pages*` -> `admission_demands`/`admit_job`"),
7:("red","NO-DIST"),8:("red","NO-DIST"),9:("red","NO-DIST"),10:("red","NO-DIST"),
11:("red","derived child handles are now rebuilt by parseFormsChild: `assert.equal` -> `deepEqual`; remaining failure is NO-DIST forms-js"),
12:("red","map delta wire is a keyed object (framework map schema): test input, 6 committed `🔺️diff` fixtures and the rewriting diff schema `entries` array -> object"),
13:("red","`advance(&mut self, body:usize, grant:` signature of fem 2d/3d editors accepted by the law regex"),
14:("red","tree-sitter-rust grammar 0.1.13 cannot parse async closures `async |x|` (toolchain)"),
15:("red","tests dir moved one level down (`io/sqlite/snapshot/tests`): 5 -> 4 `../`"),
16:("red","unit root `🧪️tests/🔬️unit/🦀️.rs` is a mount root: tree reader"),
17:("red","law forbids `ArtifactVcs` in puzzle-3d wasm bridge; code still declares `Puzzle3dArtifactVcs` (VCS removal pending)"),
18:("red","Rust readers -> tree readers; `OriginalFixtureScope` / `close_registered_fixture_app` needles"),
19:("red","545 fixtures outside `🧪️tests` (533 FEM): 13 energy + 2 puzzle + 2 schema-ancestor moved, pv-plane TS import via test module; FEM left (generator path tables); BIM glb/ifc blessed output needs cargo"),
20:("red","NO-DIST"),21:("red","NO-DIST"),
22:("red","needs SEMIO_FEM_NEUTRAL_RESULTS (cargo native output)"),23:("red","needs SEMIO_FEM_NEUTRAL_RESULTS (cargo native output)"),
24:("red","`pub fn pcg(` lives in a split module: tree reader"),
25:("red","live launcher (playground preparation invocation)"),26:("red","live launcher (bounded Nx namespace)"),27:("red","owned capture limit (live launcher)"),
28:("(not listed) regression","`🪣️fill-job` split into `➕️normal/🪣️fill/**`: tree reader; `step(scene,style,brush,cx)` signature"),
29:("(not listed) regression","`validateBrowserActorUiValues` moved to `🧩️actor-wire-helpers`"),
30:("red","store worker import cycle fixed by peers"),
31:("(not listed) regression","config-diff-twin fixtures moved to `🎚️config/🧪️tests/🧫️fixtures/🧬️mutations`; remaining: `retry` not installed, `cockatiel` not hoisted"),
32:("red","tree-sitter refuses Rust source + source-law drift"),
33:("red","completed-preview needle re-derived from `step_preview`/`settle`"),
34:("red","standalone rustc isolation no longer links (module imports carved crates)"),
35:("red","slice bounded to the `artifact_retirement_box_*` fns (tree order pulled a later trait)"),
36:("red","`.max(grant.maximum_capacity_bytes)` form accepted; bare `maximum_capacity_bytes: demand.capacity_bytes` still refused"),
37:("red","`advance_demands`/`next_demand` replace four per-lane `next_advance_*_demand`; `semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES`"),
38:("red","`pub fn seal_prepared_owned` (cross-crate callers) vs law `pub(crate)`: law/crate-layout conflict, owner decision"),
39:("red","`advance(grant, receiving)` signature"),
40:("red","fixture `requiredReceivingPorts` advance signature"),
41:("red","`header.advance(original,control,body,grant)`"),
42:("red","History driver test is a mount root: tree reader"),
43:("red","`pub struct RendererAssetDecodeJob` (was pub(crate)); slice to EOF"),
44:("red","live launcher (Original process issuance)"),
45:("red","tree readers, `map_err(ProgramFault::from)`, Drop-based `begin_close`, ProgramBridge lifted to contracts crate; remaining: `ProgramFault` derives Clone/PartialEq with a non-Clone `create_app_source` field (law wants Debug only)"),
46:("red","`pub fn poll_compiler`/`dispatch`; tree readers"),
47:("red","kernel runtime root lifted to kernel-protocol, asset stream moved to presentation-raster, `pub fn pump_renderer_source_io_sessions`"),
48:("red","`AppFramePreparation` moved to frame-build; `retire_for_component_surface_close_step` -> `retire_original_frame_step`; mailbox `pub fn close_engine_surface_step` + registry path"),
49:("red","Playwright chromium 1248 not installed"),
50:("red","child script invocation missing (live launcher)"),
51:("red","Bun 1.3.14: byte accounting of NUL-containing TEXT differs by 1 per NUL string from the committed corpus (owner decision)"),52:("red","same as above"),
53:("red","`try_serialize_cold`"),
54:("(not listed) regression","dashboard `run-script` command gained `--dry-run={dry-run}`"),
55:("red","receipts routed before `for frame_bytes in output.frames` (typed_operation_results loop gone)"),
56:("red","authority text scoped to `🤝️authority`; whitespace-insensitive close-step needle"),
57:("red","needs native result artifact (cargo)"),
58:("red","cargo-test output name `native_catalog_empty_alternate_and_refused_owners_match_portable_corpus ... ok`"),
59:("(not listed) regression","240 s timeout under 2-worker load (green in the later runs)"),
60:("(not listed) red","committed JSON/pack descriptor pairs disagree for 14 hub components (JSON regenerated 10-15, pack 10-13): needs cargo/wasm regeneration"),
61:("red","bootstrap uses `readScriptProcessEnvelope(JSON.parse(env[SCRIPT_PROCESS_INVOCATION_ENV]!),Date.now())`"),
62:("red","body slice bounded to its fn; fixture dispatch checked in its own module; tree readers"),
63:("red","hash pins of Rust bodies / mount lists (needs re-bless by Rust owners)"),
64:("red","aggregate `includes` laws on unrelated Rust (SOURCE-LAW-DRIFT)"),
65:("red","`source.matches(cx)` replaces `cx.operation().0`; tree reader"),
66:("red","cargo-running unit (native caller capabilities)"),
67:("red","removed dead `#[doc(hidden)] pub fn native_io_close_grant`"),
68:("red","`buildAdmittedWasmWebV1` accepted besides `buildRepositoryWasmWebV1`; remaining: peer taxonomy edit (`pathBudgetReserve`), play site script, bounded browser inventory"),
69:("red","peers fixed to 127/0; a later peer Cargo edit (mutation-diff -> semio-framework-value) fails 1"),
}
rows=[]
def pf(r): return f"{r['pass_']}/{r['fail']}"
red1set=set(red1)
for i,f in enumerate(red1):
    a=r1[f]; b=fin[f]; (p,fl)=ovr.get(next((k for k in ovr if k in f),None),(b['pass_'],b['fail']))
    after="green "+f"{p}/{fl}" if b['rc']==0 else "red "+f"{p}/{fl}"
    rows.append((f,N[i][0],"red "+pf(a),after,N[i][1]))
extra=[f for f in fin if f not in red1set and (r2[f]['rc']!=0 or fin[f]['rc']!=0)]
EX={"bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite":"peer in flight: `runArtifactRustPackageMain` now `adopt()`s a ScriptRouter, the bim package script still passes a record",
"service-composition/🧪️tests/📥️checked":"peer split of gis `💡️inference-port` test (fixture-count law)","flow/🧩️extensions/🕸️wasm":"`require(\"@semio-tech/framework-plugin/**/schema.json\")` now resolves to a json facade `.ts` (namespace, not schema): `schemaOf` unwraps `default`","local-hub/🎫️session":"2 s resource-loan deadline under 2-worker load (flaky)"}
for f in extra:
    note=next((v for k,v in EX.items() if k in f),"")
    b=fin[f]
    rows.append((f,"green (not red in tst3b)","green "+pf(r1[f]),("green " if b['rc']==0 else "red ")+pf(b),note))
def short(f): return f.replace("🧰️framework/🛍️products/💻️os/","OS/").replace("✏️s/🔌️plugins/","P/").replace("🏅️standards/🔖️1/🪆️subsets/✳️any/","S1/")
out=["| suite | tst3b | now (start of tst5) | after fix | cause / fix |","|---|---|---|---|---|"]
for r in rows: out.append("| `"+short(r[0])+"` | "+r[1]+" | "+r[2]+" | "+r[3]+" | "+r[4]+" |")
open(C+'/🗑️generated/tst5-out/table.md','w',encoding='utf-8').write("\n".join(out)+"\n")
g1=sum(1 for r in r1.values() if r['rc']==0); gf=sum(1 for f,r in fin.items() if r['rc']==0)
print(len(r1),g1,len(fin),gf,len(rows))
