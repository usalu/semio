# anchored edits of existing live files. Every edit replaces exactly one region (text/region) or states a counted substitution (sub) or transforms parsed JSON (json).
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
R = B + "/🌐️remote-cache"
import os as _os
EDITS = []


def text(file, old, new):
    EDITS.append({"file": file, "kind": "text", "old": old, "new": new})


def region(file, start, end, new):
    EDITS.append({"file": file, "kind": "region", "start": start, "end": end, "new": new})


def sub(file, old, new, count=1):
    EDITS.append({"file": file, "kind": "sub", "old": old, "new": new, "count": count})


def js(file, fn):
    EDITS.append({"file": file, "kind": "json", "fn": fn})


# ---- build policy: the server section (ledger directory, read-only, TLS, action result validation, maintenance, environment names)
def policy(value):
    server = value["remoteCache"]["server"]
    value["remoteCache"]["server"] = {
        "host": server["host"],
        "port": server["port"],
        "directory": ".🧬semio/🦑️repo/⚡️cache/remote-cache",
        "maximumBytes": 2684354560,
        "maximumEntryBytes": server["maximumEntryBytes"],
        "maximumAgeDays": server["maximumAgeDays"],
        "readOnly": False,
        "tls": None,
        "actionResults": "complete",
        "maximumActionResultBytes": 4194304,
        "maintenanceSeconds": 300,
        "environment": {
            "readOnly": "SEMIO_REMOTE_CACHE_SERVER_READ_ONLY",
            "tlsCertificate": "SEMIO_REMOTE_CACHE_SERVER_TLS_CERTIFICATE",
            "tlsKey": "SEMIO_REMOTE_CACHE_SERVER_TLS_KEY",
        },
        "tokenEnvironment": server["tokenEnvironment"],
    }
    return value


js(B + "/🔣️policy.json", policy)

# ---- contract schema: ServerPolicy and Status
SCHEMA = B + "/🏃️execution/🧬️schema/🌐️remote-cache.json"
region(SCHEMA, '''    "ServerPolicy": {''', '''    "Policy": {''', '''    "ServerPolicy": {
      "description": "The bounds, switches and tokens of the repository's own cache server (bun repo remote-cache serve). The store is finite: it evicts least recently used entries beyond maximumBytes and entries idle for more than maximumAgeDays. actionResults says how far uploaded Bazel action results are checked: off, structure (a well-formed message) or complete (additionally every blob they name is stored, and a result is answered only while that holds).",
      "type": "object",
      "additionalProperties": false,
      "required": ["host", "port", "directory", "maximumBytes", "maximumEntryBytes", "maximumAgeDays", "readOnly", "tls", "actionResults", "maximumActionResultBytes", "maintenanceSeconds", "environment", "tokenEnvironment"],
      "properties": {
        "host": { "type": "string", "minLength": 1, "maxLength": 253 },
        "port": { "type": "integer", "minimum": 0, "maximum": 65535 },
        "directory": { "description": "Relative to the workspace root; the default is the home the cache ledger declares for the remote cache.", "type": "string", "pattern": "^[^\\\\u0000/\\\\\\\\][^\\\\u0000]*$" },
        "maximumBytes": { "type": "integer", "minimum": 1048576 },
        "maximumEntryBytes": { "type": "integer", "minimum": 1024 },
        "maximumAgeDays": { "type": "integer", "minimum": 0, "maximum": 3650 },
        "readOnly": { "description": "true refuses every write with 403, whatever token is presented.", "type": "boolean" },
        "tls": {
          "description": "null serves plain HTTP; otherwise the certificate chain and private key files (PEM) of an HTTPS listener.",
          "oneOf": [
            { "const": null },
            {
              "type": "object",
              "additionalProperties": false,
              "required": ["certificate", "key"],
              "properties": {
                "certificate": { "type": "string", "minLength": 1 },
                "key": { "type": "string", "minLength": 1 }
              }
            }
          ]
        },
        "actionResults": { "enum": ["off", "structure", "complete"] },
        "maximumActionResultBytes": { "type": "integer", "minimum": 1024, "maximum": 67108864 },
        "maintenanceSeconds": { "description": "How often a running server re-reads its directory, so entries collected by another process stop counting; 0 switches it off.", "type": "integer", "minimum": 0, "maximum": 86400 },
        "environment": {
          "type": "object",
          "additionalProperties": false,
          "required": ["readOnly", "tlsCertificate", "tlsKey"],
          "properties": {
            "readOnly": { "$ref": "#/definitions/EnvironmentName" },
            "tlsCertificate": { "$ref": "#/definitions/EnvironmentName" },
            "tlsKey": { "$ref": "#/definitions/EnvironmentName" }
          }
        },
        "tokenEnvironment": {
          "type": "object",
          "additionalProperties": false,
          "required": ["read", "write"],
          "properties": {
            "read": { "$ref": "#/definitions/EnvironmentName" },
            "write": { "$ref": "#/definitions/EnvironmentName" }
          }
        }
      }
    },
''')
region(SCHEMA, '''    "Status": {''', None, '''    "Status": {
      "description": "What bun repo remote-cache status prints: the client contract, how both build tools render it, one probe per protocol, what the running server reports about itself and what the local store directory holds.",
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "configured", "contract", "effectiveUpload", "token", "renderings", "probes", "server", "store"],
      "properties": {
        "version": { "const": 1 },
        "configured": { "type": "boolean" },
        "contract": { "oneOf": [{ "$ref": "#/definitions/Contract" }, { "const": null }] },
        "effectiveUpload": { "type": "boolean" },
        "token": {
          "oneOf": [
            { "const": null },
            {
              "type": "object",
              "additionalProperties": false,
              "required": ["environment", "present"],
              "properties": {
                "environment": { "$ref": "#/definitions/EnvironmentName" },
                "present": { "type": "boolean" }
              }
            }
          ]
        },
        "renderings": {
          "oneOf": [
            { "const": null },
            {
              "type": "object",
              "additionalProperties": false,
              "required": ["nx", "bazel"],
              "properties": {
                "nx": { "$ref": "#/definitions/Rendering" },
                "bazel": { "$ref": "#/definitions/Rendering" }
              }
            }
          ]
        },
        "probes": { "oneOf": [{ "const": null }, { "type": "array", "items": { "$ref": "#/definitions/Probe" } }] },
        "server": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/ServerStatus" }] },
        "store": {
          "oneOf": [
            { "const": null },
            {
              "type": "object",
              "additionalProperties": false,
              "required": ["directory", "entries", "bytes", "maximumBytes"],
              "properties": {
                "directory": { "type": "string", "minLength": 1 },
                "entries": { "type": "integer", "minimum": 0 },
                "bytes": { "type": "integer", "minimum": 0 },
                "maximumBytes": { "type": "integer", "minimum": 1048576 }
              }
            }
          ]
        }
      }
    },
''' + open(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), 'staged', 'schema-defs.txt'), encoding='utf8').read() + '''''')

# ---- operations catalogue: remote-cache is its own operation; cache keeps the ledger and the task cache
CATALOG = B + "/⚙️operations/🔣️catalog.json"
region(CATALOG, '''    {
      "name": "--port",''', '''    {
      "name": "--platform",''', '''    {
      "name": "--port",
      "value": "separate",
      "valueName": "port",
      "operations": ["remote-cache", "remote-execution"],
      "description": { "en": "remote-cache serve: port to listen on; 0 picks a free port (default from the build policy). remote-execution: loopback port of the cluster frontend (default 8980).", "de": "remote-cache serve: Port, auf dem gelauscht wird; 0 wählt einen freien Port (Standard aus der Build-Policy). remote-execution: Loopback-Port des Cluster-Frontends (Standard 8980)." }
    },
    {
      "name": "--host",
      "value": "separate",
      "valueName": "address",
      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: address to listen on; beyond loopback a token is required.", "de": "remote-cache serve: Adresse, auf der gelauscht wird; außerhalb von Loopback ist ein Token nötig." }
    },
    {
      "name": "--directory",
      "value": "separate",
      "valueName": "path",
      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve|status|gc: store directory, relative to the workspace root (default: the remote cache home of the cache ledger).", "de": "remote-cache serve|status|gc: Speicherverzeichnis, relativ zum Workspace-Wurzelverzeichnis (Standard: die Remote-Cache-Heimat der Cache-Bilanz)." }
    },
    {
      "name": "--max-bytes",
      "value": "separate",
      "valueName": "bytes",
      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve|gc: upper bound of the store; the least recently used entries are evicted beyond it.", "de": "remote-cache serve|gc: Obergrenze des Speichers; darüber werden die am längsten ungenutzten Einträge entfernt." }
    },
    {
      "name": "--read-only",
      "value": "none",
      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: refuse every write with 403, whatever token is presented.", "de": "remote-cache serve: lehnt jedes Schreiben mit 403 ab, welches Token auch vorgelegt wird." }
    },
    {
      "name": "--tls-certificate",
      "value": "separate",
      "valueName": "path",
      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: PEM certificate chain; together with --tls-key the server speaks HTTPS.", "de": "remote-cache serve: PEM-Zertifikatskette; zusammen mit --tls-key spricht der Server HTTPS." }
    },
    {
      "name": "--tls-key",
      "value": "separate",
      "valueName": "path",
      "operations": ["remote-cache"],
      "description": { "en": "remote-cache serve: PEM private key of the certificate.", "de": "remote-cache serve: PEM-Privatschlüssel des Zertifikats." }
    },
''')
text(CATALOG, '''      "operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache"],
      "description": { "en": "Print lists as JSON instead of one entry per line."''', '''      "operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache", "remote-cache"],
      "description": { "en": "Print lists as JSON instead of one entry per line."''')
text(CATALOG, '''      "operations": ["run", "run-many", "affected", "exec", "cache"],
      "description": {
        "en": "Print the task graph as a DryRunPlan on stdout and execute nothing; the same for every tool. With cache gc: list what would be removed and remove nothing.",
        "de": "Gibt den Task-Graphen als DryRunPlan auf stdout aus und führt nichts aus; für jedes Werkzeug gleich. Mit cache gc: listet auf, was entfernt würde, und entfernt nichts."''', '''      "operations": ["run", "run-many", "affected", "exec", "cache", "remote-cache"],
      "description": {
        "en": "Print the task graph as a DryRunPlan on stdout and execute nothing; the same for every tool. With cache gc or remote-cache gc: list what would be removed and remove nothing.",
        "de": "Gibt den Task-Graphen als DryRunPlan auf stdout aus und führt nichts aus; für jedes Werkzeug gleich. Mit cache gc oder remote-cache gc: listet auf, was entfernt würde, und entfernt nichts."''')
text(CATALOG, '''"synopsis": "cache status|gc|clear|remote status|serve",
      "summary": {
        "en": "Show and collect every declared cache home with its budget, clear the selected tool's local task cache, show the remote cache contract, or serve a remote cache.",
        "de": "Zeigt und sammelt jede deklarierte Cache-Heimat mit ihrem Budget, leert den lokalen Task-Cache des gewählten Werkzeugs, zeigt den Remote-Cache-Vertrag oder stellt einen Remote-Cache bereit."''', '''"synopsis": "cache status|gc|clear",
      "summary": {
        "en": "Show and collect every declared cache home with its budget, or clear the selected tool's local task cache.",
        "de": "Zeigt und sammelt jede deklarierte Cache-Heimat mit ihrem Budget oder leert den lokalen Task-Cache des gewählten Werkzeugs."''')
region(CATALOG, ''',
        {
          "name": "remote",
          "synopsis": "cache remote status",''', '''
      ],
      "flags": ["--port", "--host", "--directory", "--max-bytes", "--json", "--dry-run", "--area", "--tasks"],''', "")
text(CATALOG, '''"flags": ["--port", "--host", "--directory", "--max-bytes", "--json", "--dry-run", "--area", "--tasks"],''', '''"flags": ["--json", "--dry-run", "--area", "--tasks"],''')
region(CATALOG, ''',
        { "command": "repo cache serve"''', '''
      ],
      "notes": [''', "")
region(CATALOG, ''',
        {"en": "The remote cache is off until''', '''
      ]
    },
    {
      "name": "capacity",''', "")
text(CATALOG, '''    {
      "name": "capacity",
      "requires": "none",''', '''    {
      "name": "remote-cache",
      "requires": "none",
      "synopsis": "remote-cache serve|status|gc",
      "summary": {
        "en": "Serve a bounded, shared result cache for Bazel and Nx from this machine, inspect the remote cache contract and the server, or collect the cache store.",
        "de": "Stellt von diesem Rechner aus einen begrenzten, gemeinsamen Ergebnis-Cache für Bazel und Nx bereit, zeigt den Remote-Cache-Vertrag und den Server oder sammelt den Cache-Speicher ein."
      },
      "implementations": { "nx": "neutral", "bazel": "neutral" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        {
          "name": "serve",
          "synopsis": "remote-cache serve [--port <n>] [--host <address>] [--directory <path>] [--max-bytes <n>] [--read-only] [--tls-certificate <path> --tls-key <path>]", "flags": ["--port", "--host", "--directory", "--max-bytes", "--read-only", "--tls-certificate", "--tls-key"],
          "summary": {
            "en": "Serve the store over HTTP (or HTTPS) until stopped: Bazel's HTTP cache protocol and the Nx self-hosted cache API share one content-addressed, bounded store.",
            "de": "Stellt den Speicher über HTTP (oder HTTPS) bereit, bis er gestoppt wird: Bazels HTTP-Cache-Protokoll und die selbst gehostete Cache-API von Nx teilen sich einen inhaltsadressierten, begrenzten Speicher."
          }
        },
        {
          "name": "status",
          "synopsis": "remote-cache status [--directory <path>]", "flags": ["--directory"],
          "summary": {
            "en": "Print a RemoteCacheStatus: the remote cache contract, how Nx and Bazel render it, one read-only probe per protocol, what the running server reports and what the local store holds.",
            "de": "Gibt einen RemoteCacheStatus aus: den Remote-Cache-Vertrag, wie Nx und Bazel ihn umsetzen, eine lesende Probe je Protokoll, was der laufende Server meldet und was der lokale Speicher enthält."
          }
        },
        {
          "name": "gc",
          "synopsis": "remote-cache gc [--dry-run] [--json] [--directory <path>] [--max-bytes <n>]", "flags": ["--directory", "--max-bytes", "--dry-run", "--json"],
          "summary": {
            "en": "Collect the store from outside, also while a server runs: remove unusable action results, entries idle for too long and the least recently used entries beyond the bound; --dry-run only plans.",
            "de": "Sammelt den Speicher von außen ein, auch während ein Server läuft: entfernt unbrauchbare Action-Ergebnisse, zu lange ungenutzte Einträge und die am längsten ungenutzten Einträge über der Grenze; --dry-run plant nur."
          }
        }
      ],
      "flags": ["--port", "--host", "--directory", "--max-bytes", "--read-only", "--tls-certificate", "--tls-key", "--json", "--dry-run"],
      "examples": [
        { "command": "repo remote-cache serve", "description": { "en": "Serve a local remote cache on 127.0.0.1:6090.", "de": "Stellt einen lokalen Remote-Cache auf 127.0.0.1:6090 bereit." } },
        {
          "command": "repo remote-cache serve --host 0.0.0.0 --tls-certificate cert.pem --tls-key key.pem",
          "description": { "en": "Share the cache with other machines over HTTPS; a token in the server environment is required beyond loopback.", "de": "Teilt den Cache über HTTPS mit anderen Rechnern; außerhalb von Loopback ist ein Token in der Server-Umgebung nötig." }
        },
        {
          "command": "repo remote-cache status",
          "description": { "en": "Show how Nx and Bazel use the remote cache (once an endpoint is configured) and whether the server answers.", "de": "Zeigt, wie Nx und Bazel den Remote-Cache nutzen (sobald ein Endpunkt konfiguriert ist) und ob der Server antwortet." }
        },
        { "command": "repo remote-cache gc --dry-run", "description": { "en": "List what the collector would remove from the store.", "de": "Listet auf, was der Einsammler aus dem Speicher entfernen würde." } }
      ],
      "notes": [
        {
          "en": "The remote cache is off for clients until SEMIO_REMOTE_CACHE_ENDPOINT names an http or https origin. SEMIO_REMOTE_CACHE_INSTANCE, _ACCESS (read-only|read-write), _UPLOAD, _COMPRESSION, _DOWNLOAD (toplevel|all), _TIMEOUT_SECONDS and _RETRIES override the defaults of the build policy; the access token is read from the variable named by SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT (default SEMIO_REMOTE_CACHE_TOKEN) and is never written to disk.",
          "de": "Der Remote-Cache ist für Clients aus, bis SEMIO_REMOTE_CACHE_ENDPOINT einen http- oder https-Ursprung nennt. SEMIO_REMOTE_CACHE_INSTANCE, _ACCESS (read-only|read-write), _UPLOAD, _COMPRESSION, _DOWNLOAD (toplevel|all), _TIMEOUT_SECONDS und _RETRIES überschreiben die Standardwerte der Build-Policy; das Zugriffstoken wird aus der Variable gelesen, die SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT nennt (Standard SEMIO_REMOTE_CACHE_TOKEN), und nie auf die Platte geschrieben."
        },
        {
          "en": "Bazel renders the contract into its remote cache settings (endpoint, instance name, upload, compression, download mode, timeout, retries) and passes the token through a credential helper. Nx renders it into its self-hosted remote cache variables (server and access token); it cannot honour compression, download mode, timeouts or retries and leaves read-only to the server. status prints both renderings.",
          "de": "Bazel setzt den Vertrag in seine Remote-Cache-Einstellungen um (Endpunkt, Instanzname, Upload, Kompression, Download-Modus, Zeitlimit, Wiederholungen) und reicht das Token über einen Credential-Helper durch. Nx setzt ihn in seine Variablen für den selbst gehosteten Remote-Cache (Server und Zugriffstoken) um; es kann Kompression, Download-Modus, Zeitlimits und Wiederholungen nicht einhalten und überlässt read-only dem Server. status zeigt beide Umsetzungen."
        },
        {
          "en": "serve speaks Bazel's HTTP cache protocol (GET, HEAD and PUT on /ac/<sha256> and /cas/<sha256>) and the Nx self-hosted cache API (GET and PUT on /v1/cache/<hash>, 409 when an entry exists), each under an optional /<instance> prefix. A blob is refused with 400 unless its content has the digest in its address; an action result is refused with 400 unless it is a well-formed ActionResult whose output blobs are already stored (the policy can relax this to structure or off), and a result whose blobs left the store is answered as a miss. Bearer tokens come from SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN and SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN; without a token it refuses to listen beyond loopback; --read-only (or SEMIO_REMOTE_CACHE_SERVER_READ_ONLY=true) refuses every write; --tls-certificate and --tls-key (or SEMIO_REMOTE_CACHE_SERVER_TLS_CERTIFICATE and SEMIO_REMOTE_CACHE_SERVER_TLS_KEY) switch to HTTPS. One server owns a directory at a time.",
          "de": "serve spricht Bazels HTTP-Cache-Protokoll (GET, HEAD und PUT auf /ac/<sha256> und /cas/<sha256>) und die selbst gehostete Cache-API von Nx (GET und PUT auf /v1/cache/<hash>, 409 bei vorhandenem Eintrag), jeweils mit optionalem Präfix /<instance>. Ein Blob wird mit 400 abgelehnt, wenn sein Inhalt nicht den Digest seiner Adresse hat; ein Action-Ergebnis wird mit 400 abgelehnt, wenn es kein wohlgeformtes ActionResult mit bereits gespeicherten Ausgabe-Blobs ist (die Policy kann das auf structure oder off lockern), und ein Ergebnis, dessen Blobs den Speicher verlassen haben, wird als Fehltreffer beantwortet. Bearer-Tokens stammen aus SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN und SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN; ohne Token lauscht er nur auf Loopback; --read-only (oder SEMIO_REMOTE_CACHE_SERVER_READ_ONLY=true) lehnt jedes Schreiben ab; --tls-certificate und --tls-key (oder SEMIO_REMOTE_CACHE_SERVER_TLS_CERTIFICATE und SEMIO_REMOTE_CACHE_SERVER_TLS_KEY) schalten auf HTTPS. Ein Server besitzt ein Verzeichnis."
        },
        {
          "en": "The store is bounded and evicts the least recently used entries and entries idle for too long. It lives in the remote cache home of the cache ledger, so cache status reports it and cache gc collects it with the same bounds; gc here does the same from the command line, takes the collector lock, never blocks a running server and stops at the next checkpoint when the caller cancels. serve and gc publish progress while they scan the store; the output follows SEMIO_LANGUAGE or --lang and is printed in both languages when neither is set.",
          "de": "Der Speicher ist begrenzt und entfernt die am längsten ungenutzten und die zu lange ungenutzten Einträge. Er liegt in der Remote-Cache-Heimat der Cache-Bilanz, sodass cache status ihn meldet und cache gc ihn mit denselben Grenzen einsammelt; gc hier tut dasselbe von der Befehlszeile aus, nimmt die Einsammler-Sperre, blockiert nie einen laufenden Server und hält am nächsten Kontrollpunkt an, wenn der Aufrufer abbricht. serve und gc melden den Fortschritt beim Einlesen des Speichers; die Ausgabe folgt SEMIO_LANGUAGE oder --lang und erscheint in beiden Sprachen, wenn keins von beiden gesetzt ist."
        }
      ]
    },
    {
      "name": "capacity",
      "requires": "none",''')
text(CATALOG[:-len("/⚙️operations/🔣️catalog.json")] + "/⚙️operations/🧬️schema/🔣️.json", '"cache", "capacity", "leases", "remote-execution"]', '"cache", "capacity", "leases", "remote-execution", "remote-cache"]')

# ---- root command: the only router branch
SCRIPT = B + "/📜️script.ts"
region(SCRIPT, '''    if(command==="cache"&&["serve","remote"].includes(resolved.args[1]??"")){''', '''    const {withRepositoryCacheStoreV1}=await import(''', '''    if(command==="remote-cache"){
      const {runRemoteCacheCommandV1}=await import("./🌐️remote-cache/⌨️command/🟦️.ts");
      const tools=async()=>[(await import("../⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts")).nxRemoteCacheToolV1(),(await import("./🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts")).bazelRemoteCacheToolV1()];
      const outcome=await runRemoteCacheCommandV1(resolved.args.slice(1),{root,environment:process.env,signal:phaseControl.signal,progress:async stage=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(invocation,"build:"+stage,"running");},write:line=>console.log(line),tools});
      if(outcome.result!==undefined)console.log(JSON.stringify(outcome.result,null,2));
      if(outcome.exitCode!==0)process.exitCode=outcome.exitCode;
      return;
    }
''')

# ---- dashboard tools
PROJECT = "📋️project.json"
region(PROJECT, '''          {
            "id": "build-cache-remote-status",''', '''          {
            "id": "build-remote-execution-status",''', '''          {
            "id": "build-remote-cache-status",
            "title": { "en": "Show the remote build cache", "de": "Remote-Build-Cache anzeigen" },
            "description": { "en": "Report the remote cache contract, how Nx and Bazel use it, whether the server answers and what the local store holds; nothing is stored.", "de": "Den Remote-Cache-Vertrag, die Umsetzung in Nx und Bazel, die Erreichbarkeit des Servers und den Inhalt des lokalen Speichers melden; es wird nichts gespeichert." },
            "verb": "inspect",
            "command": ["bun", "repo", "remote-cache", "status"]
          },
          {
            "id": "build-remote-cache-serve",
            "title": { "en": "Serve the local remote build cache", "de": "Lokalen Remote-Build-Cache bereitstellen" },
            "description": { "en": "Serve a bounded remote cache for Bazel and Nx on this machine until stopped; clients point SEMIO_REMOTE_CACHE_ENDPOINT at it.", "de": "Einen begrenzten Remote-Cache für Bazel und Nx auf diesem Rechner bereitstellen, bis er gestoppt wird; Clients richten SEMIO_REMOTE_CACHE_ENDPOINT darauf." },
            "verb": "serve",
            "command": ["bun", "repo", "remote-cache", "serve", "--port", "{port}"],
            "continuous": true,
            "ready": { "port": 6090, "portEnv": "SEMIO_REMOTE_CACHE_PORT", "path": "/healthz" }
          },
          {
            "id": "build-remote-cache-gc",
            "title": { "en": "Plan the collection of the remote build cache", "de": "Einsammeln des Remote-Build-Caches planen" },
            "description": { "en": "List which unusable, idle and least recently used entries the remote cache store would lose; nothing is removed.", "de": "Auflisten, welche unbrauchbaren, ungenutzten und am längsten ungenutzten Einträge der Remote-Cache-Speicher verlieren würde; es wird nichts entfernt." },
            "verb": "inspect",
            "command": ["bun", "repo", "remote-cache", "gc", "--dry-run"]
          },
''')

# ---- cache ledger: the remote cache is a declared home with a budget, collected by its owner
LEDGER = L + "/⚡️caching/💾️storage/🧾️ledger"


def ledger(value):
    areas = [area for area in value["areas"] if area["id"] != "remote-cache"]
    areas.append({
        "id": "remote-cache",
        "root": "repository",
        "kind": "tree",
        "directory": "remote-cache",
        "budget": {"bytes": 3221225472},
        "managed": {"owner": "remote-cache"},
        "writers": [B + "/🌐️remote-cache/💾️store/🟦️.ts"],
        "purpose": {
            "en": "Shared Bazel and Nx result cache served by repo remote-cache, bounded and least recently used first.",
            "de": "Gemeinsamer Bazel- und Nx-Ergebniscache von repo remote-cache, begrenzt und die am längsten ungenutzten zuerst.",
        },
    })
    value["areas"] = areas
    return value


js(LEDGER + "/🔣️policy.json", ledger)
sub(LEDGER + "/🧬️schema/🔣️.json", '"enum": ["image", "control", "cargo", "nx", "bazel", "test-evidence", "dashboard"]', '"enum": ["image", "control", "cargo", "nx", "bazel", "test-evidence", "dashboard", "remote-cache"]')
sub(LEDGER + "/🟦️.ts", 'export type CacheOwnerV1="image"|"control"|"cargo"|"nx"|"bazel"|"test-evidence"|"dashboard";', 'export type CacheOwnerV1="image"|"control"|"cargo"|"nx"|"bazel"|"test-evidence"|"dashboard"|"remote-cache";')
text(B + "/🧹️cache-gc/⌨️command/🟦️.ts", '''  delegate:(owner,_id,dryRun)=>{
   if(owner!=="test-evidence")return Promise.resolve(undefined);''', '''  delegate:(owner,_id,dryRun)=>{
   if(owner==="remote-cache")return import("../../🌐️remote-cache/⌨️command/🟦️.ts").then(module=>module.collectRemoteCacheForLedgerV1(root,dryRun,invocation.control.signal)).catch(error=>"failed: "+String((error as Error).message));
   if(owner!=="test-evidence")return Promise.resolve(undefined);''')

# ---- tool renderings keep their docstrings truthful
sub(B + "/🔧️tools/🏗️bazel/🌐️remote-cache/🟦️.ts", "Bazel's side of `cache remote status`", "Bazel's side of `remote-cache status`")
sub(L + "/⚡️caching/🚀️bootstrap/🛠️tools/🌐️remote-cache/🟦️.ts", "Nx's side of `cache remote status`", "Nx's side of `remote-cache status`")

# ---- README, English and German
README = "🧰️framework/🛍️products/🦑️repo/README.md"
sub(README, "repo cache remote status                             # the remote cache contract, its rendering for the tool, whether the endpoint answers\nrepo cache serve [--port <n>]                        # serve a bounded remote cache for Bazel and Nx until stopped",
    "repo remote-cache status                             # the remote cache contract, both renderings, probes, the server and the local store\nrepo remote-cache serve [--port <n>]                 # serve a bounded remote cache for Bazel and Nx until stopped\nrepo remote-cache gc [--dry-run]                     # collect the store: unusable results, idle entries, least recently used beyond the bound")
sub(README, "repo cache remote status                             # der Remote-Cache-Vertrag, seine Umsetzung für das Werkzeug, ob der Endpunkt antwortet\nrepo cache serve [--port <n>]                        # einen begrenzten Remote-Cache für Bazel und Nx bereitstellen, bis er gestoppt wird",
    "repo remote-cache status                             # der Remote-Cache-Vertrag, beide Umsetzungen, Proben, der Server und der lokale Speicher\nrepo remote-cache serve [--port <n>]                 # einen begrenzten Remote-Cache für Bazel und Nx bereitstellen, bis er gestoppt wird\nrepo remote-cache gc [--dry-run]                     # den Speicher einsammeln: unbrauchbare Ergebnisse, ungenutzte Einträge, am längsten ungenutzte über der Grenze")
sub(README, "| `cache remote status`, `cache serve` | the remote cache contract and a local cache server (see Caching model) | `repo cache remote status --tool=bazel` |", "| `remote-cache serve\\|status\\|gc` | the remote cache contract, a shared cache server and its collector (see Caching model) | `repo remote-cache status` |")
sub(README, "| `cache remote status`, `cache serve` | der Remote-Cache-Vertrag und ein lokaler Cache-Server (siehe Cache-Modell) | `repo cache remote status --tool=bazel` |", "| `remote-cache serve\\|status\\|gc` | der Remote-Cache-Vertrag, ein gemeinsamer Cache-Server und sein Einsammler (siehe Cache-Modell) | `repo remote-cache status` |")
sub(README, "or `repo cache serve` (store in `.cache/remote-cache`)", "or `repo remote-cache serve` (store in the ledger home `remote-cache`)")
sub(README, "oder `repo cache serve` (Speicher in `.cache/remote-cache`)", "oder `repo remote-cache serve` (Speicher in der Bilanz-Heimat `remote-cache`)")
sub(README, "repo cache serve --port 6090\n", "repo remote-cache serve --port 6090\n", 2)
sub(README, "repo cache remote status --tool=bazel\n", "repo remote-cache status\n", 2)
region(README, "- **`cache remote status`.** Prints the resolved contract", "\n", "- **`remote-cache status`.** Prints the resolved contract (`configured: false` while no endpoint is set), the rendering both tools get with their `unsupported` fields, whether the token variable is set (never its value), a read-only probe of the endpoint per protocol (`reachable`, `authorized`, the HTTP status), what the running server reports on `/status` and what the local store directory holds. It stores nothing.")
region(README, "- **`cache serve`.** Serves Bazel's HTTP cache protocol", "\n", "- **`remote-cache serve`.** Serves Bazel's HTTP cache protocol (`/ac/<sha256>`, `/cas/<sha256>`, blobs verified against their digest, action results validated) and the Nx self-hosted cache API (`/v1/cache/<hash>`) from one bounded store until you stop it. Options: `--port` (default 6090, `0` picks a free one), `--host` (default `127.0.0.1`), `--directory`, `--max-bytes` (default 2.5 GiB, at least 1 MiB; the least recently used entries go first), `--read-only` (every write answers 403), `--tls-certificate` with `--tls-key` (HTTPS). Tokens come from `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` and `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; without a token the server refuses to listen beyond loopback. The dashboard tool `build-remote-cache-serve` starts it.\n- **`remote-cache gc`.** Collects the store from outside, also while a server runs: it removes unusable action results, entries idle for more than 30 days and the least recently used entries beyond the bound, under a lock, with progress; `--dry-run` only plans and `--json` prints the report. `repo cache gc` delegates the ledger home `remote-cache` to the same collector.")
region(README, "- **`cache remote status`.** Gibt den aufgelösten Vertrag aus", "\n", "- **`remote-cache status`.** Gibt den aufgelösten Vertrag aus (`configured: false`, solange kein Endpunkt gesetzt ist), die Umsetzung beider Werkzeuge mit ihren `unsupported`-Feldern, ob die Token-Variable gesetzt ist (nie ihren Wert), eine lesende Sondierung des Endpunkts je Protokoll (`reachable`, `authorized`, den HTTP-Status), was der laufende Server auf `/status` meldet und was das lokale Speicherverzeichnis enthält. Es speichert nichts.")
region(README, "- **`cache serve`.** Stellt das HTTP-Cache-Protokoll von Bazel", "\n", "- **`remote-cache serve`.** Stellt das HTTP-Cache-Protokoll von Bazel (`/ac/<sha256>`, `/cas/<sha256>`, Blobs gegen ihren Digest geprüft, Action-Ergebnisse validiert) und die selbst gehostete Cache-API von Nx (`/v1/cache/<hash>`) aus einem begrenzten Speicher bereit, bis du ihn stoppst. Optionen: `--port` (Standard 6090, `0` wählt einen freien), `--host` (Standard `127.0.0.1`), `--directory`, `--max-bytes` (Standard 2,5 GiB, mindestens 1 MiB; die am längsten ungenutzten Einträge gehen zuerst), `--read-only` (jedes Schreiben antwortet 403), `--tls-certificate` mit `--tls-key` (HTTPS). Die Tokens kommen aus `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` und `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; ohne Token weigert sich der Server, außerhalb von Loopback zu lauschen. Das Dashboard-Werkzeug `build-remote-cache-serve` startet ihn.\n- **`remote-cache gc`.** Sammelt den Speicher von außen ein, auch während ein Server läuft: entfernt unbrauchbare Action-Ergebnisse, länger als 30 Tage ungenutzte Einträge und die am längsten ungenutzten Einträge über der Grenze, unter einer Sperre und mit Fortschritt; `--dry-run` plant nur, `--json` gibt den Bericht aus. `repo cache gc` übergibt die Bilanz-Heimat `remote-cache` an denselben Einsammler.")
sub(README, "`repo cache remote status` shows `configured: false` when no endpoint is set", "`repo remote-cache status` shows `configured: false` when no endpoint is set")
sub(README, "`repo cache remote status` zeigt `configured: false`, wenn kein Endpunkt gesetzt ist", "`repo remote-cache status` zeigt `configured: false`, wenn kein Endpunkt gesetzt ist")
sub(README, "| `cache status\\|clear\\|remote status\\|serve` | adapter | adapter | no |", "| `cache status\\|gc\\|clear` | adapter | adapter | no |")
sub(README, "`remote status` and `serve` belong to the remote cache (see Remote cache). |", "The remote cache has its own operation `remote-cache` (see Remote cache). |")
sub(README, "| `cache status\\|clear\\|remote status\\|serve` | adapter | adapter | nein |", "| `cache status\\|gc\\|clear` | adapter | adapter | nein |")
sub(README, "`remote status` und `serve` gehören zum Remote-Cache (siehe Remote-Cache). |", "Der Remote-Cache hat die eigene Operation `remote-cache` (siehe Remote-Cache). |")
sub(README, "size bound of the remote cache server (`--name value` or `--name=value`). | `cache serve` |", "size bound of the remote cache server (`--name value` or `--name=value`); `--read-only`, `--tls-certificate` and `--tls-key` switch it to read-only or HTTPS. | `remote-cache` |")
sub(README, "Größengrenze des Remote-Cache-Servers (`--name value` oder `--name=value`). | `cache serve` |", "Größengrenze des Remote-Cache-Servers (`--name value` oder `--name=value`); `--read-only`, `--tls-certificate` und `--tls-key` schalten ihn auf nur lesend oder HTTPS. | `remote-cache` |")
sub(README, "`bun repo cache remote status [--tool=nx|bazel]` prints the resolved contract, the rendering with its unsupported fields and a read-only probe of the route the tool uses.", "`bun repo remote-cache status` prints the resolved contract, both renderings with their unsupported fields and a read-only probe of the route each tool uses.")
sub(README, "`bun repo cache remote status [--tool=nx|bazel]` zeigt den aufgelösten Vertrag, die Umsetzung samt nicht unterstützter Felder und eine lesende Probe der Route des Werkzeugs.", "`bun repo remote-cache status` zeigt den aufgelösten Vertrag, beide Umsetzungen samt nicht unterstützter Felder und eine lesende Probe der Route jedes Werkzeugs.")
region(README, "- **Server.** `bun repo cache serve [--port <n>] [--host <address>]", "\n", "- **Server.** `bun repo remote-cache serve [--port <n>] [--host <address>] [--directory <path>] [--max-bytes <n>] [--read-only] [--tls-certificate <path> --tls-key <path>]` speaks Bazel's HTTP cache protocol (`GET`, `HEAD`, `PUT` on `/ac/<sha256>` and `/cas/<sha256>`; a blob is refused with 400 unless its content has the digest of its address; an action result is refused with 400 unless it is a well-formed `ActionResult` whose blobs are stored, and is answered as a miss once a blob left the store) and the Nx self-hosted cache API (`GET`, `PUT` on `/v1/cache/<hash>`, 409 on overwrite, 401 with `text/plain`, 403 for a read token that writes or on a read-only server), each under an optional `/<instance>` prefix, over HTTP or, with a certificate, HTTPS. `/healthz` and `/status` are plain JSON. Bearer tokens come from `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` and `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; without a token it refuses to listen beyond loopback. The store lives in the remote cache home of the cache ledger (`.🧬semio/🦑️repo/⚡️cache/remote-cache`, 2.5 GiB of a 3 GiB budget, 512 MiB per entry, 30 days idle by default), writes atomically, reserves bytes before streaming, evicts least recently used entries and publishes progress while it scans; one server owns a directory, `remote-cache gc` and `cache gc` may collect it meanwhile. Cancelling the command stops the server. The dashboard tool `build-remote-cache-serve` starts it on port 6090.")
region(README, "- **Server.** `bun repo cache serve [--port <n>] [--host <Adresse>]", "\n", "- **Server.** `bun repo remote-cache serve [--port <n>] [--host <Adresse>] [--directory <Pfad>] [--max-bytes <n>] [--read-only] [--tls-certificate <Pfad> --tls-key <Pfad>]` spricht Bazels HTTP-Cache-Protokoll (`GET`, `HEAD`, `PUT` auf `/ac/<sha256>` und `/cas/<sha256>`; ein Blob wird mit 400 abgelehnt, wenn sein Inhalt nicht den Digest seiner Adresse hat; ein Action-Ergebnis wird mit 400 abgelehnt, wenn es kein wohlgeformtes `ActionResult` mit gespeicherten Blobs ist, und gilt als Fehltreffer, sobald ein Blob den Speicher verlassen hat) und die Nx-Self-Hosted-Cache-API (`GET`, `PUT` auf `/v1/cache/<hash>`, 409 beim Überschreiben, 401 mit `text/plain`, 403 für ein schreibendes Lese-Token oder auf einem Nur-Lese-Server), jeweils mit optionalem Präfix `/<instance>`, über HTTP oder mit Zertifikat über HTTPS. `/healthz` und `/status` sind schlichtes JSON. Bearer-Tokens stammen aus `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` und `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; ohne Token lauscht er nur auf Loopback. Der Speicher liegt in der Remote-Cache-Heimat der Cache-Bilanz (`.🧬semio/🦑️repo/⚡️cache/remote-cache`, standardmäßig 2,5 GiB von 3 GiB Budget, 512 MiB je Eintrag, 30 Tage Leerlauf), schreibt atomar, reserviert Bytes vor dem Streamen, entfernt die am längsten ungenutzten Einträge und meldet beim Einlesen den Fortschritt; ein Server besitzt ein Verzeichnis, `remote-cache gc` und `cache gc` dürfen es währenddessen einsammeln. Abbrechen des Befehls beendet den Server. Das Dashboard-Werkzeug `build-remote-cache-serve` startet ihn auf Port 6090.")

# ---- wire scenarios of the new modes (strings only; binary action results are covered by the action-result tests)
WIRE = R + "/🧪️tests/🧫️fixtures/📡️wire.json"
HELLO = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
OTHER = "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7"
POINTER = "\x12\x49\x0a\x01p\x12\x44\x0a\x40" + HELLO + "\x10\x05"


def wire(value):
    names = {scenario["name"] for scenario in value["scenarios"]}
    added = [
        {
            "name": "read-only server: every write is refused, reads still work",
            "server": {"tokens": {}, "readOnly": True},
            "steps": [
                {"name": "a blob cannot be written", "request": {"method": "PUT", "path": "/cas/" + HELLO, "body": "hello"}, "response": {"status": 403, "body": "Forbidden: this cache is read-only\n"}},
                {"name": "an Nx archive cannot be written", "request": {"method": "PUT", "path": "/v1/cache/1", "body": "x"}, "response": {"status": 403}},
                {"name": "nothing was stored", "request": {"method": "GET", "path": "/cas/" + HELLO}, "response": {"status": 404}},
                {"name": "health still answers", "request": {"method": "GET", "path": "/healthz"}, "response": {"status": 200}},
            ],
        },
        {
            "name": "read-only server with tokens: a write token does not lift read-only, a missing token is still 401",
            "server": {"tokens": {"write": "writer-token"}, "readOnly": True},
            "steps": [
                {"name": "no token", "request": {"method": "PUT", "path": "/cas/" + HELLO, "body": "hello"}, "response": {"status": 401}},
                {"name": "write token", "request": {"method": "PUT", "path": "/cas/" + HELLO, "body": "hello", "headers": {"authorization": "Bearer writer-token"}}, "response": {"status": 403, "body": "Forbidden: this cache is read-only\n"}},
                {"name": "read with the token", "request": {"method": "GET", "path": "/cas/" + HELLO, "headers": {"authorization": "Bearer writer-token"}}, "response": {"status": 404}},
            ],
        },
        {
            "name": "structure validation: a body that is no ActionResult is refused, an empty result is stored and read back",
            "server": {"tokens": {}, "validation": "structure"},
            "steps": [
                {"name": "text is not a protobuf message", "request": {"method": "PUT", "path": "/ac/" + HELLO, "body": "hello"}, "response": {"status": 400, "body": "Invalid action result: wire type 4 is not allowed\n"}},
                {"name": "the refused result was not stored", "request": {"method": "GET", "path": "/ac/" + HELLO}, "response": {"status": 404}},
                {"name": "the empty message is a valid result", "request": {"method": "PUT", "path": "/ac/" + HELLO, "body": ""}, "response": {"status": 200}},
                {"name": "it reads back empty", "request": {"method": "GET", "path": "/ac/" + HELLO}, "response": {"status": 200, "headers": {"content-length": "0"}, "body": ""}},
            ],
        },
        {
            "name": "complete validation: a result is accepted only after the blob it names is stored",
            "server": {"tokens": {}, "validation": "complete"},
            "steps": [
                {"name": "the result names a blob that is not stored yet", "request": {"method": "PUT", "path": "/ac/" + OTHER, "body": POINTER}, "response": {"status": 400, "body": "Invalid action result: blob " + HELLO + " is not in the content store\n"}},
                {"name": "the blob is stored", "request": {"method": "PUT", "path": "/cas/" + HELLO, "body": "hello"}, "response": {"status": 200}},
                {"name": "now the result is accepted", "request": {"method": "PUT", "path": "/ac/" + OTHER, "body": POINTER}, "response": {"status": 200}},
                {"name": "and read back byte for byte", "request": {"method": "GET", "path": "/ac/" + OTHER}, "response": {"status": 200, "body": POINTER}},
            ],
        },
    ]
    value["scenarios"] = [s for s in value["scenarios"] if s["name"] not in {a["name"] for a in added}] + added
    return value


js(WIRE, wire)

import glob as _glob
import os as _os
for _name in sorted(_glob.glob(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "edits_*.py"))):
    exec(compile(open(_name, encoding="utf8").read(), _name, "exec"), globals())

text(SCHEMA, """    { "$ref": "#/definitions/Status" }
  ],""", """    { "$ref": "#/definitions/Status" },
    { "$ref": "#/definitions/ServerStatus" },
    { "$ref": "#/definitions/CollectionReport" },
    { "$ref": "#/definitions/GcResult" },
    { "$ref": "#/definitions/TextCatalogue" }
  ],""")
