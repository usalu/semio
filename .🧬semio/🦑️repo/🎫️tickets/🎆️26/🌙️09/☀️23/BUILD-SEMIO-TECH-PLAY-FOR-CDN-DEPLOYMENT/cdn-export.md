# CDN export of the verified play release

## Command

Needs a finished release cohort in `.cache/b/p/m/play/release/pages` (from `build`); it does not depend on `build`.

```
# SEMIO_BUILD_MODE=release must be set: build, verify, export and test-release refuse an unset or prototype mode
repo run @semio-tech/semio-tech-play:release-export                  # -> 🏢️semio-tech/🎡️play/dist/cdn
repo run @semio-tech/semio-tech-play:release-export -- /some/folder  # custom, must be missing or empty
```
Native fallback: `SEMIO_BUILD_MODE=release repo ./🔨️modules/📦️site/📜️script.ts export [folder]` from `🏢️semio-tech/🎡️play` (needs the repo script invocation, so use `repo run` or `native-play.sh`-style launch).

## Layout (upload the folder content as is)

```
dist/cdn/
  play/     one host root  (CNAME, _headers, .nojekyll, 🧾️page.json, files)
  map/      host root
  media/    host root
  modules/  host root
  🔣️cohort.json   commit marker, the four manifest digests
  🔣️cdn.json      rules, hosts, headers, content types (schema 🧬️schema/🔣️.json)
```
Each family directory is the root of the host in its `CNAME` (play.<apex>, map.assets.<apex>, media.assets.<apex>, modules.assets.<apex>).

## CDN rules (also in `🔣️cdn.json`)

- Serve each family at the root of its host, no sub-path.
- Missing file answers 404, no SPA fallback (only `play` has an HTML entry).
- `Cache-Control: no-cache` on all hosts; `Access-Control-Allow-Origin: *` on map, media, modules (from each family `_headers`; provider must apply them if it ignores `_headers`).
- Exact content types: `.wasm` as `application/wasm` (streaming instantiation), `.js`/`.mjs` as `text/javascript`, `.json` as `application/json`, `.css`, `.html`, `.woff2`, `.png`; the descriptor lists present extensions and `unmappedExtensions` (for example `.glb`, `.3dm`) that the provider must type.
- No COOP/COEP (no cross-origin isolation needed; `require-corp` would break cross-origin assets).
- Upload all four families together from one export (one generation).

## Behaviour

Runs under `withPublishedPlayPagesV1` (retained readers). Per file: read, check size and sha256 against the manifest, write into `dist/cdn.partial`, reread and recheck. Then copies `🔣️cohort.json`, writes `🔣️cdn.json` (validated against its schema) and renames the staging folder atomically. Any mismatch, missing cohort/manifest, non-empty target or cancellation (SIGINT, SIGTERM, lease control) removes the staging folder. Errors are bilingual `English / Deutsch`. Progress is printed every 250 files.

## Tests

`🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/📤️export/🧪️tests/🟦️.ts` (bun, fixture cohort in the ticket `🗑️generated`, independent `Bun.CryptoHasher` and Ajv oracles): 2 tests, 43 assertions, all pass with pinned Bun 1.4.2 (`bun test <file>`). Covered: exact bytes, manifest and cohort copy, tree equality, descriptor schema and content, tampered file / manifest, missing cohort, non-empty target, cancellation. Not run: a real export (no cohort exists yet; a full tsc of the transitive graph ran out of memory, new files were reviewed by hand).
