# anchored edits of existing live files (applied after the rcache-server set). Every edit replaces exactly one region (text/region) or states a counted substitution (sub) or transforms parsed JSON (json).
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
X = B + "/🌐️remote-execution"
EDITS = []


def text(file, old, new):
    EDITS.append({"file": file, "kind": "text", "old": old, "new": new})


def region(file, start, end, new):
    EDITS.append({"file": file, "kind": "region", "start": start, "end": end, "new": new})


def sub(file, old, new, count=1):
    EDITS.append({"file": file, "kind": "sub", "old": old, "new": new, "count": count})


def js(file, fn):
    EDITS.append({"file": file, "kind": "json", "fn": fn})


# ---- build policy: the remoteExecution section
def policy(value):
    value["remoteExecution"] = {
        "version": 1,
        "environment": {
            "endpoint": "SEMIO_REMOTE_EXECUTION_ENDPOINT",
            "platform": "SEMIO_REMOTE_EXECUTION_PLATFORM",
            "instance": "SEMIO_REMOTE_EXECUTION_INSTANCE",
            "properties": "SEMIO_REMOTE_EXECUTION_PROPERTIES",
            "jobs": "SEMIO_REMOTE_EXECUTION_JOBS",
            "scheduler": "SEMIO_REMOTE_EXECUTION_SCHEDULER",
            "workerToken": "SEMIO_REMOTE_EXECUTION_WORKER_TOKEN",
        },
        "server": {
            "host": "127.0.0.1",
            "port": 8980,
            "workerPort": 8981,
            "defaultInstance": "main",
            "localWorkers": 1,
            "unmatchedSeconds": 120,
            "lingerSeconds": 5,
            "leaseSeconds": 30,
            "maximumAttempts": 3,
            "defaultActionSeconds": 3600,
            "maximumQueued": 100000,
            "maximumMessageBytes": 33554432,
            "maximumBatchBytes": 4194304,
        },
        "worker": {
            "concurrency": 0,
            "heartbeatSeconds": 5,
            "pollSeconds": 20,
            "directory": ".🧬semio/🦑️repo/⚡️cache/remote-execution",
            "materialize": "clone",
            "maximumBlobBytes": 268435456,
            "sandbox": "auto",
            "inlineOutputBytes": 4096,
        },
    }
    return value


js(B + "/🔣️policy.json", policy)

# ---- Bazel adapter: the executor doubles as the cache, so an HTTP cache flag cannot stay beside --remote_executor
BZ = B + "/🔧️tools/🏗️bazel"
EXEC = BZ + "/🏃️execution/🟦️.ts"
text(EXEC, "export interface BazelRemoteExecutionV1{endpoint:string;platform:string;properties:Readonly<Record<string,string>>;jobs:number}",
     "export interface BazelRemoteExecutionV1{endpoint:string;instance:string;platform:string;properties:Readonly<Record<string,string>>;jobs:number}")
text(EXEC, "return{endpoint,platform:platform.id,", "return{endpoint,instance:selected[names.instance]??contract.remote.defaultInstance,platform:platform.id,")
text(EXEC, ' if(remote)command.push(`--remote_executor=${remote.endpoint}`,"--noremote_local_fallback");',
     ' if(remote)command.push(`--remote_executor=${remote.endpoint}`,"--noremote_local_fallback",`--remote_instance_name=${remote.instance}`,"--noremote_cache_compression");')
text(EXEC, "/** 🛡️ Decides whether a cacheable task is a hermetic action",
     '''/** 🔗️ Reconciles the cache flags with a remote executor: Bazel refuses `--remote_executor` beside an HTTP `--remote_cache`, and the in-house executor serves the cache itself, so HTTP cache endpoints are dropped and the credential helper of the cache host is scoped to the executor host instead. */
export function bazelExecutorCacheFlagsV1(flags:readonly string[],executorEndpoint:string):string[]{
 const host=new URL(executorEndpoint.replace(/^grpc(s?):/u,"http$1:")).hostname.replace(/^\\[|\\]$/gu,"")||"localhost";
 return flags.filter(flag=>!/^--remote_cache=(?!grpcs?:)/u.test(flag)).map(flag=>{const match=/^--credential_helper=[^=]+=(.+)$/u.exec(flag);return match?`--credential_helper=${host}=${match[1]}`:flag;});
}

/** 🛡️ Decides whether a cacheable task is a hermetic action''')
SCRIPT = BZ + "/📜️script.ts"
text(SCRIPT, "bazelExecutionFlagsV1,bazelExecutionPlatformV1,", "bazelExecutionFlagsV1,bazelExecutorCacheFlagsV1,bazelExecutionPlatformV1,")
text(SCRIPT, "options.push(...execution.command);if(execution.remote)jobs=execution.jobs;}",
     "options.push(...execution.command);if(execution.remote&&remote){options.splice(0,options.length,...bazelExecutorCacheFlagsV1(options,remote.endpoint));jobs=execution.jobs;}}")
text(BZ + "/🏃️execution/🧬️schema/🔣️.json", "The cache endpoint, instance name, upload, compression, download mode and credentials stay with the remote cache contract.",
     "The instance name selects the namespace of the executor's store; the cache endpoint, upload, download mode and credentials stay with the remote cache contract, except that an HTTP cache cannot be combined with a gRPC executor, so the in-house executor serves the cache itself.")

# ---- operations catalogue: remote-execution serve|status|workers replaces the container cluster
CATALOG = B + "/⚙️operations/🔣️catalog.json"
OPERATION = r'''    {
      "name": "remote-execution",
      "requires": "none",
      "synopsis": "remote-execution serve|status|workers",
      "summary": {
        "en": "Run the in-house remote execution service that hermetic Bazel actions can run on, inspect it, or join this machine to it as a worker.",
        "de": "Betreibt den hauseigenen Remote-Execution-Dienst, auf dem hermetische Bazel-Actions laufen können, zeigt ihn an oder bindet diesen Rechner als Worker an."
      },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        {
          "name": "serve",
          "synopsis": "remote-execution serve [--bind <address>] [--grpc-port <n>] [--worker-port <n>] [--cache <url>] [--local-workers <n>] [--concurrency <n>] [--advertise <host>] [--sandbox <strategy>] [--tls-certificate <path> --tls-key <path>]", "flags": ["--bind", "--grpc-port", "--worker-port", "--cache", "--local-workers", "--concurrency", "--advertise", "--sandbox", "--work-directory", "--tls-certificate", "--tls-key"],
          "summary": {
            "en": "Run the shared store (unless one is named), the scheduler with its gRPC listener for build tools and its HTTP listener for workers, and local worker processes, until stopped; prints the variables a client sets and the command another machine joins with.",
            "de": "Betreibt den gemeinsamen Speicher (sofern keiner genannt wird), den Scheduler mit seinem gRPC-Listener für Build-Werkzeuge und seinem HTTP-Listener für Worker sowie lokale Worker-Prozesse, bis er gestoppt wird; gibt die Variablen aus, die ein Client setzt, und den Befehl, mit dem ein anderer Rechner beitritt."
          }
        },
        {
          "name": "status",
          "synopsis": "remote-execution status [--join <url>]", "flags": ["--join"],
          "summary": {
            "en": "Print a RemoteExecutionStatus: the client contract, whether the gRPC endpoint answers the protocol's capabilities call, and what the scheduler reports (workers, queues, counters).",
            "de": "Gibt einen RemoteExecutionStatus aus: den Client-Vertrag, ob der gRPC-Endpunkt den Capabilities-Aufruf des Protokolls beantwortet, und was der Scheduler meldet (Worker, Warteschlangen, Zähler)."
          }
        },
        {
          "name": "workers",
          "synopsis": "remote-execution workers --join <url> [--name <name>] [--concurrency <n>] [--sandbox <strategy>] [--properties <name=value,...>] [--work-directory <path>]", "flags": ["--join", "--name", "--concurrency", "--sandbox", "--properties", "--work-directory"],
          "summary": {
            "en": "Run a worker of this machine (or devcontainer) that joins a scheduler, runs the hermetic actions it is handed in a sandbox and reports until stopped.",
            "de": "Betreibt einen Worker dieses Rechners (oder Devcontainers), der einem Scheduler beitritt, die ihm zugeteilten hermetischen Actions in einer Sandbox ausführt und meldet, bis er gestoppt wird."
          }
        }
      ],
      "flags": ["--bind", "--grpc-port", "--worker-port", "--cache", "--local-workers", "--concurrency", "--advertise", "--sandbox", "--work-directory", "--join", "--name", "--properties", "--tls-certificate", "--tls-key"],
      "examples": [
        { "command": "repo remote-execution serve", "description": { "en": "Run the service on 127.0.0.1:8980 with the shared store and one local worker; hermetic Bazel actions can then run remotely.", "de": "Betreibt den Dienst auf 127.0.0.1:8980 mit dem gemeinsamen Speicher und einem lokalen Worker; hermetische Bazel-Actions können dann remote laufen." } },
        { "command": "repo remote-execution workers --join http://build-host:8981", "description": { "en": "Add this machine as a worker of a service that runs elsewhere.", "de": "Fügt diesen Rechner als Worker eines Dienstes hinzu, der anderswo läuft." } },
        { "command": "repo remote-execution status", "description": { "en": "Show the client contract, whether the endpoint answers and which workers are registered.", "de": "Zeigt den Client-Vertrag, ob der Endpunkt antwortet und welche Worker angemeldet sind." } }
      ],
      "notes": [
        {
          "en": "Remote execution is off for clients until SEMIO_REMOTE_EXECUTION_ENDPOINT names a grpc or grpcs origin; while it is unset every action stays local. SEMIO_REMOTE_EXECUTION_PLATFORM (default linux-x64), _INSTANCE (default main), _PROPERTIES and _JOBS are read by the Bazel implementation; serve prints the endpoint, platform and instance of this machine. Only hermetic actions are sent, with no local fallback: an unreachable service fails the build.",
          "de": "Remote-Execution ist für Clients aus, bis SEMIO_REMOTE_EXECUTION_ENDPOINT einen grpc- oder grpcs-Ursprung nennt; solange sie ungesetzt ist, bleibt jede Action lokal. SEMIO_REMOTE_EXECUTION_PLATFORM (Standard linux-x64), _INSTANCE (Standard main), _PROPERTIES und _JOBS liest die Bazel-Implementierung; serve gibt Endpunkt, Plattform und Instanz dieses Rechners aus. Nur hermetische Actions werden gesendet, ohne lokalen Rückfall: Ein nicht erreichbarer Dienst lässt den Build scheitern."
        },
        {
          "en": "The service implements the subset of the Remote Execution API v2 that Bazel needs, over gRPC on HTTP/2 with the repository's own wire codec: capabilities, execution (also waiting for an operation), action cache, content-addressable storage and ByteStream. Bazel offloads execution only over this protocol, and refuses an HTTP cache beside a gRPC executor, so the service is the cache too: it keeps its blobs and results in the shared store of repo remote-cache (started inside serve, or named with --cache), and the Bazel implementation drops its HTTP cache flags while an executor is configured. Identical actions in flight run once; results of successful actions enter the action cache.",
          "de": "Der Dienst setzt die Teilmenge der Remote Execution API v2 um, die Bazel braucht, über gRPC auf HTTP/2 mit dem eigenen Wire-Codec des Repositories: Capabilities, Ausführung (auch das Warten auf eine Operation), Action-Cache, inhaltsadressierter Speicher und ByteStream. Bazel lagert die Ausführung nur über dieses Protokoll aus und lehnt einen HTTP-Cache neben einem gRPC-Executor ab, daher ist der Dienst auch der Cache: Er hält Blobs und Ergebnisse im gemeinsamen Speicher von repo remote-cache (in serve gestartet oder mit --cache genannt), und die Bazel-Implementierung lässt ihre HTTP-Cache-Flags weg, solange ein Executor konfiguriert ist. Gleiche Actions im Flug laufen einmal; Ergebnisse erfolgreicher Actions gelangen in den Action-Cache."
        },
        {
          "en": "Workers pull work: each long-polls the scheduler's HTTP listener (default port 8981), offers its platform properties (OSFamily and Arch of its machine, plus --properties) and runs the actions whose required properties it offers, up to --concurrency at once. Every action gets a private input root built from the store, exactly its declared environment, and the sandbox of the hermetic table for the host: darwin-sandbox on macOS (no network, writes only inside the root), linux-sandbox (bubblewrap) on Linux, otherwise isolated-root, the private root alone (always the case on Windows); --sandbox pins one. A worker that stops heartbeating loses its work to another worker; cancelling a call or a timeout kills the whole process tree. --local-workers sets how many worker processes serve starts (default 1, 0 starts none).",
          "de": "Worker holen Arbeit ab: Jeder fragt den HTTP-Listener des Schedulers (Standard-Port 8981) per Long Poll, bietet die Plattform-Eigenschaften seines Rechners an (OSFamily und Arch, dazu --properties) und führt die Actions aus, deren geforderte Eigenschaften er bietet, bis zu --concurrency gleichzeitig. Jede Action erhält ein privates Eingabeverzeichnis aus dem Speicher, genau ihre deklarierte Umgebung und die Sandbox der Hermetic-Tabelle für den Host: darwin-sandbox auf macOS (kein Netzwerk, Schreiben nur im Verzeichnis), linux-sandbox (bubblewrap) auf Linux, sonst isolated-root, das private Verzeichnis allein (unter Windows immer); --sandbox legt eine fest. Ein Worker, der keine Heartbeats mehr sendet, verliert seine Arbeit an einen anderen; Abbruch eines Aufrufs oder ein Zeitlimit beendet den ganzen Prozessbaum. --local-workers legt fest, wie viele Worker-Prozesse serve startet (Standard 1, 0 startet keinen)."
        },
        {
          "en": "Tokens: the read and write tokens of repo remote-cache (SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN, SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN) also guard the gRPC listener (read: lookups and downloads; write: everything including execution), and SEMIO_REMOTE_EXECUTION_WORKER_TOKEN guards the worker API; beyond loopback --bind refuses to start without them. Workers read the cache token from SEMIO_REMOTE_CACHE_TOKEN. --tls-certificate and --tls-key switch both listeners to TLS (grpcs). serve, workers and status publish progress and stop at the next checkpoint when the caller cancels; the output follows SEMIO_LANGUAGE or --lang and is printed in both languages when neither is set.",
          "de": "Tokens: Die Lese- und Schreib-Tokens von repo remote-cache (SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN, SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN) schützen auch den gRPC-Listener (lesen: Abfragen und Downloads; schreiben: alles einschließlich Ausführung), und SEMIO_REMOTE_EXECUTION_WORKER_TOKEN schützt die Worker-API; außerhalb von Loopback startet --bind nicht ohne sie. Worker lesen das Cache-Token aus SEMIO_REMOTE_CACHE_TOKEN. --tls-certificate und --tls-key schalten beide Listener auf TLS (grpcs). serve, workers und status melden Fortschritt und halten am nächsten Kontrollpunkt an, wenn der Aufrufer abbricht; die Ausgabe folgt SEMIO_LANGUAGE oder --lang und erscheint in beiden Sprachen, wenn keins gesetzt ist."
        }
      ]
    }'''
region(CATALOG, '''    {
      "name": "remote-execution",''', "\n  ]\n}\n", OPERATION)


def flag(name, value_name, description_en, description_de, value="separate"):
    import json as _json
    lines = ['    {', f'      "name": {_json.dumps(name)},', f'      "value": {_json.dumps(value)},']
    if value_name:
        lines.append(f'      "valueName": {_json.dumps(value_name)},')
    lines += ['      "operations": ["remote-execution"],', f'      "description": {{ "en": {_json.dumps(description_en, ensure_ascii=False)}, "de": {_json.dumps(description_de, ensure_ascii=False)} }}', '    }']
    return "\n".join(lines)


FLAGS = ",\n".join([
    flag("--bind", "address", "remote-execution serve: address both listeners bind to (default 127.0.0.1); beyond loopback the cache tokens and the worker token are required.", "remote-execution serve: Adresse, an die beide Listener gebunden werden (Standard 127.0.0.1); außerhalb von Loopback sind die Cache-Tokens und das Worker-Token nötig."),
    flag("--grpc-port", "port", "remote-execution serve: port of the gRPC listener for build tools (default 8980); 0 picks a free port.", "remote-execution serve: Port des gRPC-Listeners für Build-Werkzeuge (Standard 8980); 0 wählt einen freien Port."),
    flag("--worker-port", "port", "remote-execution serve: port of the HTTP listener for workers and status (default 8981); 0 picks a free port.", "remote-execution serve: Port des HTTP-Listeners für Worker und Status (Standard 8981); 0 wählt einen freien Port."),
    flag("--cache", "url", "remote-execution serve: the shared store to use, the URL of a running repo remote-cache server; by default serve starts the store itself.", "remote-execution serve: der zu verwendende gemeinsame Speicher, die URL eines laufenden repo-remote-cache-Servers; standardmäßig startet serve den Speicher selbst."),
    flag("--local-workers", "n", "remote-execution serve: worker processes started with the service (default 1; 0 starts none).", "remote-execution serve: mit dem Dienst gestartete Worker-Prozesse (Standard 1; 0 startet keinen)."),
    flag("--concurrency", "n", "remote-execution serve|workers: actions one worker runs at the same time (default: one per CPU).", "remote-execution serve|workers: Actions, die ein Worker gleichzeitig ausführt (Standard: eine je CPU)."),
    flag("--advertise", "host", "remote-execution serve: the host name clients and workers use to reach the service (default: loopback, else this machine's name).", "remote-execution serve: der Hostname, unter dem Clients und Worker den Dienst erreichen (Standard: Loopback, sonst der Name dieses Rechners)."),
    flag("--sandbox", "auto|darwin-sandbox|linux-sandbox|isolated-root", "remote-execution serve|workers: how workers confine an action (default: the first available strategy of the hermetic table for this host).", "remote-execution serve|workers: wie Worker eine Action einsperren (Standard: die erste verfügbare Strategie der Hermetic-Tabelle für diesen Host)."),
    flag("--work-directory", "path", "remote-execution serve|workers: directory of a worker's blob cache and private input roots, relative to the workspace root (default: the remote execution home of the cache ledger).", "remote-execution serve|workers: Verzeichnis des Blob-Caches und der privaten Eingabeverzeichnisse eines Workers, relativ zum Workspace-Wurzelverzeichnis (Standard: die Remote-Execution-Heimat der Cache-Bilanz)."),
    flag("--join", "url", "remote-execution workers|status: the worker API of the scheduler, for example http://build-host:8981 (default: SEMIO_REMOTE_EXECUTION_SCHEDULER).", "remote-execution workers|status: die Worker-API des Schedulers, zum Beispiel http://build-host:8981 (Standard: SEMIO_REMOTE_EXECUTION_SCHEDULER)."),
    flag("--name", "name", "remote-execution workers: name of the worker in status and logs (default: host name and process id).", "remote-execution workers: Name des Workers in Status und Logs (Standard: Hostname und Prozess-ID)."),
    flag("--properties", "name=value,...", "remote-execution workers: platform properties the worker offers in addition to OSFamily and Arch.", "remote-execution workers: Plattform-Eigenschaften, die der Worker zusätzlich zu OSFamily und Arch anbietet."),
]) + ",\n"
region(CATALOG, '''    {
      "name": "--platform",''', '''    {
      "name": "--operation",''', FLAGS)
sub(CATALOG, '''      "operations": ["remote-cache", "remote-execution"],
      "description": { "en": "remote-cache serve: port to listen on; 0 picks a free port (default from the build policy). remote-execution: loopback port of the cluster frontend (default 8980).", "de": "remote-cache serve: Port, auf dem gelauscht wird; 0 wählt einen freien Port (Standard aus der Build-Policy). remote-execution: Loopback-Port des Cluster-Frontends (Standard 8980)." }''',
    '''      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: port to listen on; 0 picks a free port (default from the build policy).", "de": "remote-cache serve: Port, auf dem gelauscht wird; 0 wählt einen freien Port (Standard aus der Build-Policy)." }''')
sub(CATALOG, '''      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: PEM certificate chain; together with --tls-key the server speaks HTTPS.", "de": "remote-cache serve: PEM-Zertifikatskette; zusammen mit --tls-key spricht der Server HTTPS." }''',
    '''      "operations": ["remote-cache", "remote-execution"],
      "description": { "en": "remote-cache serve, remote-execution serve: PEM certificate chain; together with --tls-key the server speaks HTTPS (grpcs for the execution service).", "de": "remote-cache serve, remote-execution serve: PEM-Zertifikatskette; zusammen mit --tls-key spricht der Server HTTPS (grpcs beim Ausführungsdienst)." }''')
sub(CATALOG, '''      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: PEM private key of the certificate.", "de": "remote-cache serve: PEM-Privatschlüssel des Zertifikats." }''',
    '''      "operations": ["remote-cache", "remote-execution"],
      "description": { "en": "remote-cache serve, remote-execution serve: PEM private key of the certificate.", "de": "remote-cache serve, remote-execution serve: PEM-Privatschlüssel des Zertifikats." }''')

# ---- the script: one branch runs the command; local workers are processes of the module's own entry, so they need no issued envelope
BUILD_SCRIPT = B + "/📜️script.ts"
region(BUILD_SCRIPT, '''    if(command==="remote-execution"){''', '''    if(command==="remote-cache"){''', '''    if(command==="remote-execution"){
      const {runRemoteExecutionCommandV1}=await import("./🌐️remote-execution/⌨️command/🟦️.ts");
      const entry=join(import.meta.dir,"🌐️remote-execution","🏃️entry","🟦️.ts");
      const spawnWorker=(args:readonly string[],environment:Readonly<Record<string,string>>)=>{
        const child=Bun.spawn([process.execPath,entry,...args],{cwd:root,env:{...process.env,...environment},stdin:"ignore",stdout:"inherit",stderr:"inherit"});
        return {exited:child.exited,stop:async()=>{child.kill("SIGTERM");await Promise.race([child.exited,Bun.sleep(15000).then(()=>{child.kill("SIGKILL");})]);}};
      };
      const outcome=await runRemoteExecutionCommandV1(resolved.args.slice(1),{root,environment:process.env,signal:phaseControl.signal,progress:async stage=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(invocation,"build:"+stage,"running");},write:line=>console.log(line),spawnWorker});
      if(outcome.result!==undefined)console.log(JSON.stringify(outcome.result,null,2));
      if(outcome.exitCode!==0)process.exitCode=outcome.exitCode;
      return;
    }
''')

# ---- dashboard tools
ROOT_PROJECT = "📋️project.json"
region(ROOT_PROJECT, '''          {
            "id": "build-remote-execution-status",''', '''          {
            "id": "build-exec",''', '''          {
            "id": "build-remote-execution-status",
            "title": { "en": "Show the remote execution service", "de": "Remote-Execution-Dienst anzeigen" },
            "description": { "en": "Report the remote execution contract, whether the gRPC endpoint answers and which workers, queues and counters the scheduler reports; nothing is started.", "de": "Den Remote-Execution-Vertrag, die Erreichbarkeit des gRPC-Endpunkts und die Worker, Warteschlangen und Zähler des Schedulers melden; es wird nichts gestartet." },
            "verb": "inspect",
            "command": ["bun", "repo", "remote-execution", "status"]
          },
          {
            "id": "build-remote-execution-serve",
            "title": { "en": "Run the remote execution service", "de": "Remote-Execution-Dienst betreiben" },
            "description": { "en": "Run the in-house execution service with the shared store and a local worker on this machine until stopped so hermetic Bazel actions can run remotely; clients set the variables printed at start.", "de": "Den hauseigenen Ausführungsdienst mit dem gemeinsamen Speicher und einem lokalen Worker auf diesem Rechner bis zum Stopp betreiben, damit hermetische Bazel-Actions remote laufen können; Clients setzen die beim Start ausgegebenen Variablen." },
            "verb": "serve",
            "command": ["bun", "repo", "remote-execution", "serve", "--worker-port", "{port}"],
            "continuous": true,
            "ready": { "port": 8981, "portEnv": "SEMIO_REMOTE_EXECUTION_WORKER_PORT", "path": "/healthz" }
          },
''')

# ---- hermetic probe targets: keep the generic ones (platform, fanout, chain), drop the two that read the container descriptor
CACHING = L + "/⚡️caching/📋️project.json"
region(CACHING, '    "remote-execution-probe-descriptor": {', '    "remote-execution-probe-fanout": {', '')
sub(CACHING, '''        "remote-execution-probe-platform",
        "remote-execution-probe-descriptor",
        "remote-execution-probe-compose",
        "remote-execution-probe-fanout"''', '''        "remote-execution-probe-platform",
        "remote-execution-probe-fanout"''')
sub(CACHING, " .🧬semio/🦑️repo/⚡️cache/remote-execution/descriptor.json .🧬semio/🦑️repo/⚡️cache/remote-execution/compose.json", "")

# ---- cache ledger: the remote execution home holds the workers' blob cache and private roots, collected by its owner
LEDGER = L + "/⚡️caching/💾️storage/🧾️ledger"


def ledger(value):
    areas = [area for area in value["areas"] if area["id"] != "remote-execution"]
    areas.append({
        "id": "remote-execution",
        "root": "repository",
        "kind": "tree",
        "directory": "remote-execution",
        "budget": {"bytes": 536870912},
        "managed": {"owner": "remote-execution"},
        "writers": [X + "/👷️worker/🟦️.ts"],
        "purpose": {
            "en": "Blob cache and private input roots of the workers of repo remote-execution, bounded and least recently used first.",
            "de": "Blob-Cache und private Eingabeverzeichnisse der Worker von repo remote-execution, begrenzt und die am längsten ungenutzten zuerst.",
        },
    })
    value["areas"] = areas
    return value


js(LEDGER + "/🔣️policy.json", ledger)
sub(LEDGER + "/🧬️schema/🔣️.json", '"dashboard", "remote-cache"]', '"dashboard", "remote-cache", "remote-execution"]')
sub(LEDGER + "/🟦️.ts", '|"dashboard"|"remote-cache";', '|"dashboard"|"remote-cache"|"remote-execution";')
text(B + "/🧹️cache-gc/⌨️command/🟦️.ts", '''   if(owner!=="test-evidence")return Promise.resolve(undefined);''', '''   if(owner==="remote-execution")return import("../../🌐️remote-execution/👷️worker/🟦️.ts").then(module=>module.collectRemoteExecutionForLedgerV1(root,dryRun,invocation.control.signal)).catch(error=>"failed: "+String((error as Error).message));
   if(owner!=="test-evidence")return Promise.resolve(undefined);''')

# ---- README, English and German
README = "🧰️framework/🛍️products/🦑️repo/README.md"
sub(README, "| `remote-execution serve`, `status`, `env` | run, inspect or describe the optional local remote execution cluster (see Remote execution) | `repo remote-execution status` |",
    "| `remote-execution serve\\|status\\|workers` | run the in-house remote execution service, inspect it or join this machine to it as a worker (see Remote execution) | `repo remote-execution status` |")
sub(README, "| `remote-execution serve`, `status`, `env` | startet, prüft oder beschreibt den optionalen lokalen Remote-Execution-Cluster (siehe Remote-Ausführung) | `repo remote-execution status` |",
    "| `remote-execution serve\\|status\\|workers` | betreibt den hauseigenen Remote-Execution-Dienst, zeigt ihn an oder bindet diesen Rechner als Worker an (siehe Remote-Ausführung) | `repo remote-execution status` |")
sub(README, "| `SEMIO_REMOTE_EXECUTION_PLATFORM` | worker platform: `darwin`, `linux` or `windows` plus `-arm64` or `-x64` | `linux-x64` |\n",
    "| `SEMIO_REMOTE_EXECUTION_PLATFORM` | worker platform: `darwin`, `linux` or `windows` plus `-arm64` or `-x64` | `linux-x64` |\n| `SEMIO_REMOTE_EXECUTION_INSTANCE` | instance name (namespace) in the executor's store | `main` |\n| `SEMIO_REMOTE_EXECUTION_SCHEDULER` | worker API of the scheduler, for `remote-execution workers` and `status` | none |\n| `SEMIO_REMOTE_EXECUTION_WORKER_TOKEN` | bearer token of the worker API (required beyond loopback) | none |\n")
sub(README, "| `SEMIO_REMOTE_EXECUTION_PLATFORM` | Worker-Plattform: `darwin`, `linux` oder `windows` plus `-arm64` oder `-x64` | `linux-x64` |\n",
    "| `SEMIO_REMOTE_EXECUTION_PLATFORM` | Worker-Plattform: `darwin`, `linux` oder `windows` plus `-arm64` oder `-x64` | `linux-x64` |\n| `SEMIO_REMOTE_EXECUTION_INSTANCE` | Instanzname (Namensraum) im Speicher des Executors | `main` |\n| `SEMIO_REMOTE_EXECUTION_SCHEDULER` | Worker-API des Schedulers, für `remote-execution workers` und `status` | keiner |\n| `SEMIO_REMOTE_EXECUTION_WORKER_TOKEN` | Bearer-Token der Worker-API (außerhalb von Loopback nötig) | keiner |\n")
region(README, "- **Executor requirements.**", "\n", "- **Executor.** The in-house service below, or any service for Bazel's Remote Execution API v2 with content store, action cache and scheduler; the workers' properties must match the platform (in-house workers offer the `OSFamily` and `Arch` of their machine plus `--properties`; Buildbarn matches exactly; BuildBuddy uses `container-image` and `pool`); workers need only the uploaded Bun, no checkout. Bazel refuses an HTTP cache beside a gRPC executor, so while an executor is configured the Bazel implementation drops its HTTP `--remote_cache` flag, re-scopes the credential helper to the executor host and relies on the executor's own cache; `--remote_instance_name` follows `SEMIO_REMOTE_EXECUTION_INSTANCE` and compression is off.")
region(README, "- **Anforderungen an den Executor.**", "\n", "- **Executor.** Der hauseigene Dienst unten oder jeder Dienst für die Remote Execution API v2 von Bazel mit Inhalts-Store, Action-Cache und Scheduler; die Eigenschaften der Worker müssen zur Plattform passen (hauseigene Worker bieten `OSFamily` und `Arch` ihres Rechners plus `--properties`; Buildbarn vergleicht exakt; BuildBuddy nutzt `container-image` und `pool`); Worker brauchen nur das hochgeladene Bun, keinen Checkout. Bazel lehnt einen HTTP-Cache neben einem gRPC-Executor ab, daher lässt die Bazel-Implementierung ihr HTTP-`--remote_cache`-Flag weg, solange ein Executor konfiguriert ist, richtet den Credential-Helper auf den Executor-Host aus und nutzt den eigenen Cache des Executors; `--remote_instance_name` folgt `SEMIO_REMOTE_EXECUTION_INSTANCE`, Kompression ist aus.")
region(README, "- **Optional local cluster.**", "\n", "- **In-house service.** `repo remote-execution serve|status|workers` needs no containers and no external executor. `serve` starts the shared store of `repo remote-cache` (or uses `--cache <url>`), the scheduler with a gRPC listener for Bazel (default `127.0.0.1:8980`: capabilities, execution, action cache, content-addressable storage and ByteStream, the subset of the Remote Execution API v2 that Bazel 8.4.2 uses, implemented over Node's `http2` with a protobuf codec generated from Bazel's own descriptors) and an HTTP listener for workers and status (`8981`), and starts `--local-workers` worker processes (default 1). It prints the variables a client sets and the command another machine or devcontainer joins with, `repo remote-execution workers --join http://host:8981` (set `SEMIO_REMOTE_EXECUTION_WORKER_TOKEN` and `SEMIO_REMOTE_CACHE_TOKEN` when the service listens beyond loopback with `--bind`). Workers pull actions (long poll, heartbeats, leases; lost work is retried on another worker), build a private input root from the store, run the command with exactly the declared environment inside the sandbox of the hermetic table (`darwin-sandbox`, `linux-sandbox`, else `isolated-root`; `--sandbox` pins one) and put the outputs into the store; identical actions in flight run once and successful results enter the action cache. `repo remote-execution status` shows the contract, whether the endpoint answers and the registered workers. Why this protocol: Bazel offloads execution only over the Remote Execution API, so a private protocol would leave Bazel running every action itself; the `semio` backend can use the same scheduler and store through a native front end. Not enforced: network isolation on Windows (no sandbox) and on Linux without bubblewrap. It is a developer service, never a dependency: with `SEMIO_REMOTE_EXECUTION_ENDPOINT` unset every action stays local. Tests live under `🏗️build/🌐️remote-execution/*/🧪️tests`: schema and policy against Ajv, the wire codec against protobufjs and an independent Python decoder over the descriptors of Bazel's own classes, scenarios replayed by `curl` over HTTP/2, scheduler laws, sandbox confinement, a worker in its own process, and, with `SEMIO_TEST_BAZEL_BINARY`, real Bazel offloading a hermetic workspace.")
region(README, "- **Optionaler lokaler Cluster.**", "\n", "- **Hauseigener Dienst.** `repo remote-execution serve|status|workers` braucht weder Container noch einen externen Executor. `serve` startet den gemeinsamen Speicher von `repo remote-cache` (oder nutzt `--cache <url>`), den Scheduler mit einem gRPC-Listener für Bazel (Standard `127.0.0.1:8980`: Capabilities, Ausführung, Action-Cache, inhaltsadressierter Speicher und ByteStream, die Teilmenge der Remote Execution API v2, die Bazel 8.4.2 nutzt, umgesetzt über das `http2` von Node mit einem aus Bazels eigenen Deskriptoren erzeugten Protobuf-Codec) und einem HTTP-Listener für Worker und Status (`8981`) und startet `--local-workers` Worker-Prozesse (Standard 1). Er gibt die Variablen aus, die ein Client setzt, und den Befehl, mit dem ein anderer Rechner oder Devcontainer beitritt, `repo remote-execution workers --join http://host:8981` (`SEMIO_REMOTE_EXECUTION_WORKER_TOKEN` und `SEMIO_REMOTE_CACHE_TOKEN` setzen, wenn der Dienst mit `--bind` über Loopback hinaus lauscht). Worker holen Actions ab (Long Poll, Heartbeats, Leases; verlorene Arbeit wird auf einem anderen Worker wiederholt), bauen aus dem Speicher ein privates Eingabeverzeichnis, führen den Befehl mit genau der deklarierten Umgebung in der Sandbox der Hermetic-Tabelle aus (`darwin-sandbox`, `linux-sandbox`, sonst `isolated-root`; `--sandbox` legt eine fest) und legen die Ausgaben im Speicher ab; gleiche Actions im Flug laufen einmal, erfolgreiche Ergebnisse gelangen in den Action-Cache. `repo remote-execution status` zeigt den Vertrag, ob der Endpunkt antwortet und die angemeldeten Worker. Warum dieses Protokoll: Bazel lagert die Ausführung nur über die Remote Execution API aus, ein privates Protokoll ließe Bazel jede Action selbst ausführen; das `semio`-Backend kann denselben Scheduler und Speicher über ein natives Frontend nutzen. Nicht durchgesetzt: Netzwerkisolation unter Windows (keine Sandbox) und unter Linux ohne bubblewrap. Es ist ein Entwicklerdienst, nie eine Abhängigkeit: Ist `SEMIO_REMOTE_EXECUTION_ENDPOINT` ungesetzt, bleibt jede Action lokal. Die Tests liegen unter `🏗️build/🌐️remote-execution/*/🧪️tests`: Schema und Policy gegen Ajv, der Wire-Codec gegen protobufjs und einen unabhängigen Python-Decoder über den Deskriptoren aus Bazels eigenen Klassen, per `curl` über HTTP/2 abgespielte Szenarien, Scheduler-Gesetze, Sandbox-Einsperrung, ein Worker im eigenen Prozess und, mit `SEMIO_TEST_BAZEL_BINARY`, echtes Bazel, das einen hermetischen Workspace auslagert.")
