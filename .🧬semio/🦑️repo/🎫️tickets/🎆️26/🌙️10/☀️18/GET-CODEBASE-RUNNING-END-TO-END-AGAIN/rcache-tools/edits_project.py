# root command registry: the dashboard tools of `cache remote status` and `cache serve`, beside build-cache-status and build-cache-clear.
edit("📋️project.json", r'''            "command": ["bun", "repo", "cache", "clear"]
          },
''', r'''            "command": ["bun", "repo", "cache", "clear"]
          },
          {
            "id": "build-cache-remote-status",
            "title": { "en": "Show the remote build cache", "de": "Remote-Build-Cache anzeigen" },
            "description": { "en": "Report the remote cache contract, how the chosen backend uses it and whether the endpoint answers; nothing is stored.", "de": "Den Remote-Cache-Vertrag, die Umsetzung im gewählten Werkzeug und die Erreichbarkeit des Endpunkts melden; es wird nichts gespeichert." },
            "verb": "inspect",
            "command": ["bun", "repo", "cache", "remote", "status"]
          },
          {
            "id": "build-cache-serve",
            "title": { "en": "Serve the local remote build cache", "de": "Lokalen Remote-Build-Cache bereitstellen" },
            "description": { "en": "Serve a bounded remote cache for Bazel and Nx on this machine until stopped; clients point SEMIO_REMOTE_CACHE_ENDPOINT at it.", "de": "Einen begrenzten Remote-Cache für Bazel und Nx auf diesem Rechner bereitstellen, bis er gestoppt wird; Clients richten SEMIO_REMOTE_CACHE_ENDPOINT darauf." },
            "verb": "serve",
            "command": ["bun", "repo", "cache", "serve", "--port", "{port}"],
            "continuous": true,
            "ready": { "port": 6090, "portEnv": "SEMIO_REMOTE_CACHE_PORT", "path": "/healthz" }
          },
''')
