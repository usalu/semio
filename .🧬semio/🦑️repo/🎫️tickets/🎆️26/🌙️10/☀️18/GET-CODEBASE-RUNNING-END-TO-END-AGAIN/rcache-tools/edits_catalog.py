# operations catalogue: `cache remote status` and `cache serve`, their flags, EN and DE help.
CATALOG = B + "/⚙️operations/🔣️catalog.json"

edit(CATALOG, r'''      "description": { "en": "Positive number of cores for Cargo.", "de": "Positive Anzahl von Kernen für Cargo." }
    }
  ],
  "operations": [''', r'''      "description": { "en": "Positive number of cores for Cargo.", "de": "Positive Anzahl von Kernen für Cargo." }
    },
    {
      "name": "--port",
      "value": "separate",
      "valueName": "port",
      "operations": ["cache"],
      "description": { "en": "cache serve: port to listen on; 0 picks a free port (default from the build policy).", "de": "cache serve: Port, auf dem gelauscht wird; 0 wählt einen freien Port (Standard aus der Build-Policy)." }
    },
    {
      "name": "--host",
      "value": "separate",
      "valueName": "address",
      "operations": ["cache"],
      "description": { "en": "cache serve: address to listen on; beyond loopback a token is required.", "de": "cache serve: Adresse, auf der gelauscht wird; außerhalb von Loopback ist ein Token nötig." }
    },
    {
      "name": "--directory",
      "value": "separate",
      "valueName": "path",
      "operations": ["cache"],
      "description": { "en": "cache serve: store directory, relative to the workspace root.", "de": "cache serve: Speicherverzeichnis, relativ zum Workspace-Wurzelverzeichnis." }
    },
    {
      "name": "--max-bytes",
      "value": "separate",
      "valueName": "bytes",
      "operations": ["cache"],
      "description": { "en": "cache serve: upper bound of the store; the least recently used entries are evicted beyond it.", "de": "cache serve: Obergrenze des Speichers; darüber werden die am längsten ungenutzten Einträge entfernt." }
    }
  ],
  "operations": [''')

region(CATALOG, '    {\n      "name": "cache",\n', None, r'''    {
      "name": "cache",
      "synopsis": "cache status|clear|remote status|serve",
      "summary": {
        "en": "Show or clear the selected tool's local task cache, show the remote cache contract, or serve a remote cache.",
        "de": "Zeigt oder leert den lokalen Task-Cache des gewählten Werkzeugs, zeigt den Remote-Cache-Vertrag oder stellt einen Remote-Cache bereit."
      },
      "implementations": { "nx": "adapter", "bazel": "adapter" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "subcommands": [
        {
          "name": "status",
          "synopsis": "cache status",
          "summary": {
            "en": "Print a CacheStatus: entries, bytes and, for Bazel, the recorded build activity.",
            "de": "Gibt einen CacheStatus aus: Einträge, Bytes und, für Bazel, die aufgezeichnete Build-Aktivität."
          }
        },
        { "name": "clear", "synopsis": "cache clear", "summary": { "en": "Empty the cache and print a CacheClear.", "de": "Leert den Cache und gibt einen CacheClear aus." } },
        {
          "name": "remote",
          "synopsis": "cache remote status",
          "summary": {
            "en": "Print a RemoteCacheStatus: the remote cache contract from the build policy and the environment, how the selected tool renders it, and whether the endpoint answers.",
            "de": "Gibt einen RemoteCacheStatus aus: den Remote-Cache-Vertrag aus Build-Policy und Umgebung, wie das gewählte Werkzeug ihn umsetzt und ob der Endpunkt antwortet."
          }
        },
        {
          "name": "serve",
          "synopsis": "cache serve",
          "summary": {
            "en": "Serve a bounded remote cache for Bazel and Nx from this machine until stopped.",
            "de": "Stellt von diesem Rechner aus einen begrenzten Remote-Cache für Bazel und Nx bereit, bis er gestoppt wird."
          }
        }
      ],
      "flags": ["--port", "--host", "--directory", "--max-bytes"],
      "examples": [
        { "command": "bun repo cache status --tool=bazel", "description": { "en": "Show the Bazel disk cache and build activity.", "de": "Zeigt den Bazel-Disk-Cache und die Build-Aktivität." } },
        { "command": "bun repo cache clear", "description": { "en": "Empty the Nx task cache.", "de": "Leert den Nx-Task-Cache." } },
        { "command": "bun repo cache serve", "description": { "en": "Serve a local remote cache on 127.0.0.1:6090.", "de": "Stellt einen lokalen Remote-Cache auf 127.0.0.1:6090 bereit." } },
        {
          "command": "SEMIO_REMOTE_CACHE_ENDPOINT=http://127.0.0.1:6090 bun repo cache remote status --tool=bazel",
          "description": { "en": "Show how Bazel uses the remote cache and whether it answers.", "de": "Zeigt, wie Bazel den Remote-Cache nutzt und ob er antwortet." }
        }
      ],
      "notes": [
        "status prints a CacheStatus, clear prints a CacheClear; both stop at the next checkpoint when the caller cancels and publish progress while walking or removing entries.",
        "Nx: status walks the nx/artifacts directory of the rust cache partition, clear runs nx reset --onlyCache inside the bounded Nx namespace.",
        "Bazel: status counts the files of the disk cache entry folders (ac and cas) of the products partition and adds the recorded build activity (builds, actions, executed, cacheHits) read from Bazel's build event stream; clear stops the persistent servers, removes their output bases (Bazel's local action cache) and empties the disk cache, so the next build is a miss; toolchains and modules stay.",
        "The remote cache is off until SEMIO_REMOTE_CACHE_ENDPOINT names an http or https origin. SEMIO_REMOTE_CACHE_INSTANCE, _ACCESS (read-only|read-write), _UPLOAD, _COMPRESSION, _DOWNLOAD (minimal|toplevel|all), _TIMEOUT_SECONDS and _RETRIES override the defaults of the build policy; the access token is read from the variable named by SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT (default SEMIO_REMOTE_CACHE_TOKEN) and is never written to disk.",
        "Bazel renders the contract into --remote_cache, --remote_instance_name, --remote_upload_local_results, --remote_cache_compression, --remote_download_outputs, --remote_timeout and --remote_retries and passes the token through a credential helper. Nx renders it into its self-hosted remote cache variables (server and access token); it cannot honour compression, download mode, timeouts or retries and leaves read-only to the server's token scope. remote status prints both facts.",
        "serve speaks Bazel's HTTP cache protocol (GET, HEAD and PUT on /ac/<sha256> and /cas/<sha256>, blobs verified against their digest) and the Nx self-hosted cache API (GET and PUT on /v1/cache/<hash>, 409 when an entry exists), each under an optional /<instance> prefix. Bearer tokens come from SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN and SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN; without a token it refuses to listen beyond loopback. The store is bounded and evicts the least recently used entries; serve publishes progress while it scans the store and stops at the next checkpoint when the caller cancels."
      ]
    }
  ]
}
''')
