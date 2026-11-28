#!/usr/bin/env python3
"""H83: anchored, atomic patch of the fleet-status code (summary view, detail filters, section gating) and the CLI catalogue. Usage: apply.py <fleet-status dir> <catalog.json> <block.ts>"""
import json, os, sys
module, catalog, block = sys.argv[1:4]
SCRIPTS = os.path.dirname(os.path.abspath(__file__))

pending = {}
def patch(path, edits):
    text = open(path, encoding="utf-8").read()
    for old, new in edits:
        assert text.count(old) == 1, (path, old[:80], text.count(old))
        text = text.replace(old, new)
    pending[path] = text

main = os.path.join(module, "🟦️.ts")
if "export function summarizeFleetReportV1" in open(main, encoding="utf-8").read():
    sys.exit("already applied")
gate = [
 ('export function formatFleetReportV1(language:FleetLanguageV1,report:FleetReportV1,limit:number=20):readonly string[]{\n const say=', 'export function formatFleetReportV1(language:FleetLanguageV1,report:FleetReportV1,limit:number=20,sections?:ReadonlySet<string>):readonly string[]{\n const want=(name:string):boolean=>sections===undefined||sections.has(name),say='),
 (' lines.push(say("header",', ' if(want("summary"))lines.push(say("header",'),
 (' lines.push(memory.sampleAgeSeconds===null?', ' if(want("memory"))lines.push(memory.sampleAgeSeconds===null?'),
 (' if(warden!==undefined&&warden!==null&&!warden.absent)lines.push', ' if(want("memory")&&warden!==undefined&&warden!==null&&!warden.absent)lines.push'),
 (' if(report.runs.length===0)lines.push(say("runsNone"));\n else{', ' if(!want("runs")){}\n else if(report.runs.length===0)lines.push(say("runsNone"));\n else{'),
 (' lines.push(say("queuesHeader"));', ' if(want("queues")){\n lines.push(say("queuesHeader"));'),
 (' if(report.queues.leasesOmitted>0)lines.push(say("queue.leasesOmitted",{count:report.queues.leasesOmitted}));', ' if(report.queues.leasesOmitted>0)lines.push(say("queue.leasesOmitted",{count:report.queues.leasesOmitted}));\n }'),
 (' if(scheduler!==undefined){', ' if(want("scheduler")&&scheduler!==undefined){'),
 (' if(fleet!==undefined){\n  lines.push(say("fleet",', ' if(want("fleet")&&fleet!==undefined){\n  lines.push(say("fleet",'),
 (' if(workers!==undefined){\n  const counts=', ' if(want("workers")&&workers!==undefined){\n  const counts='),
 (' if(cas!==undefined){\n  lines.push(say("cas",', ' if(want("cas")&&cas!==undefined){\n  lines.push(say("cas",'),
 (' if(retention===null)lines.push(say("retention.none"));\n else if(retention!==undefined){', ' if(!want("retention")){}\n else if(retention===null)lines.push(say("retention.none"));\n else if(retention!==undefined){'),
 (' if(offload!==undefined){\n  lines.push(say("offload",{local', ' if(want("offload")&&offload!==undefined){\n  lines.push(say("offload",{local'),
 (' if(rates!==undefined&&Object.keys(rates.fleet).length>0){', ' if(want("rates")&&rates!==undefined&&Object.keys(rates.fleet).length>0){'),
 (' lines.push(say("bazel",{live:bazel.live,rss:bazel.rssMegabytes,held:bazel.held,slots:bazel.slots}));\n for(const server of', ' if(want("bazel"))lines.push(say("bazel",{live:bazel.live,rss:bazel.rssMegabytes,held:bazel.held,slots:bazel.slots}));\n if(want("bazel"))for(const server of'),
 (' lines.push(say("storage",{free:', ' if(want("storage"))lines.push(say("storage",{free:'),
 (' for(const image of storage.images.filter(', ' if(want("storage"))for(const image of storage.images.filter('),
 (' lines.push(say("sealed",{mode:sealed.mode,count:sealed.entries.length,', ' if(want("sealed"))lines.push(say("sealed",{mode:sealed.mode,count:sealed.entries.length,'),
 (' for(const entry of sealed.entries)lines.push(', ' if(want("sealed"))for(const entry of sealed.entries)lines.push('),
 (' if(generation===null)lines.push(say("generationNone"));\n else lines.push(say("generation"', ' if(!want("generation")){}\n else if(generation===null)lines.push(say("generationNone"));\n else lines.push(say("generation"'),
 (' if(report.findings.length>0){lines.push(say("findingsHeader"))', ' if(want("findings")&&report.findings.length>0){lines.push(say("findingsHeader"))'),
 (' lines.push(failed.length===0?say("sources"', ' if(want("sources"))lines.push(failed.length===0?say("sources"'),
 ('/** 🖨️ Renders the report as compact text lines in the chosen language; `limit` bounds the run rows shown. */', open(block, encoding="utf-8").read() + '/** 🖨️ Renders the report as compact text lines in the chosen language; `limit` bounds the run rows shown, `sections` (names of the detail report) restricts the output to those sections. */'),
]
patch(main, gate)

cmd = os.path.join(module, "⌨️command", "🟦️.ts")
patch(cmd, [
 ('import {fleetTextV1,formatFleetReportV1,summarizeFleetV1,validateFleetReportV1,type FleetLanguageV1} from "../🟦️.ts";', 'import {filterFleetReportV1,fleetTextV1,formatFleetReportV1,formatFleetSummaryV1,renderFleetJsonV1,selectFleetSectionsV1,summarizeFleetReportV1,summarizeFleetV1,validateFleetReportV1,validateFleetSelectionV1,validateFleetSummaryV1,type FleetLanguageV1} from "../🟦️.ts";'),
 ('/** 📝️ The closed arguments of `repo fleet status`. */\nexport type FleetArgumentsV1=Readonly<{json:boolean;limit:number}>;', '/** 📝️ The closed arguments of `repo fleet status`: `detail` asks for the full report, `origin` and `section` select a part of it (and imply the detail view). */\nexport type FleetArgumentsV1=Readonly<{json:boolean;limit:number;detail:boolean;origin?:string;section?:string}>;'),
 ('/** 🧷️ Reads the options of `repo fleet status`;', '/** 🧷️ Reads the options of `repo fleet status` (`--json`, `--detail`, `--origin=<id>`, `--section=<name>`, `--limit=<n>`);'),
 (' let json=false,limit=DEFAULT_LIMIT;const seen', ' let json=false,detail=false,limit=DEFAULT_LIMIT,origin:string|undefined,section:string|undefined;const seen'),
 ('  const name=argument.startsWith("--limit=")?"--limit":argument;\n  if(seen.has(name)||name!=="--json"&&name!=="--limit")throw Error(both("error.syntax"));\n  seen.add(name);\n  if(name==="--json")json=true;\n  else{', '  const name=/^(--limit|--origin|--section)=/u.exec(argument)?.[1]??argument;\n  if(seen.has(name)||!["--json","--detail","--limit","--origin","--section"].includes(name))throw Error(both("error.syntax"));\n  seen.add(name);\n  if(name==="--json")json=true;\n  else if(name==="--detail")detail=true;\n  else if(name==="--origin"){origin=argument.slice(9);if(origin==="")throw Error(both("error.syntax"));}\n  else if(name==="--section"){section=argument.slice(10);const known=schema.definitions.Policy.const.summary.sections;if(!known.includes(section))throw Error(both("error.section",{section,list:known.join(", ")}));}\n  else{'),
 (' return Object.freeze({json,limit});', ' return Object.freeze({json,limit,detail,...origin===undefined?{}:{origin},...section===undefined?{}:{section}});'),
 (' if(parsed.json)write(JSON.stringify(report,null,2));\n else for(const line of formatFleetReportV1(language,report,parsed.limit))write(line);', ''' if(!parsed.detail&&parsed.origin===undefined&&parsed.section===undefined){
  const summary=summarizeFleetReportV1(report,parsed.limit),invalid=validateFleetSummaryV1(summary);
  if(invalid.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:invalid.slice(0,3).join("; ")}));
  if(parsed.json)write(renderFleetJsonV1(summary));
  else for(const line of formatFleetSummaryV1(language,summary))write(line);
  return 0;
 }
 const view=parsed.origin===undefined?report:filterFleetReportV1(report,parsed.origin),sections=parsed.section===undefined?undefined:new Set([parsed.section]);
 if(parsed.json&&parsed.origin===undefined&&parsed.section===undefined)write(JSON.stringify(report,null,2));
 else if(parsed.json){
  const selection=selectFleetSectionsV1(view,parsed.section===undefined?schema.definitions.Policy.const.summary.sections:[parsed.section],parsed.origin),invalid=validateFleetSelectionV1(selection);
  if(invalid.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:invalid.slice(0,3).join("; ")}));
  write(JSON.stringify(selection,null,2));
 }
 else for(const line of formatFleetReportV1(language,view,parsed.limit,sections))write(line);'''),
 ('/** ▶️ Runs `repo fleet status [--json] [--limit=<n>]`: reads the existing status readers in parallel, aggregates, validates the report against its closed schema and prints it as compact text in the chosen language or as one JSON document.', '/** ▶️ Runs `repo fleet status [--json] [--detail] [--origin=<id>] [--section=<name>] [--limit=<n>]`: reads the existing status readers in parallel, aggregates, validates the report against its closed schema and prints by default the summary (under 16 KB) as compact text in the chosen language or as one JSON document; the full report is behind `--detail`, a part of it behind `--origin` and `--section`.'),
])

help_path = os.path.join(os.path.dirname(catalog), "📖️help", "🟦️.ts")
if os.path.exists(help_path):
    patch(help_path, [("flag=(name:string)=>catalog.flags.find(row=>row.name===name)!;", "flag=(name:string)=>catalog.flags.find(row=>row.name===name&&row.operations?.includes(request.operation!)===true)??catalog.flags.find(row=>row.name===name)!;")])

cat = json.loads(open(catalog, encoding="utf-8").read().replace("{backends}", "@@"))
text = open(catalog, encoding="utf-8").read()
def edit(old, new):
    global text
    assert text.count(old) == 1, (old[:80], text.count(old))
    text = text.replace(old, new)
edit('''      "operations": ["fleet"],
      "description": { "en": "fleet status: how many runs the text output lists (default: 20, the JSON output always lists all).", "de": "fleet status: wie viele Läufe die Textausgabe auflistet (Standard: 20, die JSON-Ausgabe listet immer alle)." }
    },''', '''      "operations": ["fleet"],
      "description": { "en": "fleet status: how many runs the summary lists (default: 20).", "de": "fleet status: wie viele Läufe die Zusammenfassung auflistet (Standard: 20)." }
    },
    {
      "name": "--detail",
      "value": "none",
      "operations": ["fleet"],
      "description": { "en": "fleet status: print the full report (all workers, machines, counters and findings, over 100 KB with --json) instead of the summary.", "de": "fleet status: gibt den vollständigen Bericht aus (alle Worker, Maschinen, Zähler und Befunde, mit --json über 100 KB) statt der Zusammenfassung." }
    },
    {
      "name": "--origin",
      "value": "required",
      "valueName": "id",
      "operations": ["fleet"],
      "description": { "en": "fleet status: print the full report of this origin only (its worker, machine, counters and findings); implies --detail.", "de": "fleet status: gibt nur den vollständigen Bericht dieses Origins aus (Worker, Maschine, Zähler und Befunde); schließt --detail ein." }
    },
    {
      "name": "--section",
      "value": "required",
      "valueName": "''' + "|".join(json.load(open(os.path.join(module, "🧬️schema", "🔣️.json")))["definitions"]["Policy"]["const"]["summary"]["sections"]) + '''",
      "operations": ["fleet"],
      "description": { "en": "fleet status: print one section of the full report only; implies --detail.", "de": "fleet status: gibt nur einen Abschnitt des vollständigen Berichts aus; schließt --detail ein." }
    },''')
edit('''      "flags": [
        "--json",
        "--limit"
      ],''', '''      "flags": [
        "--json",
        "--detail",
        "--origin",
        "--section",
        "--limit"
      ],''')
edit('"synopsis": "fleet status [--json] [--limit=<n>]"', '"synopsis": "fleet status [--json] [--detail] [--origin=<id>] [--section=<name>] [--limit=<n>]"')
edit('"en": "Print the fleet picture as compact text, or with --json as one document whose schema is validated before it is printed.",\n            "de": "Gibt das Gesamtbild als kompakten Text aus, mit --json als ein Dokument, dessen Schema vor der Ausgabe geprüft wird."', '"en": "Print the fleet summary (under 16 KB: totals per window, anomalous counters per origin, findings with counts, workers that need a look) as compact text, or with --json as one document whose schema is validated before it is printed; --detail, --origin and --section print the full report or a part of it.",\n            "de": "Gibt die Flottenzusammenfassung (unter 16 KB: Summen je Zeitfenster, auffällige Zähler je Origin, Befunde mit Anzahl, Worker mit Handlungsbedarf) als kompakten Text aus, mit --json als ein Dokument, dessen Schema vor der Ausgabe geprüft wird; --detail, --origin und --section geben den vollständigen Bericht oder einen Teil davon aus."')
edit('''          "command": "repo fleet status --json",
          "description": { "en": "The same picture as one machine-readable document for an agent.", "de": "Dasselbe Bild als ein maschinenlesbares Dokument für einen Agenten." }
        }''', '''          "command": "repo fleet status --json",
          "description": { "en": "The same summary as one machine-readable document for an agent.", "de": "Dieselbe Zusammenfassung als ein maschinenlesbares Dokument für einen Agenten." }
        },
        {
          "command": "repo fleet status --json --origin=g1-m01 --section=workers",
          "description": { "en": "The full worker record of one machine, as the schema-validated detail document.", "de": "Der vollständige Worker-Eintrag einer Maschine als schemageprüftes Detaildokument." }
        }''')
json.loads(text.replace("{backends}", "@@"))
pending[catalog] = text
for path, content in pending.items():
    tmp = path + ".h83.tmp"
    open(tmp, "w", encoding="utf-8").write(content)
    os.replace(tmp, path)
print("ok", len(pending))
