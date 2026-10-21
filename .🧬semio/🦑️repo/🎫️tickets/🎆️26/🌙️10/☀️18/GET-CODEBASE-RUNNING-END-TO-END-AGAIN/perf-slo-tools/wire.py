#!/usr/bin/env python3
"""Anchored edits of existing files that wire `repo bench` into the CLI and into the CI workflow. Every edit is a pure function text->text: it is idempotent (already applied -> unchanged), refuses when its anchor is missing or ambiguous, and never rewrites unrelated text, so it composes with the other staged sets (lists are widened in place, not replaced as a whole)."""
import json,os,re,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
B=names.B;OPS=f"{B}/{names.OPERATIONS}"
class Drift(Exception):pass
def once(text,anchor,label):
    n=text.count(anchor)
    if n!=1:raise Drift(f"{label}: anchor found {n} times: {anchor[:90]!r}")
def after(text,anchor,add,label,mark=None):
    if (mark or add) in text:return text
    once(text,anchor,label);return text.replace(anchor,anchor+add,1)
def before(text,anchor,add,label,mark=None):
    if (mark or add) in text:return text
    once(text,anchor,label);return text.replace(anchor,add+anchor,1)
def sub_optional(text,old,new):
    return text.replace(old,new,1) if old in text and new not in text else text

FLAGS='''    {
      "name": "--slo",
      "value": "required",
      "valueName": "ids|all",
      "operations": ["bench"],
      "description": { "en": "bench run: comma list of objective ids, probe:<id>, kind:<cli-help|noop|edit|graph|cache|concurrency>, or all.", "de": "bench run: Kommaliste von Ziel-IDs, probe:<id>, kind:<cli-help|noop|edit|graph|cache|concurrency> oder all." }
    },
    {
      "name": "--runs",
      "value": "required",
      "valueName": "n",
      "operations": ["bench"],
      "description": { "en": "bench run: samples per probe instead of the authored count (the 95th percentile needs 20 samples, the 99th needs 100; fewer leave those objectives unproven).", "de": "bench run: Stichproben je Messung statt der verfassten Anzahl (das 95. Perzentil braucht 20 Stichproben, das 99. braucht 100; bei weniger bleiben diese Ziele unbewiesen)." }
    },
    {
      "name": "--baseline",
      "value": "required",
      "valueName": "path",
      "operations": ["bench"],
      "description": { "en": "bench run|compare: a stored report to compare with; a change beyond the tolerance of an objective is a regression or an improvement.", "de": "bench run|compare: ein gespeicherter Bericht als Referenz; eine Änderung über die Toleranz eines Ziels hinaus ist eine Verschlechterung oder Verbesserung." }
    },
    {
      "name": "--save-baseline",
      "value": "required",
      "valueName": "path",
      "operations": ["bench"],
      "description": { "en": "bench run: store the report as the baseline of later runs.", "de": "bench run: speichert den Bericht als Referenz für spätere Läufe." }
    },
    {
      "name": "--current",
      "value": "required",
      "valueName": "path",
      "operations": ["bench"],
      "description": { "en": "bench compare: the stored report to judge against --baseline.", "de": "bench compare: der gespeicherte Bericht, der gegen --baseline beurteilt wird." }
    },
    {
      "name": "--markdown",
      "value": "required",
      "valueName": "path",
      "operations": ["bench"],
      "description": { "en": "bench run|compare: also write the report as a markdown file.", "de": "bench run|compare: schreibt den Bericht zusätzlich als Markdown-Datei." }
    },
    {
      "name": "--max-load",
      "value": "required",
      "valueName": "per-core",
      "operations": ["bench"],
      "description": { "en": "bench run: refuse to start when the one-minute load per core of the host exceeds this number.", "de": "bench run: startet nicht, wenn die Einminutenlast je Kern des Rechners diese Zahl überschreitet." }
    },
    {
      "name": "--prepare",
      "value": "none",
      "operations": ["bench"],
      "description": { "en": "bench run: discard and prepare the disposable copy first, so that the cold graph probe starts from a copy without graph.", "de": "bench run: verwirft die Wegwerf-Kopie und bereitet sie zuerst vor, damit die Messung des kalten Graphen von einer Kopie ohne Graph startet." }
    },
'''
OPERATION='''    {
      "name": "bench",
      "requires": "none",
      "synopsis": "bench list|run|compare",
      "summary": { "en": "Measure the performance objectives (hot-cache latency, edit to rebuild per language, cache hit rate, graph, memory, concurrent agents) in a disposable checkout copy, judge them against the SLO table and compare with a stored baseline.", "de": "Misst die Leistungsziele (Latenz bei heißem Cache, Änderung bis Neubau je Sprache, Cache-Trefferquote, Graph, Speicher, gleichzeitige Agenten) in einer Wegwerf-Kopie des Checkouts, beurteilt sie anhand der Zieltabelle und vergleicht mit einer gespeicherten Referenz." },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "notes": [
        {"en": "Prepare the disposable copy first with `repo incremental prepare` (or pass --prepare).", "de": "Zuerst die Wegwerf-Kopie mit `repo incremental prepare` vorbereiten (oder --prepare übergeben)."},
        {"en": "Only blocking objectives make the exit code fail; advisory objectives, recorded values and everything unproven are reported.", "de": "Nur verbindliche Ziele lassen den Exit-Code fehlschlagen; beratende Ziele, erfasste Werte und alles Unbewiesene werden berichtet."}
      ],
      "subcommands": [
        { "name": "list", "synopsis": "bench list", "flags": ["--json"], "summary": { "en": "List the objectives with their probe, goal, gate and enforcement.", "de": "Listet die Ziele mit Messung, Zielwert, Schwelle und Durchsetzung auf." } },
        { "name": "run", "synopsis": "bench run [--slo=<ids|all>] [--runs=<n>] [--baseline=<report>] [--save-baseline=<report>] [--prepare]", "flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json"], "summary": { "en": "Measure the selected objectives with every implementation, print percentiles, host load and verdicts, and compare with the baseline.", "de": "Misst die gewählten Ziele mit jeder Implementierung, gibt Perzentile, Rechnerlast und Urteile aus und vergleicht mit der Referenz." } },
        { "name": "compare", "synopsis": "bench compare --current=<report> --baseline=<report>", "flags": ["--current", "--baseline", "--markdown", "--json"], "summary": { "en": "Compare two stored reports of the same host class and name every regression beyond the tolerance of its objective.", "de": "Vergleicht zwei gespeicherte Berichte derselben Rechnerklasse und nennt jede Verschlechterung über die Toleranz ihres Ziels hinaus." } }
      ],
      "flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json"],
      "examples": [
        { "command": "repo bench list", "description": { "en": "List the performance objectives.", "de": "Listet die Leistungsziele auf." } },
        { "command": "repo bench run --slo=kind:noop --baseline=bench-baseline.json", "description": { "en": "Measure the warm no-op objectives and compare with the stored baseline.", "de": "Misst die Ziele für den warmen Leerlauf und vergleicht mit der gespeicherten Referenz." } }
      ]
    },
'''
def widen(text,flag,token,after_token="concurrency"):
    i=text.find(f'"name": "{flag}",')
    if i<0:raise Drift(f"catalog {flag}: flag missing")
    j=text.index('"operations": [',i);k=text.index("]",j)+1
    row=text[j:k]
    if f'"{token}"' in row:return text
    if f'"{after_token}"' in row:row=row.replace(f'"{after_token}"',f'"{after_token}", "{token}"',1)
    else:row=row[:-1]+f', "{token}"]'
    return text[:j]+row+text[k:]
def catalog(text):
    text=before(text,'    {\n      "name": "--scope",\n',FLAGS,"catalog flags",mark='"name": "--slo"')
    text=before(text,'    {\n      "name": "dependencies",\n      "requires": "none",\n',OPERATION,"catalog operation",mark='"name": "bench",\n      "requires"')
    for flag in ("--file","--json","--implementations","--root"):text=widen(text,flag,"bench")
    text=sub_optional(text,'"incremental and concurrency plan|run: implementations to judge (default: both).", "de": "incremental und concurrency plan|run: zu prüfende Implementierungen (Standard: beide)."','"incremental, concurrency and bench plan|run: implementations to judge (default: both).", "de": "incremental, concurrency und bench plan|run: zu prüfende Implementierungen (Standard: beide)."')
    text=sub_optional(text,'"incremental, concurrency: the disposable repository copy','"incremental, concurrency, bench: the disposable repository copy')
    text=sub_optional(text,'"incremental, concurrency: die Wegwerf-Kopie des Repositorys','"incremental, concurrency, bench: die Wegwerf-Kopie des Repositorys')
    return text
def enum_after(text,label):
    m=re.search(r'"incremental", "concurrency",( "[a-z-]+",)*',text)
    if not m:raise Drift(f"{label}: the operation list with incremental and concurrency is missing")
    if '"bench"' in m.group(0):return text
    if len(re.findall(r'"incremental", "concurrency",',text))!=1:raise Drift(f"{label}: ambiguous operation list")
    return text.replace('"incremental", "concurrency",','"incremental", "concurrency", "bench",',1)
def schema_enum(text):return enum_after(text,"operations schema enum")
def requires(text):return enum_after(text,"requires fixture")
def script(text):
    anchor='await runConcurrencyV1(root,selected.args,parent);return;}\n'
    add=' if(selected.args[0]==="bench"){const{runBenchV1}=await import("./%s/%s/%s/%s");await runBenchV1(root,selected.args,parent);return;}\n'%(names.INCR,names.BENCH,names.COMMAND,names.TS)
    return after(text,anchor,add,"build script dispatch",mark='selected.args[0]==="bench"')
def child(text):
    pattern=re.compile(r'\[("[a-z-]+",)*"concurrency"(,"[a-z-]+")*\]\.includes\(command\?\?""\)')
    m=pattern.search(text)
    if not m:raise Drift("child nx exclusion: the command list with concurrency is missing")
    if '"bench"' not in m.group(0):text=text[:m.end()-len('].includes(command??"")')]+',"bench"'+text[m.end()-len('].includes(command??"")'):]
    anchor='concurrencyArgumentsV1(resolved.args.slice(1));}}\n'
    add='  if(resolved.args[0]==="bench"){const {buildInvocationRequestsHelpV1}=await import("../../%s/%s");if(!buildInvocationRequestsHelpV1(resolved.args)){const {benchArgumentsV1}=await import("../../%s/%s/%s/%s");benchArgumentsV1(resolved.args.slice(1));}}\n'%(names.INVOCATION,names.TS,names.INCR,names.BENCH,names.COMMAND,names.TS)
    return after(text,anchor,add,"child bootstrap arguments",mark='resolved.args[0]==="bench"')

HELP=json.load(open(os.path.join(here,"help-snapshots.json"),encoding="utf8")) if os.path.exists(os.path.join(here,"help-snapshots.json")) else None
def helpfixture(text):
    if HELP is None:raise Drift("help-snapshots.json missing: run render-help.py")
    d=json.loads(text)
    for lang,rows in HELP.items():
        s=d["snapshots"][lang]
        for key,row in rows["changed"].items():
            if s[key]==row["new"]:continue
            if s[key]!=row["old"]:raise Drift(f"help fixture {lang}/{key}: neither the old nor the new rendering (an earlier set changed it: run render-help.py again)")
            s[key]=row["new"]
        if "bench" in s:continue
        rebuilt={}
        for key,value in s.items():
            rebuilt[key]=value
            if key=="concurrency run":rebuilt.update(rows["pages"])
        if "bench" not in rebuilt:raise Drift(f"help fixture {lang}: the page `concurrency run` is missing")
        d["snapshots"][lang]=rebuilt
    return json.dumps(d,ensure_ascii=False,indent=2)+"\n"
EDITS=[(f"{OPS}/{names.CATALOG}",catalog),(f"{OPS}/{names.SCHEMA}/{names.JSON}",schema_enum),(f"{OPS}/{names.TESTS}/{names.REQUIRES}/{names.JSON}",requires),
 (f"{OPS}/{names.TESTS}/{names.FIXTURES}/{names.HELP}/{names.JSON}",helpfixture),(f"{B}/{names.SCRIPT}",script),(f"{B}/{names.CALLER}/{names.CHILD}/{names.TS}",child)]

WORKFLOW=".github/workflows/build-system.yml"
PINS={"checkout":"actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1","cache":"actions/cache/restore@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0","save":"actions/cache/save@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0","upload":"actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2"}
HEADER_ANCHOR="#              and the sample target must be green and cached under Nx.\n"
HEADER_ADD="""#   bench    - per operating system (linux, macos), after smoke, NOT blocking (continue-on-error): `bun repo bench run` measures the
#              performance objectives (command line help, warm no-op, edit to rebuild per language, cache hit rate, graph,
#              memory, concurrent agents) in a disposable copy with every implementation, prints percentiles with the host load,
#              compares with the baseline of the last default-branch run on the same runner class and uploads the JSON and
#              markdown report; only default-branch pushes store a new baseline. Objectives marked blocking in the table
#              fail the step but never the workflow until the job loses continue-on-error.
"""
JOB='''
  bench:
    name: bench (${{ matrix.name }})
    needs: [smoke]
    runs-on: ${{ matrix.os }}
    timeout-minutes: 120
    continue-on-error: true
    strategy:
      fail-fast: false
      matrix:
        include:
          - name: linux
            os: ubuntu-latest
          - name: macos
            os: macos-latest
    env:
      BENCH_BASELINE: ${{ runner.temp }}/bench-baseline.json
      BENCH_REPORT: ${{ runner.temp }}/bench-report.json
      BENCH_MARKDOWN: ${{ runner.temp }}/bench-report.md
    steps:
      - uses: %(checkout)s
        with:
          persist-credentials: false
      - uses: ./.github/actions/bun-repo
        with:
          bun-version: ${{ env.BUN_VERSION }}
          node-version: ${{ env.NODE_VERSION }}
      - name: Preflight
        run: bun repo preflight
      - name: Provision every build tool
        run: bun repo tools setup all
      - name: Compute the cache keys
        id: keys
        run: |
          echo "bun=bun-store-${RUNNER_OS}-${RUNNER_ARCH}-${{ hashFiles('bun.lock', '*/bun.lock', 'package.json') }}" >> "$GITHUB_OUTPUT"
          echo "baseline=bench-baseline-${RUNNER_OS}-${RUNNER_ARCH}" >> "$GITHUB_OUTPUT"
      - name: Restore the Bun store
        uses: %(cache)s
        with:
          path: .cache/b/d/m/bun-cache
          key: ${{ steps.keys.outputs.bun }}
          restore-keys: bun-store-${{ runner.os }}-${{ runner.arch }}-
      - name: Install the authored dependency scopes
        run: bun repo dependencies sync --all
      - name: Restore the baseline of this runner class
        uses: %(cache)s
        with:
          path: ${{ env.BENCH_BASELINE }}
          key: ${{ steps.keys.outputs.baseline }}-${{ github.sha }}
          restore-keys: ${{ steps.keys.outputs.baseline }}-
      - name: List the performance objectives
        run: bun repo bench list
      - name: Prepare the disposable copy
        run: bun repo incremental prepare --fresh
      - name: Measure the objectives and compare with the baseline
        env:
          EVENT: ${{ github.event_name }}
          REF: ${{ github.ref }}
          DEFAULT_REF: refs/heads/${{ github.event.repository.default_branch }}
        run: |
          args=(--slo=all --file="$BENCH_REPORT" --markdown="$BENCH_MARKDOWN")
          if [ -f "$BENCH_BASELINE" ]; then args+=(--baseline="$BENCH_BASELINE"); fi
          if [ "$EVENT" = push ] && [ "$REF" = "$DEFAULT_REF" ]; then args+=(--save-baseline="$BENCH_BASELINE.next"); fi
          bun repo bench run "${args[@]}"
      - name: Show the report
        if: ${{ !cancelled() }}
        run: |
          if [ -f "$BENCH_MARKDOWN" ]; then cat "$BENCH_MARKDOWN" >> "$GITHUB_STEP_SUMMARY"; fi
      - name: Upload the reports
        if: ${{ !cancelled() }}
        uses: %(upload)s
        with:
          name: bench-${{ matrix.name }}
          path: |
            ${{ env.BENCH_REPORT }}
            ${{ env.BENCH_MARKDOWN }}
          if-no-files-found: ignore
          retention-days: 30
      - name: Promote the new baseline (default branch)
        id: promote
        if: ${{ !cancelled() && github.event_name == 'push' && github.ref == format('refs/heads/{0}', github.event.repository.default_branch) }}
        run: |
          if [ -f "$BENCH_BASELINE.next" ]; then mv "$BENCH_BASELINE.next" "$BENCH_BASELINE"; echo "promoted=true" >> "$GITHUB_OUTPUT"; fi
      - name: Save the baseline (default branch)
        if: ${{ steps.promote.outputs.promoted == 'true' }}
        uses: %(save)s
        with:
          path: ${{ env.BENCH_BASELINE }}
          key: ${{ steps.keys.outputs.baseline }}-${{ github.sha }}
'''%PINS
def workflow(text):
    text=after(text,HEADER_ANCHOR,HEADER_ADD,"workflow header",mark="#   bench    -")
    if "\n  bench:\n" in text:return text
    if not text.endswith("\n"):text+="\n"
    once(text,"\n  parity:\n","workflow parity job")
    return text+JOB
WORKFLOW_EDITS=[(WORKFLOW,workflow)]
