import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { resolve, join } from "node:path";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";
import { Database } from "bun:sqlite";

test("execution budgets follow portable environment laws and independent native oracles", async () => {
  const owner = resolve(import.meta.dir, ".."), source = join(owner, "🟦️.ts");
  const corpus = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
  const require = createRequire(import.meta.url), ajv = new (require("ajv").default)();
  ajv.addSchema(schema);
  const api = await import(source);
  const read = (api: any, env: Readonly<Record<string, string>>): unknown => {
    try { return { build: api.buildBudgetMs(env), command: api.cmdBudgetMs(env), orchestrator: api.orchestratorBudgetMs(env), daemon: api.daemonBudgetMs(env) }; }
    catch { return null; }
  };
  const values = corpus.cases.map((row: any) => read(api, row.environment));
  expect(values).toEqual(corpus.cases.map((row: any) => row.expected));
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/Budgets" });
  for (let index = 0; index < values.length; index++) if (values[index] !== null) expect(validate(values[index])).toBe(true);
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR!; expect(output).toBeTruthy(); mkdirSync(output, { recursive: true });
  const temporary = mkdtempSync(join(output, "execution-budget-")), module = join(temporary, "budget.mjs");
  try {
    const bundled = await require("esbuild").build({ entryPoints: [source], outfile: module, bundle: true, platform: "node", format: "esm", metafile: true });
    expect(Object.keys(bundled.metafile.inputs).sort()).toEqual([source.substring(process.cwd().length + 1), join(owner, "🧬️schema/🔣️.json").substring(process.cwd().length + 1)].sort());
    const program = `const rows=JSON.parse(require('node:fs').readFileSync(0,'utf8')).cases;const read=${read.toString()};import(require('node:url').pathToFileURL(process.argv[1]).href).then(api=>console.log(JSON.stringify(rows.map(row=>read(api,row.environment)))));`;
    const child = spawnSync("node", ["-e", program, module], { input: JSON.stringify(corpus), encoding: "utf8", timeout: 15000 });
    expect(child.status, child.stderr).toBe(0); expect(JSON.parse(child.stdout)).toEqual(values);
    const oracle = spawnSync(process.platform === "win32" ? "python" : "python3", ["-c", `import sys,json,decimal\ndata=json.load(sys.stdin); result=[]\nkeys={'build':'SEMIO_BUILD_BUDGET_MS','command':'SEMIO_CMD_BUDGET_MS','orchestrator':'SEMIO_ORCHESTRATOR_BUDGET_MS','daemon':'SEMIO_DAEMON_BUDGET_MS'}\nfor row in data['cases']:\n try:\n  values={kind:decimal.Decimal(row['environment'].get(key,'0').strip() or '0') for kind,key in keys.items()}\n  if any(not value.is_finite() or value!=value.to_integral_value() or value<0 or value>9007199254740991 for value in values.values()): raise ValueError('invalid')\n  result.append({kind:int(value) for kind,value in values.items()})\n except (ValueError,decimal.InvalidOperation): result.append(None)\nprint(json.dumps(result))`], { input: JSON.stringify(corpus), encoding: "utf8", timeout: 15000 });
    expect(oracle.status, oracle.stderr).toBe(0); expect(JSON.parse(oracle.stdout)).toEqual(values);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
});


/** 🧭️ Keeps finite compiler preparation distinct from a long-lived caller session. */
test("build preparation has an authored finite fallback and honors explicit finite caller limits", async () => {
  const owner = resolve(import.meta.dir, ".."), corpus = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
  const api = await import(join(owner, "🟦️.ts")), require = createRequire(import.meta.url), ajv = new (require("ajv").default)(), validate = ajv.compile({ ...schema, $ref: "#/$defs/BuildPreparationMilliseconds" }), database = new Database(":memory:");
  try { for (const row of corpus.preparation) {
    if (row.expected === null) { expect(() => api.buildPreparationBudgetMs(row.environment)).toThrow(/Invalid process budget/u); continue; }
    const value = api.buildPreparationBudgetMs(row.environment), independent = database.query("select coalesce(nullif(cast(?1 as integer),0),?2) value").get(row.environment.SEMIO_BUILD_BUDGET_MS ?? "0", schema.$defs.BuildPreparationDefault.const) as { value: number };
    expect(value).toBe(row.expected); expect(value).toBe(independent.value); expect(validate(value)).toBe(true);
    expect(api.buildBudgetMs({})).toBe(0);
  } } finally { database.close(); }
});

test("owned process timers preserve a single deadline beyond the native timer range", async () => {
  const owner=resolve(import.meta.dir,".."),corpus=JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")),require=createRequire(import.meta.url),ajv=new(require("ajv").default)(),validate=ajv.compile({...schema,$ref:"#/$defs/TimerCase"}),ts=require("typescript"),source=resolve(owner,"../🎛️owned-execution/🟦️.ts"),file=ts.createSourceFile(source,readFileSync(source,"utf8"),ts.ScriptTarget.Latest,true),declaration=file.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="startOwnedCommandTimeoutV1");
  expect(declaration).toBeDefined();
  const implementation=require("esbuild").transformSync(declaration.getText(file).replace(/^export /u,""),{loader:"ts",format:"cjs"}).code,database=new Database(":memory:");
  try { for(const row of corpus.timers){
    expect(validate(row),JSON.stringify(validate.errors)).toBe(true);
    let elapsed=0,expirations=0,next=0;const delays:number[]=[],pending=new Map<number,()=>void>();
    const create=new Function("processBudgetSchema","performance","setTimeout","clearTimeout",implementation+";return startOwnedCommandTimeoutV1;")(schema,{now:()=>elapsed},(callback:()=>void,delay:number)=>{delays.push(delay);pending.set(++next,callback);return next;},(handle:number)=>pending.delete(handle)),stop=create(row.budget,()=>expirations++);
    for(const[index,check]of row.checks.entries()){const entry=pending.entries().next().value as[number,()=>void]|undefined;if(entry)pending.delete(entry[0]);if(row.cancelBeforeCheck===index)stop();elapsed=check;entry?.[1]();}
    stop();stop();expect(pending.size).toBe(0);expect(delays).toEqual(row.expectedDelays);expect(expirations).toBe(row.expectedExpirations);
    const oracle=database.query("WITH observations AS (SELECT -1 ordinal,0 elapsed UNION ALL SELECT CAST(key AS INTEGER),CAST(value AS INTEGER) FROM json_each(?1)) SELECT json_group_array(delay) delays FROM (SELECT min(?4,?2-o.elapsed) delay FROM observations o WHERE ?2>0 AND o.elapsed<?2 AND o.ordinal<coalesce(?3,2147483647) AND NOT EXISTS (SELECT 1 FROM observations e WHERE e.ordinal<o.ordinal AND e.elapsed>=?2) ORDER BY o.ordinal)").get(JSON.stringify(row.checks),row.budget,row.cancelBeforeCheck??null,schema.$defs.TimerMaximumMilliseconds.const)as{delays:string},expiry=database.query("SELECT CASE WHEN ?2>0 AND EXISTS (SELECT 1 FROM json_each(?1) WHERE CAST(value AS INTEGER)>=?2 AND CAST(key AS INTEGER)<coalesce(?3,2147483647)) THEN 1 ELSE 0 END value").get(JSON.stringify(row.checks),row.budget,row.cancelBeforeCheck??null)as{value:number};
    expect(delays).toEqual(JSON.parse(oracle.delays));expect(expirations).toBe(expiry.value);
  } } finally { database.close(); }
  const{runOwnedCommand}=await import(source);
  for(const budget of[2147483647,2147483648,9007199254740991])await runOwnedCommand(process.execPath,["-e","setTimeout(()=>process.exit(0),60)"],process.cwd(),"process:large-timer",budget,{stdout:"ignore",input:""});
  await expect(runOwnedCommand(process.execPath,["-e","setTimeout(()=>process.exit(0),1000)"],process.cwd(),"process:short-timer",30,{stdout:"ignore",input:""})).rejects.toThrow(/timeout 30ms/u);
},15000);


test("public bounded timeout validates before scheduling and retires cancellation",async()=>{
 const owner=resolve(import.meta.dir,".."),corpus=JSON.parse(readFileSync(join(owner,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")),api=await import(resolve(owner,"../🎛️owned-execution/🟦️.ts")),require=createRequire(import.meta.url),valid=new(require("ajv").default)().compile({...schema,$ref:"#/$defs/TimerInputCase"}),db=new Database(":memory:");try{for(const row of corpus.timerInputs){expect(valid(row)).toBe(true);const oracle=db.query("SELECT json_type(?1,'$.budget') IN ('integer','real') AND json_extract(?1,'$.budget')>=0 AND json_extract(?1,'$.budget')<=9007199254740991 AND json_extract(?1,'$.budget')=cast(json_extract(?1,'$.budget')AS INTEGER) AND json_extract(?1,'$.callback')=1 accepted").get(JSON.stringify(row))as{accepted:number};expect(Boolean(oracle.accepted)).toBe(row.accepted);const create=()=>api.startOwnedCommandTimeoutV1(row.budget,row.callback?()=>{}:null);if(row.accepted){const stop=create();stop();stop();}else expect(create).toThrow();}
 let expired=0;const stop=api.startOwnedCommandTimeoutV1(20,()=>expired++);stop();await new Promise(resolve=>setTimeout(resolve,40));expect(expired).toBe(0);await new Promise<void>(resolve=>api.startOwnedCommandTimeoutV1(10,()=>{expired++;resolve();}));expect(expired).toBe(1);}finally{db.close();}
});

const progressOwner = resolve(import.meta.dir, ".."), progressCorpus = (): any => JSON.parse(readFileSync(join(progressOwner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
const progressSchemaDocument = (): any => JSON.parse(readFileSync(join(progressOwner, "🧬️schema/🔣️.json"), "utf8"));
const python = process.platform === "win32" ? "python" : "python3";

test("a progress deadline follows one law: the real activity ledger, the shared function and an independent Python model agree on every case", async () => {
  const corpus = progressCorpus(), schema = progressSchemaDocument(), require = createRequire(import.meta.url), ajv = new (require("ajv").default)({ strict: false });
  ajv.addSchema(schema);
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/ProgressDeadlineCase" }), api = await import(join(progressOwner, "🟦️.ts")), watchdog = await import(join(progressOwner, "../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts"));
  const realNow = Date.now;
  let base = realNow() + 1e7;
  const actual: any[] = [];
  try {
    for (const row of corpus.progressDeadlines) {
      expect(validate(row), JSON.stringify(validate.errors)).toBe(true);
      expect(row.expected.length).toBe(row.checks.length);
      base += 1e7;
      let at = 0;
      Date.now = () => base + at;
      const deadline = api.createProgressDeadlineV1({ idleMilliseconds: row.idleMilliseconds, maximumMilliseconds: row.maximumMilliseconds }, { now: () => Date.now() - base, idle: () => watchdog.scriptProcessActivityIdleMillisecondsV1(Date.now()) });
      const events: { at: number; order: number; run: () => void }[] = [], leaves: (() => void)[] = [];
      for (const time of row.progress) events.push({ at: time, order: 1, run: () => watchdog.noteScriptProcessActivityV1(Date.now()) });
      row.waits.forEach(([from, to]: number[], index: number) => { events.push({ at: from, order: 0, run: () => { leaves[index] = watchdog.enterScriptProcessWaitV1(); } }); events.push({ at: to, order: 2, run: () => leaves[index]!() }); });
      const checks = row.checks.map((time: number) => ({ at: time, order: 3, run: () => actual.push({ remaining: deadline.remaining(), expiry: deadline.expiry() }) }));
      const mine: any[] = [];
      for (const event of [...events, ...checks].sort((a, b) => a.at - b.at || a.order - b.order)) {
        at = event.at;
        if (checks.includes(event)) { event.run(); mine.push(actual.pop()); } else event.run();
      }
      for (const leave of leaves) leave?.();
      expect(mine).toEqual(row.expected);
    }
  } finally { Date.now = realNow; }
  const oracle = spawnSync(python, ["-I", "-c", `import sys,json
rows=json.load(sys.stdin); out=[]
for r in rows:
    res=[]
    for t in r['checks']:
        last=0
        for p in r['progress']:
            if p<=t: last=max(last,p)
        for a,b in r['waits']:
            if a<=t: last=max(last,min(t,b))
        mx=r['maximumMilliseconds']-t; idle=r['idleMilliseconds']-(t-last)
        rem=min(mx,idle)
        res.append({'remaining':rem,'expiry':('maximum' if mx<=0 else 'idle' if idle<=0 else None)})
    out.append(res)
print(json.dumps(out))`], { input: JSON.stringify(corpus.progressDeadlines), encoding: "utf8", timeout: 15000 });
  expect(oracle.status, oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(corpus.progressDeadlines.map((row: any) => row.expected));
});

test("the class policy derives the idle window and the cost-derived maximum, refusing anything else, as an independent Python model does", async () => {
  const corpus = progressCorpus(), schema = progressSchemaDocument(), require = createRequire(import.meta.url), ajv = new (require("ajv").default)({ strict: false });
  ajv.addSchema(schema);
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/ProgressDeadlineDerivationCase" }), api = await import(join(progressOwner, "🟦️.ts"));
  const derived = corpus.progressDerivations.map((row: any) => { expect(validate(row), JSON.stringify(validate.errors)).toBe(true); try { return api.progressDeadlinePolicyV1(row.kind, row.units, row.environment); } catch { return null; } });
  expect(derived).toEqual(corpus.progressDerivations.map((row: any) => row.expected));
  const oracle = spawnSync(python, ["-I", "-c", `import sys,json
doc=json.load(open(sys.argv[1])); pol=doc['$defs']['ProgressDeadlinePolicy']['const']; rows=json.load(sys.stdin); out=[]
for r in rows:
    c=pol['classes'][r['kind']]
    if r['units']<0: out.append(None); continue
    mx=min(pol['ceilingMilliseconds'],c['baseMilliseconds']+c['perUnitMilliseconds']*r['units'])
    raw=r['environment'].get(pol['idleEnvironment'])
    idle=c['idleMilliseconds']
    if raw is not None:
        if not raw.isdigit(): out.append(None); continue
        idle=int(raw)
    if idle<pol['minimumIdleMilliseconds'] or idle>pol['ceilingMilliseconds']: out.append(None); continue
    out.append({'idleMilliseconds':min(idle,mx),'maximumMilliseconds':mx})
print(json.dumps(out))`, join(progressOwner, "🧬️schema/🔣️.json")], { input: JSON.stringify(corpus.progressDerivations), encoding: "utf8", timeout: 15000 });
  expect(oracle.status, oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(derived);
});

test("a progress deadline timer re-reads the deadline when it believes it is due, so progress moves the expiry and nothing else does", async () => {
  const api = await import(join(progressOwner, "🟦️.ts")), started = performance.now();
  let lastProgress = started, expired: { reason: string; at: number } | undefined;
  const deadline = api.createProgressDeadlineV1({ idleMilliseconds: 160, maximumMilliseconds: 5000 }, { now: () => performance.now(), idle: () => performance.now() - lastProgress });
  const stop = api.startProgressDeadlineV1(deadline, (reason: string) => { expired = { reason, at: performance.now() }; });
  for (let step = 0; step < 4; step++) { await new Promise((accept) => setTimeout(accept, 100)); lastProgress = performance.now(); expect(expired).toBeUndefined(); }
  const lastRenewal = lastProgress;
  await new Promise((accept) => setTimeout(accept, 400));
  stop(); stop();
  expect(expired?.reason).toBe("idle");
  expect(expired!.at - lastRenewal).toBeGreaterThanOrEqual(150);
  expect(expired!.at - started).toBeGreaterThan(400);
  const capped = api.createProgressDeadlineV1({ idleMilliseconds: 10000, maximumMilliseconds: 120 }, { now: () => performance.now(), idle: () => 0 });
  let reason = "";
  const stopCapped = api.startProgressDeadlineV1(capped, (why: string) => { reason = why; });
  await new Promise((accept) => setTimeout(accept, 300));
  stopCapped();
  expect(reason).toBe("maximum");
});

test("a progress deadline is refused in every selected language", async () => {
  const api = await import(join(progressOwner, "🟦️.ts")), policy = api.progressDeadlinePolicyV1("native-child", 2, {}), both = ["en", "de"] as const;
  const idle = api.progressDeadlineNoticeV1("idle", "native:owner-command", policy, 31 * 60000, 2, both), maximum = api.progressDeadlineNoticeV1("maximum", "native:owner-command", policy, 0, 2, both);
  expect(idle).toContain("native:owner-command stopped: no progress for 31 min (idle window 60 min)");
  expect(idle).toContain("native:owner-command gestoppt: seit 31 min kein Fortschritt (Leerlauffenster 60 min)");
  expect(maximum).toContain("maximum duration of 260 min derived from its declared cost (2 unit(s))");
  expect(maximum).toContain("Höchstdauer von 260 min ist erreicht");
  expect(api.progressDeadlineNoticeV1("idle", "x", policy, 0, 0, ["de"])).not.toContain("stopped");
});
