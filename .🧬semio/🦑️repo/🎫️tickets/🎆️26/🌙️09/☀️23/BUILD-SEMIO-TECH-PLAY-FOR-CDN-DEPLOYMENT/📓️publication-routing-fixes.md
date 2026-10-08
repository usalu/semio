# Publication Routing Fixes

Implemented the two production defects confirmed by the second release-assets audit and the final publication byte-accounting defect reproduced during the follow-up review.

## Behavior

- The shared page resolver and generated shard-worker fetch prelude select CDN routes using a safely decoded pathname. They retain the original encoded pathname, query and fragment in the outgoing request.
- Literal, percent-encoded, absolute same-origin, URL and Request representations now resolve to the same satellite. Prefix lookalikes, foreign origins, network-relative URLs, malformed encodings and encoded path separators remain outside relocation.
- Final worker publication rewrites literal relative dynamic imports to absolute module-satellite URLs before copying the same script to the app origin. The diagnostic WASI CLI shim consequently uses the same satellite module instance as the component bridge.
- Request HEAD method, headers and credentials survive worker relocation. POST Request instances stay unchanged.
- Returned/logged sizes and budget enforcement use the final published bytes, including metadata, rewritten URLs, worker prelude and same-origin worker copy.

## Test Evidence

Language-neutral fixtures precede the source fixes. Existing third-party `whatwg-url` and `undici` packages independently normalize the expected URL and Request cases. A real generated worker executes in a separate Node VM context with native URL and Request objects and a recorded fetch transport.

Red baseline: `bun nx run @semio-tech/semio-tech-play:test --skip-nx-cache --excludeTaskDependencies -- long --testNamePattern=relocates|publishes a worker` completed with 7 failed and 5 passed assertions. Failures included encoded module routes, emoji absolute URLs, route-root query suffixes and the unresolved relative diagnostic import.

Green focused suite: `bun nx run @semio-tech/semio-tech-play:test --skip-nx-cache --excludeTaskDependencies -- long --testNamePattern=play CDN pages` completed with 15 passed tests. Runtime output included `[DEBUG] Published worker URL/Request routing and diagnostic import verified`.

Full Play suite after the final fixture edit: `bun nx run @semio-tech/semio-tech-play:test --skip-nx-cache --excludeTaskDependencies -- long` completed with 54 passed and 9 failed tests. All 15 publication tests passed. Eight failures came from the current canonical catalog omitting `cad`; the build agent is adding the required post-materialization catalog gate. One fresh-build test read the project file from the wrong parent folder; the build agent has fixed that path. These failures are reported as failures, not a full-suite acceptance result.

Final publication follow-up: a real 3-failure byte-accounting regression baseline was followed by an 18-passing-test focused suite. The final suite also parses the real shard-worker generator with third-party es-module-lexer, verifies its complete import inventory before/after publication and confirms activation expression imports remain unchanged. Runtime output included `[DEBUG] Actual generated worker import forms and publication targets verified`.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/🟦️.ts`
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpages/🟦️.ts`
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playpages/🧫️fixtures/🔣️.json`

## Release Boundary

These checks validate source routing and final worker publication in a synthetic site. The build agent has been notified to regenerate the final production bundle and pages with these edits. The root production harness remains responsible for the actual app/worker runtime against the fresh published artifacts and HTTP MIME/CORS responses.

## Stable Asset Cache Regression

A language-neutral `cache` fixture in Play's publication corpus describes all four hosts, stable representative files, previous long-immutable response headers and the required `no-cache` policy. The production publication test parses each emitted `_headers` and uses existing `http-cache-semantics` to prove the previous policy permits immediate reuse while the current policy requires revalidation; satellite CORS is still present. The regression actually failed before repair (1 failed, 18 passed; app `_headers` absent), then passed after writing revalidation metadata to every host (19 passed). Logs: `🗑️generated/publication-cache-red.log` and `publication-cache-green.log`.

The same fixture now seeds obsolete files in every previous page and a retired fifth page. The independent `glob` inventory verifies none survives republishing. The final rerun including these clean-republication assertions passed: 19 tests, exit 0; its separate log is `🗑️generated/publication-cache-clean-green.log`. The completeness auditor now requires `Cache-Control: no-cache` on all four final pages and checks literal references passed through `__semioVersionedComponentAssetUrl`, in addition to native `new URL` asset references. Provider metadata consumption and actual provider cache invalidation are unverified deployment operations.
