# bzl-rbe edit set: anchored edits of existing files; new files come from targets.py. Single source of truth for mkoverlay.py and apply.py.
# edit(file, old, new): replace exactly one occurrence of old.
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from targets import L, B, S

BZ = B + "/🔧️tools/🏗️bazel"
EDITS = []


def edit(file, old, new):
    EDITS.append({"file": file, "old": old, "new": new})


OPERATION = r'''    },
    {
      "name": "remote-execution",
      "synopsis": "remote-execution serve|status|env",
      "summary": { "en": "Run, inspect or describe the optional local remote execution cluster that hermetic Bazel actions can run on.", "de": "Startet, prüft oder beschreibt den optionalen lokalen Remote-Execution-Cluster, auf dem hermetische Bazel-Actions laufen können." },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        {
          "name": "serve",
          "synopsis": "remote-execution serve",
          "summary": { "en": "Run the local cluster (storage, frontend, scheduler, worker, runner) in containers until stopped, then remove containers and volumes.", "de": "Betreibt den lokalen Cluster (Speicher, Frontend, Scheduler, Worker, Runner) in Containern bis zum Stopp und entfernt danach Container und Volumes." }
        },
        {
          "name": "status",
          "synopsis": "remote-execution status",
          "summary": { "en": "Print a RemoteExecutionServiceStatus: whether the container runtime answers, the containers of the cluster and whether the frontend answers.", "de": "Gibt einen RemoteExecutionServiceStatus aus: ob die Container-Laufzeit antwortet, die Container des Clusters und ob das Frontend antwortet." }
        },
        {
          "name": "env",
          "synopsis": "remote-execution env",
          "summary": { "en": "Print the contract variables a client sets to use the cluster, one NAME=value per line.", "de": "Gibt die Vertragsvariablen aus, die ein Client für den Cluster setzt, je Zeile NAME=Wert." }
        }
      ],
      "flags": [
        "--platform",
        "--port",
        "--diagnostics-port"
      ],
      "examples": [
        {
          "command": "repo remote-execution serve",
          "description": { "en": "Run the local cluster on 127.0.0.1:8980 with workers that match this machine's CPU.", "de": "Betreibt den lokalen Cluster auf 127.0.0.1:8980 mit Workern passend zur CPU dieses Rechners." }
        },
        {
          "command": "repo remote-execution env --platform=linux-x64",
          "description": { "en": "Print the variables that route hermetic actions to x64 Linux workers.", "de": "Gibt die Variablen aus, die hermetische Actions an x64-Linux-Worker leiten." }
        },
        {
          "command": "repo remote-execution status",
          "description": { "en": "Show the runtime, the containers and whether the frontend answers.", "de": "Zeigt Laufzeit, Container und ob das Frontend antwortet." }
        }
      ],
      "notes": [
        "Optional developer service, never a dependency: with SEMIO_REMOTE_EXECUTION_ENDPOINT unset every action stays local. The service description, schema and pinned container images live in the build module; every image is an official one and is pinned by digest.",
        "serve needs a running container runtime (Docker Desktop or the docker service) and the compose plugin. It copies the packaged stack to an ASCII-only runtime directory below the workspace, starts the cluster on loopback only (the cluster has no authentication), prints the client variables once the frontend answers and, when the caller cancels, removes containers and volumes: its state is ephemeral and local to this machine.",
        "The workers advertise the properties of their execution platform (linux-arm64 or linux-x64) and the instance name main; set the printed variables, then run hermetic targets with the Bazel implementation.",
        "serve publishes progress while it checks the runtime, renders the stack, starts the cluster and tears it down; status and env change nothing."
      ]
    }
  ]
}
'''

edit(B + "/⚙️operations/🔣️catalog.json",
     '''      "operations": ["cache"],
      "description": { "en": "cache serve: port to listen on; 0 picks a free port (default from the build policy).", "de": "cache serve: Port, auf dem gelauscht wird; 0 wählt einen freien Port (Standard aus der Build-Policy)." }
    },''',
     '''      "operations": ["cache", "remote-execution"],
      "description": { "en": "cache serve: port to listen on; 0 picks a free port (default from the build policy). remote-execution: loopback port of the cluster frontend (default 8980).", "de": "cache serve: Port, auf dem gelauscht wird; 0 wählt einen freien Port (Standard aus der Build-Policy). remote-execution: Loopback-Port des Cluster-Frontends (Standard 8980)." }
    },''')
edit(B + "/⚙️operations/🔣️catalog.json",
     '''      "description": { "en": "cache serve: upper bound of the store; the least recently used entries are evicted beyond it.", "de": "cache serve: Obergrenze des Speichers; darüber werden die am längsten ungenutzten Einträge entfernt." }
    }
  ],
  "operations": [''',
     '''      "description": { "en": "cache serve: upper bound of the store; the least recently used entries are evicted beyond it.", "de": "cache serve: Obergrenze des Speichers; darüber werden die am längsten ungenutzten Einträge entfernt." }
    },
    {
      "name": "--platform",
      "value": "separate",
      "valueName": "linux-arm64|linux-x64",
      "operations": ["remote-execution"],
      "description": { "en": "remote-execution: execution platform of the workers; otherwise the one that matches this machine's CPU.", "de": "remote-execution: Ausführungsplattform der Worker; sonst die zur CPU dieses Rechners passende." }
    },
    {
      "name": "--diagnostics-port",
      "value": "separate",
      "valueName": "port",
      "operations": ["remote-execution"],
      "description": { "en": "remote-execution: loopback port of the health route of the frontend (default 8981).", "de": "remote-execution: Loopback-Port der Gesundheitsroute des Frontends (Standard 8981)." }
    }
  ],
  "operations": [''')
edit(B + "/⚙️operations/🔣️catalog.json",
     '''serve publishes progress while it scans the store and stops at the next checkpoint when the caller cancels."
      ]
    }
  ]
}
''',
     '''serve publishes progress while it scans the store and stops at the next checkpoint when the caller cancels."
      ]
''' + OPERATION)

edit(B + "/📜️script.ts",
     '''    if(!["tools","reset","cache"].includes(command)){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}
''',
     '''    if(!["tools","reset","cache","remote-execution"].includes(command)){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}
    if(command==="remote-execution"){
      const {runRemoteExecutionCommandV1}=await import("./🏃️execution/🐳️service/🟦️.ts");
      const result=await runRemoteExecutionCommandV1(resolved.args.slice(1),{root,environment:process.env,host:{platform:process.platform,arch:process.arch},signal:phaseControl.signal,progress:async stage=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(invocation,"build:"+stage,"running");},write:line=>console.log(line)});
      if(result!==undefined)console.log(JSON.stringify(result,null,2));
      return;
    }
''')

edit("📋️project.json",
     '''            "ready": { "port": 6090, "portEnv": "SEMIO_REMOTE_CACHE_PORT", "path": "/healthz" }
          },
''',
     '''            "ready": { "port": 6090, "portEnv": "SEMIO_REMOTE_CACHE_PORT", "path": "/healthz" }
          },
          {
            "id": "build-remote-execution-status",
            "title": { "en": "Show the local remote execution cluster", "de": "Lokalen Remote-Execution-Cluster anzeigen" },
            "description": { "en": "Report whether the container runtime answers, the containers of the optional local cluster and whether its frontend answers; nothing is started.", "de": "Melden, ob die Container-Laufzeit antwortet, welche Container der optionale lokale Cluster hat und ob sein Frontend antwortet; es wird nichts gestartet." },
            "verb": "inspect",
            "command": ["bun", "repo", "remote-execution", "status"]
          },
          {
            "id": "build-remote-execution-serve",
            "title": { "en": "Run the local remote execution cluster", "de": "Lokalen Remote-Execution-Cluster betreiben" },
            "description": { "en": "Run an optional container cluster on this machine until stopped so hermetic Bazel actions can run remotely; clients set the variables printed at start.", "de": "Einen optionalen Container-Cluster auf diesem Rechner bis zum Stopp betreiben, damit hermetische Bazel-Actions entfernt laufen können; Clients setzen die beim Start ausgegebenen Variablen." },
            "verb": "serve",
            "command": ["bun", "repo", "remote-execution", "serve", "--diagnostics-port", "{port}"],
            "continuous": true,
            "ready": { "port": 8981, "portEnv": "SEMIO_RBE_DIAGNOSTICS_PORT", "path": "/-/healthy" }
          },
''')

PROBE = B + "/🏃️execution/🧪️tests/🔬️probes/🟦️.ts"
DESCRIPTOR = S + "/🔣️.json"
COMPOSE = S + "/🐳️stack/compose.yaml"
OUT = ".🧬semio/🦑️repo/⚡️cache/remote-execution"


def probe(name, description, arguments, inputs, outputs, depends=()):
    import json
    body = {"executor": "repo:script", "inputs": ["{workspaceRoot}/" + path for path in inputs], "cache": True, "outputs": ["{workspaceRoot}/" + path for path in outputs]}
    if depends:
        body["dependsOn"] = list(depends)
    body["options"] = {"cwd": ".", "command": f"bun {PROBE} {arguments}", "forwardAllArgs": False}
    body["metadata"] = {"semio": {"hermetic": {"version": 1, "tools": ["bun"]}}}
    text = json.dumps({name: body}, ensure_ascii=False, indent=2)
    return "\n".join("  " + line for line in text.split("\n")[1:-1]) + ",\n"


PROBES = "".join([
    probe("remote-execution-probe-platform", "", f"platform {OUT}/platform.json", [PROBE], [f"{OUT}/platform.json"]),
    probe("remote-execution-probe-descriptor", "", f"descriptor {OUT}/descriptor.json {DESCRIPTOR}", [PROBE, DESCRIPTOR], [f"{OUT}/descriptor.json"]),
    probe("remote-execution-probe-compose", "", f"compose {OUT}/compose.json {DESCRIPTOR} {COMPOSE}", [PROBE, DESCRIPTOR, COMPOSE], [f"{OUT}/compose.json"]),
    probe("remote-execution-probe-fanout", "", f"fanout {OUT}/fanout", [PROBE], [f"{OUT}/fanout"]),
    probe("remote-execution-probe-chain", "", f"chain {OUT}/chain.json {OUT}/platform.json {OUT}/descriptor.json {OUT}/compose.json {OUT}/fanout/index.json", [PROBE], [f"{OUT}/chain.json"],
          ["remote-execution-probe-platform", "remote-execution-probe-descriptor", "remote-execution-probe-compose", "remote-execution-probe-fanout"]),
])
edit(L + "/⚡️caching/📋️project.json",
     '''    "toolchain": {
      "executor": "repo:script",
      "cache": true,
      "outputs": [],
      "options": {
        "command": "bun ./🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts fingerprint"''',
     PROBES + '''    "toolchain": {
      "executor": "repo:script",
      "cache": true,
      "outputs": [],
      "options": {
        "command": "bun ./🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts fingerprint"''')

edit(BZ + "/🏃️execution/🧬️schema/🔣️.json",
     '''          "platform": "SEMIO_REMOTE_EXECUTION_PLATFORM",
          "jobs": "SEMIO_REMOTE_EXECUTION_JOBS"
        },
        "remote": {
          "defaultPlatform": "linux-x64",
          "defaultJobs": 128,''',
     '''          "platform": "SEMIO_REMOTE_EXECUTION_PLATFORM",
          "instance": "SEMIO_REMOTE_EXECUTION_INSTANCE",
          "jobs": "SEMIO_REMOTE_EXECUTION_JOBS"
        },
        "remote": {
          "defaultPlatform": "linux-x64",
          "defaultInstance": "main",
          "defaultJobs": 128,''')
edit(BZ + "/🏃️execution/🧬️schema/🔣️.json",
     '''            "platform",
            "jobs"
          ],
          "properties": {
            "endpoint": {
              "type": "string",
              "pattern": "^SEMIO_[A-Z0-9_]+$"
            },''',
     '''            "platform",
            "instance",
            "jobs"
          ],
          "properties": {
            "endpoint": {
              "type": "string",
              "pattern": "^SEMIO_[A-Z0-9_]+$"
            },
            "instance": {
              "type": "string",
              "pattern": "^SEMIO_[A-Z0-9_]+$"
            },''')
edit(BZ + "/🏃️execution/🧬️schema/🔣️.json",
     '''          "required": [
            "defaultPlatform",
            "defaultJobs",''',
     '''          "required": [
            "defaultPlatform",
            "defaultInstance",
            "defaultJobs",''')
edit(BZ + "/🏃️execution/🧬️schema/🔣️.json",
     '''            "defaultPlatform": {
              "type": "string"
            },''',
     '''            "defaultPlatform": {
              "type": "string"
            },
            "defaultInstance": {
              "type": "string",
              "pattern": "^[a-z0-9][a-z0-9._-]{0,63}$"
            },''')
edit(BZ + "/🏃️execution/🧬️schema/🔣️.json",
     '''        "SEMIO_REMOTE_EXECUTION_JOBS": {
          "type": "string",
          "pattern": "^[1-9][0-9]{0,3}$"
        }''',
     '''        "SEMIO_REMOTE_EXECUTION_INSTANCE": {
          "type": "string",
          "pattern": "^[a-z0-9][a-z0-9._-]{0,63}$"
        },
        "SEMIO_REMOTE_EXECUTION_JOBS": {
          "type": "string",
          "pattern": "^[1-9][0-9]{0,3}$"
        }''')


# round2 merged variant (integrate2): the two array-tail edits of the operations catalogue are made independent of what else was appended to the same arrays (mem-gov, parity),
# so the set applies before or after them. The original anchors end in the closing of the flags array and of the file.
_CATALOG = B + "/⚙️operations/🔣️catalog.json"
_rows = [e for e in EDITS if e["file"] == _CATALOG]
_flags, _operations = _rows[1], _rows[2]
_flags_tail = '\n  ],\n  "operations": ['
assert _flags["old"].endswith(_flags_tail) and _flags["new"].endswith(_flags_tail)
_x = _flags["old"][:-len(_flags_tail)]
assert _flags["new"].startswith(_x)
_flags["new"] = _x + _flags["new"][len(_x):-len(_flags_tail)]
_flags["old"] = _x
_operations["old"] = "\n    }\n  ]\n}\n"
_operations["new"] = "\n" + OPERATION
