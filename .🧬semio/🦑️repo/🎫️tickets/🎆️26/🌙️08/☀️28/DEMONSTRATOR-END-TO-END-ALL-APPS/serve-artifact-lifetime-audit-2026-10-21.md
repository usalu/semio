# Retained Serve Artifact Lifetime

Read-only authored serve audit. No server was launched or changed.

`@semio-tech/mit-bestand-demonstrator:serve` dispatches runtime/📜️script.ts ServeScript → serveVite, after activate-prototype. ServeScript validates completed activation and serves from authored Vite config; it does not read SEMIO_TEST_ARTIFACT_DIR. No E2E service-session file is opened by ordinary serve.

Concrete inherited environment use: Nx executeLiteralBuildArgumentsV1 resolves SEMIO_TEST_ARTIFACT_DIR and publishes `<digest>.json` private command ingress there. Its runtime bridge reads/digest-validates that file ONCE before executing the command, deletes private command-directory environment fields, then retains parsed source/readiness state in memory. It does not reread that file while serve is live. Therefore startup must complete before deleting the ticket output directory; after observed `Demonstrator ready` the command ingress dependency has already been consumed.

Live Vite filesystem state uses withViteStorageV1 products partition layout (`layout.state`, managed temporary/cache directories), artifact input readers, compiler-state storage and product artifact snapshots. None of this live state is rooted in inherited ticket/generated/serve. Serve callbacks/transform admission use retained control and managed store environment. The compiler owner also explicitly sets its admitted capture directory from managed storage, not inherited arbitrary ticket captures. Stdout can stay attached to the Native session; no serve.log is required.

On this authored ordinary serve path no post-ready runtime read/write dependency on the ticket generated directory was found. Generated-only cleanup after readiness can therefore remove ticket ingress/output artifacts while leaving the live managed service and authored/source reports intact. This conclusion does not cover starting new tasks using the deleted command ingress, E2E serve-test sessions, or deleting managed products/cache state. No hypothetical extra cleanup constraint is introduced.

Exact source evidence:

- /Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/📋️project.json serve target.
- /Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts ServeScript.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs executeLiteralBuildArgumentsV1.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟦️.ts runScriptProgram detached ingress.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts serveVite.
- /Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🟦️.ts withViteStorageV1.
