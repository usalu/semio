# Workflow Artifact Creation in the Local Dev Shell

The browser resolves a Catalogue drop against the owning Space's authenticated creation catalog. It carries the selected plugin, app, node, and position through the existing creation worker. A Ready result can resume the Workflow only for the same request, catalog generation, Space, artifact schema, and dialect. The native continuation then uses the standard spawned-document opening.

The Studio route reads the complete Hub directory history through a dedicated, client-owned read lane. The worker sends an explicit history-ready message to that owner once it reaches the sealed frontier. The ordinary Space Index subscription stays independent. Studio then looks up the registered Workflow artifact by its exact dialect and schema, and opens that document through the ordinary document owner. A Space without a registered Workflow uses the existing creation saga. SQLite, PostgreSQL, and Neo4j enforce one Studio genesis in the existing Space transaction. The browser rereads the sealed directory history when a competing request publishes first.

## Local acceptance

Run the first-party local Hub on its default loopback port, 8787. Its current trusted catalog must contain the Space Studio Workflow and GIS Terrain entries.

```sh
repo run os-hub:dev --lang=en
repo ./📜️script.ts dev playground s react prototype
```

The commands hold their respective processes and run in separate terminals. The regular s browser row serves port 6070. The serve joins the local Hub by default. To select an already running first-party local Hub explicitly, set `S_HUB_URL` to its loopback URL for the second command. Leave `S_LOCAL_ONLY` unset.

Run [verify-s-shell-terrain.mjs](./verify-s-shell-terrain.mjs) through `repo tools exec --light node -- <absolute-path-to-runner>`. The default run starts a fresh Chromium profile, observes the local Dev broker offer and authenticated Hub session, and creates a private Atelier through Home. It waits for the mounted Workflow before dragging Terrain into the canvas. The runner removes its disposable Space through Home. An explicit `http://127.0.0.1:6070/spaces/<space-id>/studio` runs Studio acceptance in an existing Space. An explicit `http://127.0.0.1:6070/spaces/<space-id>` uses the Space Index's Create Artifact dialog. Both explicit modes keep the existing Space.

## Verification

The targeted renderer test entry passed six tests, including Ajv agreement for the request, exact Ready correlation, and the worker-wire scenario where another creator wins after the browser starts creation. The same scenario rejects unrelated directory batches for the same Space and confirms that closing the lookup preserves the independent Index lane. The test ran with the owned Vitest configuration outside the busy project-planning queue. A full renderer typecheck remained red with no diagnostics in the modified creation and route files. Node syntax checking passed for the acceptance runner.

The live runner reached its HTTP preflight. Both regular s on 6070 and the local Hub on 8787 refused connections. Scoped GIS on 6083 returned HTML. No live Studio or backend acceptance was claimed from the scoped preview.

## Build planning

The Hub build and admin build use ordinary repository-owned invocations. Both receive their Original process authority from the repository launcher. The package scripts require that authority. Direct script execution cannot supply it.

`repo run os-hub:build-dev --lang=en --exclude-dependencies --output=stream` and `repo run os-hub-admin:build --lang=en --exclude-dependencies --output=stream` ask the resident planner before handing work to a backend. The flag narrows subsequent owner admission. It still queues in the shared service while that service is reading the workspace inventory. Selecting the in-house backend changes the executor after planning. `--graph-policy=current` can read an existing sealed graph through the service. A nested child can skip the planner only with its existing exact graph publication. The native recovery allowlist omits these Hub targets.

The canonical artifact-creation test invocation with `--exclude-dependencies` also stayed in the resident queue. It was cancelled after the equivalent owned Vitest run passed with the repository configuration and explicit policy. The local Hub or admin build was not duplicated.
