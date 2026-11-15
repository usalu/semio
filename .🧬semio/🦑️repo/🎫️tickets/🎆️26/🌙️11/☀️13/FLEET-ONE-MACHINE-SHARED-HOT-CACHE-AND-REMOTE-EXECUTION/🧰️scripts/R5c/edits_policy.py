LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PC=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/"
EDITS=[
(PC+"🔣️.json",'''        "resources": [],
        "view": "native",
        "contract": {
          "en": "The cargo driver every Rust target runs its compiles through (the native command fallback of every native target).",
          "de": "Der Cargo-Treiber, über den jedes Rust-Target seine Kompilate ausführt (der native Befehls-Fallback jedes nativen Targets)."''',
'''        "resources": [],
        "view": "native",
        "selection": "included",
        "contract": {
          "en": "The cargo driver every Rust target runs its compiles through (the native command fallback of every native target). A command that runs the driver with some router words is covered by the whole driver closure.",
          "de": "Der Cargo-Treiber, über den jedes Rust-Target seine Kompilate ausführt (der native Befehls-Fallback jedes nativen Targets). Ein Befehl, der den Treiber mit einzelnen Router-Wörtern ausführt, ist durch den ganzen Treiberabschluss gedeckt."''',1),
(PC+"🧬️schema/🔣️.json",'''        "view": {
          "enum": [
            "scheduling",
            "native"
          ]
        },
        "contract": {
          "$ref": "#/definitions/Contract"
        }
      }
    },
    "Relative": {''','''        "view": {
          "enum": [
            "scheduling",
            "native"
          ]
        },
        "selection": {
          "const": "included"
        },
        "contract": {
          "$ref": "#/definitions/Contract"
        }
      }
    },
    "Relative": {''',1),
]
INF=LIB+"🏗️build/🕸️graph/📥️inference/"
EDITS+=[
(INF+"🔖️stamps/🧬️schema/🔣️.json",'"stamp": "semio.graph-inferrer.script-inputs/3"','"stamp": "semio.graph-inferrer.script-inputs/4"',1),
(INF+"🧪️tests/🔖️output-stamps/🔣️.json",'"semio.graph-inferrer.script-inputs/3": {','"semio.graph-inferrer.script-inputs/4": {',1),
]
import posixpath
TOOLS={
 "driver-stages":("../../../../../💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts","The plugin driver stages script that every plugin driver stage target runs.","Das Treiberstufen-Skript, das jedes Plugin-Treiberstufen-Target ausführt."),
 "testing-framework":("../../📦️packages/🟦️typescript/🧪️testing/🟦️.ts","The testing framework module that the package scripts of Go, TypeScript and .NET packages load to run their tests, with everything the test runners import.","Das Testgerüst-Modul, das die Paketskripte von Go-, TypeScript- und .NET-Paketen zum Ausführen ihrer Tests laden, mit allem, was die Test-Läufer importieren."),
 "process-router":("../../../../../../🔨️modules/🏃️process/📜️script.ts","The process module router script, the command framework that the commands of the repository tooling load.","Das Router-Skript des Prozessmoduls, das Befehlsgerüst, das die Befehle der Repository-Werkzeuge laden."),
}
def tool_block(name):
    entry,en,de=TOOLS[name]
    return '''      "%s": {
        "entries": [
          "%s"
        ],
        "resources": [],
        "view": "scheduling",
        "selection": "included",
        "contract": {
          "en": "%s",
          "de": "%s"
        }
      },
'''%(name,entry,en,de)
EDITS.append((PC+"🔣️.json",'      "cargo-lock-owner": {\n',"".join(tool_block(n) for n in sorted(TOOLS))+'      "cargo-lock-owner": {\n',1))
CORPUS=PC+"🧪️tests/🔣️.json"
def corpus_block(name):
    entry=TOOLS[name][0]
    return '''    "%s": {
      "entries": [
        "%s"
      ],
      "resources": []
    },
'''%(name,posixpath.normpath(posixpath.join("lib/providers",entry)))
EDITS.append((CORPUS,'    "cargo-lock-owner": {\n      "entries": [',"".join(corpus_block(n) for n in sorted(TOOLS))+'    "cargo-lock-owner": {\n      "entries": [',1))
EDITS.append((CORPUS,'  "files": {\n','  "files": {\n    "📦️packages/🟦️typescript/🧪️testing/🟦️.ts": "export const testing = 1;\\n",\n',1))
CF=INF+"🧪️tests/🧷️closure-facts/🔣️.json"
EDITS+=[
(CF,'''   "name": "an entry script is admitted by its physical shape",
   "function": "targetScriptSources",''','''   "name": "an entry script is admitted by its physical shape",
   "function": "targetScriptEntriesV1",''',1),
(CF,'''   "name": "the project's own manifest is read through its declared field, not by its bytes",
   "function": "targetScriptSources",''','''   "name": "the project's own manifest is read through its declared field, not by its bytes",
   "function": "targetScriptEntriesV1",''',1),
]
EDITS+=[
(CF,'''  "targetScriptSources": {
   "file": "scriptInputs",
   "start": "export async function targetScriptSources(",''','''  "targetScriptEntriesV1": {
   "file": "scriptInputs",
   "start": "export async function targetScriptEntriesV1(",
   "end": "const toolTablesV1 = new WeakMap();"
  },
  "targetScriptSources": {
   "file": "scriptInputs",
   "start": "export async function targetScriptSources(",''',1),
(CF,'''   "name": "no entry script is read for its bytes",
   "function": "targetScriptSources",''','''   "name": "no entry script is read for its bytes",
   "function": "targetScriptEntriesV1",''',1),
]
