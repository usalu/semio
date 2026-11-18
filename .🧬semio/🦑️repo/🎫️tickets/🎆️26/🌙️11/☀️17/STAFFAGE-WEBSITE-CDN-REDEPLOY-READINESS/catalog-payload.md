# Compact Catalog Payload

## Problem

The overview loaded all 152 catalog asset files as chunks that embedded the pretty-printed file text verbatim (about 37 % of the bytes were indentation) and then ran `JSON.parse` plus `revisionOfText` (sha256) in the browser for every asset.

## Design

A Vite plugin owned by the staffage react target serves `<asset file>?staffage-asset` as

```js
export default { revision: "sha256:...", json: "<compact asset JSON>" };
```

- `revision` is computed at build time over the authored file text with `\r\n` normalised to `\n`, exactly what `revisionOfText` states, so exports and the CLI see the same value.
- `json` is `JSON.stringify(document.asset)`: no indentation, no `$schema` or `version` envelope. A string literal parsed with `JSON.parse` is the fastest way to materialise large data.
- The loader (`bundledCatalog`) globs with the literal query `?staffage-asset`, parses `payload.json` and takes `payload.revision`. The browser no longer hashes anything.
- The plugin rewrites the id to a virtual `\0staffage-asset:<file>.mjs` module so Vite's built-in JSON plugin never touches it.

## Wiring

- Site build and dev server: `staffageAssetVitePlugin()` in the site builder plugin list.
- Vitest of `@teaching/staffage-react`: the plugin in the test config (`bundledCatalog` is used by the catalog-loading test and the app mount tests).
- The site vitest config does not import the loader; it needs nothing.

## Test

`🎯️targets/⚛️react/🧪️tests/🗜️catalog-payload/🟦️.tsx` (scenarios in `🥒️.feature`): the served revision equals `revisionOfText` of the file text and equals an independent `node:crypto` sha256; the parsed `json` equals the file's `asset`, has no indentation and is shorter than the file; an `import.meta.glob` import through the real plugin yields the same payload; other ids are left to Vite. 4 of 4 pass.

`📚️catalog-loading` passes 7 of 8; the failing one (`loads the authored archive ...`) fails on `manifest-dimension-mismatch` for `standing-adult`, `pig` and others, i.e. the in-flight asset edits of another agent against the manifest, not on loading (every row loads).

## Sizes of `dist/release/assets` (release build)

| | files | raw bytes | gzip -9 bytes (per file, summed) |
|---|---|---|---|
| before | 159 | 68,585,509 | 7,334,217 |
| after | 160 | 38,224,167 | 6,916,582 |
| change | | -44.3 % | -5.7 % |

The 152 asset chunks are minified to a single quoted JSON string each. The gzip gain is modest because gzip already compressed indentation almost for free; the raw win is what a parse and memory budget sees.

## Notes

- Dev server reload: `load` registers the asset file with `this.addWatchFile`, so an edited asset reloads (covered by a test with a fake context).
- Gzip stays around 6.9 MB because the content (numbers and strings) dominates; further reduction would need a content change (for example shared geometry), not a transport change.
