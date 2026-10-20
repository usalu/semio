# README of the repo product: the remote cache contract in English and German.
README = "🧰️framework/🛍️products/🦑️repo/README.md"

region(README, '- **Remote cache switch** (off by default, no flag is passed while the endpoint is unset):', '- **Cache operations.**', '''- **Remote cache.** Rendered from the tool-neutral remote cache contract (section "Remote cache"): `--remote_cache`, `--remote_instance_name`, `--[no]remote_upload_local_results`, `--[no]remote_cache_compression`, `--remote_download_outputs`, `--remote_timeout` and `--remote_retries`, and nothing while no endpoint is configured. `--skip-remote-cache` drops it for one invocation; `--skip-cache` adds `--noremote_accept_cached`. The adapter restores task snapshots from `bazel-bin`, so `minimal` downloads are refused. The access token never appears in a flag: a credential helper in the products `tmp` folder holds only the variable name and the token travels in the environment of the Bazel client (`--remote_header` would copy it into Bazel's `java.log` and build event stream).
''')

edit(README, '## 🎛️ Command registry\n', '''## 🌐️ Remote cache

One schema-first contract (`🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json`, authored defaults in `🏗️build/🔣️policy.json`), two renderings and one server. The remote cache is off until an endpoint is set. The contract holds no secret.

| Variable | Contract field | Default |
|---|---|---|
| `SEMIO_REMOTE_CACHE_ENDPOINT` | `endpoint`: `http://` or `https://` origin | none (off) |
| `SEMIO_REMOTE_CACHE_INSTANCE` | `instance`: namespace, the first URL path segment | `main` |
| `SEMIO_REMOTE_CACHE_ACCESS` | `access`: `read-only` or `read-write` | `read-only` |
| `SEMIO_REMOTE_CACHE_UPLOAD` | `upload`: push local results (only effective with `read-write`) | `true` |
| `SEMIO_REMOTE_CACHE_COMPRESSION` | `compression` | `true` |
| `SEMIO_REMOTE_CACHE_DOWNLOAD` | `download`: `minimal`, `toplevel` or `all` | `toplevel` |
| `SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS`, `SEMIO_REMOTE_CACHE_RETRIES` | `timeoutSeconds` (1-600), `retries` (0-10) | `30`, `3` |
| `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT` | `auth.tokenEnvironment`: name of the variable that holds the bearer token | `SEMIO_REMOTE_CACHE_TOKEN` |

- **Bazel** renders every field into the flags above and passes the token through a credential helper scoped to the cache host. **Nx** renders `endpoint/instance` into `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` and the token into `NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN`; Nx 23 has no switch for compression, download mode, timeouts, retries or withholding uploads, so read-only is enforced by a read-only token on the server. `bun repo cache remote status [--tool=nx|bazel]` prints the resolved contract, the rendering with its unsupported fields and a read-only probe of the route the tool uses.
- **Server.** `bun repo cache serve [--port <n>] [--host <address>] [--directory <path>] [--max-bytes <n>]` speaks Bazel's HTTP cache protocol (`GET`, `HEAD`, `PUT` on `/ac/<sha256>` and `/cas/<sha256>`, blobs verified against their digest, 400 on a mismatch) and the Nx self-hosted cache API (`GET`, `PUT` on `/v1/cache/<hash>`, 409 on overwrite, 401 with `text/plain`, 403 for a read token that writes), each under an optional `/<instance>` prefix. `/healthz` and `/status` are plain JSON. Bearer tokens come from `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` and `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; without a token it refuses to listen beyond loopback. The store (`.cache/remote-cache`, 4 GiB, 512 MiB per entry, 30 days idle by default) writes atomically, reserves bytes before streaming, evicts least recently used entries and publishes progress while it scans; cancelling the command stops the server. The dashboard tool `build-cache-serve` starts it on port 6090.
- **Tests.** `🏗️build/🌐️remote-cache/🧪️tests` replays the JSON fixtures (contract, renderings, wire scenarios, eviction) against Ajv, Python's `re` and `http.client` and `hashlib`; with `SEMIO_TEST_BAZEL_BINARY` a real Bazel builds against the server and hits the remote cache after `bazel clean`, and with `SEMIO_TEST_NX_BIN` (a pristine nx 23 install; the repository's patched Nx refuses to run in a scratch workspace) a real Nx does the same.

## 🎛️ Command registry
''')

region(README, '- **Remote-Cache-Schalter** (standardmäßig aus;', '- **Cache-Operationen.**', '''- **Remote-Cache.** Wird aus dem werkzeugneutralen Remote-Cache-Vertrag erzeugt (Abschnitt "Remote-Cache"): `--remote_cache`, `--remote_instance_name`, `--[no]remote_upload_local_results`, `--[no]remote_cache_compression`, `--remote_download_outputs`, `--remote_timeout` und `--remote_retries`, und nichts, solange kein Endpunkt gesetzt ist. `--skip-remote-cache` lässt ihn für einen Aufruf weg, `--skip-cache` ergänzt `--noremote_accept_cached`. Der Adapter stellt Task-Snapshots aus `bazel-bin` wieder her, deshalb wird `minimal` abgelehnt. Das Zugriffstoken steht nie in einem Flag: ein Credential-Helper im `tmp`-Ordner der Produkte kennt nur den Variablennamen, das Token reist in der Umgebung des Bazel-Clients (`--remote_header` würde es in Bazels `java.log` und den Build-Event-Stream kopieren).
''')

edit(README, '### Befehlsregister\n', '''### Remote-Cache

Ein schema-erster Vertrag (`🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json`, Standardwerte in `🏗️build/🔣️policy.json`), zwei Umsetzungen und ein Server. Ohne Endpunkt ist der Remote-Cache aus. Der Vertrag enthält kein Geheimnis.

| Variable | Feld | Standard |
|---|---|---|
| `SEMIO_REMOTE_CACHE_ENDPOINT` | `endpoint`: `http://`- oder `https://`-Ursprung | keiner (aus) |
| `SEMIO_REMOTE_CACHE_INSTANCE` | `instance`: Namensraum, erstes URL-Pfadsegment | `main` |
| `SEMIO_REMOTE_CACHE_ACCESS` | `access`: `read-only` oder `read-write` | `read-only` |
| `SEMIO_REMOTE_CACHE_UPLOAD` | `upload`: lokale Ergebnisse hochladen (wirkt nur mit `read-write`) | `true` |
| `SEMIO_REMOTE_CACHE_COMPRESSION` | `compression` | `true` |
| `SEMIO_REMOTE_CACHE_DOWNLOAD` | `download`: `minimal`, `toplevel` oder `all` | `toplevel` |
| `SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS`, `SEMIO_REMOTE_CACHE_RETRIES` | `timeoutSeconds` (1-600), `retries` (0-10) | `30`, `3` |
| `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT` | `auth.tokenEnvironment`: Name der Variable mit dem Bearer-Token | `SEMIO_REMOTE_CACHE_TOKEN` |

- **Bazel** setzt jedes Feld in die oben genannten Flags um und reicht das Token über einen auf den Cache-Host begrenzten Credential-Helper durch. **Nx** setzt `endpoint/instance` in `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` und das Token in `NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN` um; Nx 23 kennt keinen Schalter für Kompression, Download-Modus, Zeitlimit, Wiederholungen oder das Zurückhalten von Uploads, deshalb setzt ein Nur-Lese-Token auf dem Server read-only durch. `bun repo cache remote status [--tool=nx|bazel]` zeigt den aufgelösten Vertrag, die Umsetzung samt nicht unterstützter Felder und eine lesende Probe der Route des Werkzeugs.
- **Server.** `bun repo cache serve [--port <n>] [--host <Adresse>] [--directory <Pfad>] [--max-bytes <n>]` spricht Bazels HTTP-Cache-Protokoll (`GET`, `HEAD`, `PUT` auf `/ac/<sha256>` und `/cas/<sha256>`, Blobs gegen ihren Digest geprüft, 400 bei Abweichung) und die Nx-Self-Hosted-Cache-API (`GET`, `PUT` auf `/v1/cache/<hash>`, 409 beim Überschreiben, 401 mit `text/plain`, 403 für ein schreibendes Lese-Token), jeweils mit optionalem Präfix `/<instance>`. Bearer-Tokens stammen aus `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` und `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; ohne Token lauscht er nur auf Loopback. Der Speicher (`.cache/remote-cache`, standardmäßig 4 GiB, 512 MiB je Eintrag, 30 Tage Leerlauf) schreibt atomar, reserviert Bytes vor dem Streamen, entfernt die am längsten ungenutzten Einträge und meldet beim Einlesen den Fortschritt; Abbrechen des Befehls beendet den Server. Das Dashboard-Werkzeug `build-cache-serve` startet ihn auf Port 6090.
- **Tests.** `🏗️build/🌐️remote-cache/🧪️tests` spielt die JSON-Fixtures (Vertrag, Umsetzungen, Leitungsszenarien, Verdrängung) gegen Ajv, Pythons `re`, `http.client` und `hashlib` ab; mit `SEMIO_TEST_BAZEL_BINARY` baut ein echtes Bazel gegen den Server und trifft nach `bazel clean` den Remote-Cache, mit `SEMIO_TEST_NX_BIN` (eine unveränderte Nx-23-Installation; das gepatchte Nx des Repositorys läuft nicht in einem Scratch-Workspace) tut ein echtes Nx dasselbe.

### Befehlsregister
''')
