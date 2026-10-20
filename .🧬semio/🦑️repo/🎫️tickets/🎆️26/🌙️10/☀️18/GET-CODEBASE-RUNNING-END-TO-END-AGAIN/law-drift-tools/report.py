import json,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r1=[json.loads(l) for l in open(C+"/🗑️generated/law-drift-run/r1.jsonl")]
idx={r['file']:i for i,r in enumerate(r1)}
r3={idx[json.loads(l)["file"]]:json.loads(l) for l in open(C+"/🗑️generated/law-drift-run/r6.jsonl")}
r8={idx[json.loads(l)["file"]]:json.loads(l) for l in open(C+"/🗑️generated/law-drift-run/r8.jsonl")}
led={}
for l in open(C+"/law-drift-tools/ledger.tsv",encoding='utf-8'):
    p=l.rstrip("\n").split("\t")
    if len(p)>=5: led[p[0]]=p
rows=[]
for i in range(117):
    r=r3[i]; live=("green" if r['rc']==0 else f"red {r['pass_']}/{r['fail']}")+" / r8 "+("green" if r8[i]['rc']==0 else "red")
    p=led.get(str(i)); short=r1[i]['file'].replace('🧰️framework/','FW/').replace('🛍️products/','P/').replace('🦑️repo/🔨️modules/📚️library','REPO-LIB')
    rows.append((i,short,p[2] if p else ("(not red after the harness change / already fixed by peers)" if r['rc']==0 else "(not triaged)"),p[3] if p else "",live,p[4] if p else ""))
out=["# Result law-drift: triage of the 117 law-drift reds and 9 non-library real defects (2026-10-20)","",
"Executor law-drift, foreground, no sub-agents, no cargo/wasm build, no `bun repo run`, no git modification. Coordinator write pause at ~04:50: from then on every fix is STAGED under `C/law-drift-tools/staged/` (apply with `C/law-drift-tools/staged/apply.sh`, dry-run first) and validated in a symlink shadow tree (`shadow.py`/`stage.py`); nothing under the frozen paths was edited.","",
"## Totals (live tree, run 6 (last environment-consistent) and run 8 (current), `law-drift-tools/run.py`, per-suite finite envelope + short artifact dir)","",
f"- 117 suites: {sum(1 for r in r3.values() if r['rc']==0)} green in r6, {sum(1 for r in r8.values() if r['rc']==0)} in r8 (2 at 03:00): edits applied to non-library laws/fixtures before the pause, the harness change, and peer fixes.",
"- the 24 staged patches were APPLIED after the first pause lifted (22 applied; `cc-project` and `cc-test` skipped because project.json stays frozen).",
"- Remaining reds are classified: envelope/launcher-gated, needs cargo/rustc, timing under load, taxonomy-normalizer churn (tax owners), or real code/data defects listed in the next section.","",
"## Harness findings (affect every future census)","",
"1. `SEMIO_TEST_ARTIFACT_DIR` must be short (<256 UTF-16 units for lease paths) AND, for some laws, below `.🧬semio/🦑️repo/🎫️tickets/🎆️YY/🌙️MM/☀️DD/<slug>/🗑️generated`. Use `C/🗑️generated/l/<8 chars>` (135 units).",
"2. The launcher envelope must be FINITE (`maximumElapsedMilliseconds` > 0): owned interactive commands refuse an infinite control. The runner mints one envelope per suite.",
"3. `receiveScriptProcessInvocation(process.env, …)` admits the process birth ONCE per process: tests with several such calls must mint envelopes with `withScriptProcessEnvelope(createScriptProcessEnvelope(...))` (fixed in 2 suites; `law-drift-tools/birth.py`).",
"4. TypeScript 5.9.3 is installed (root pins ^5.9.3) but several laws call `tsc --ignoreConfig` (TS >= 6): made conditional on the resolved major.",
"5. Bun 1.3.14 `Bun.TOML.parse` diverges from the spec (`\\\\UXXXXXXXX` unsupported, surrogate escapes accepted); smol-toml is correct.",
"6. tree-sitter-rust grammar 0.20 (tree-sitter-wasms 0.1.13) cannot parse async closures (`async |x|`) used repo-wide: laws now ignore ERROR nodes that are exactly `async`. Real fix = grammar bump (deps owner).","",
"## Per suite (idx = row in tst7 law-drift list; suite -> cause -> fix -> now)","",
"| idx | suite | cause | fix | now r6 / r8 | note |","|---|---|---|---|---|---|"]
for i,s,c,f,n,note in rows:
    esc=lambda x:x.replace("|","/").replace("\n"," ")
    out.append(f"| {i} | `{esc(s)[-100:]}` | {esc(c)} | {esc(f)} | {n} | {esc(note)} |")
out+=open(C+"/law-drift-tools/report-tail.md",encoding='utf-8').read().split("\n")
open(C+"/result-law-drift.md","w",encoding='utf-8').write("\n".join(out)+"\n")
print("ok",len(out))
