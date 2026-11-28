# H63: coordinator `repo typecheck` outage (node_modules/typescript missing)

## Cause (evidence)
- Not a scope-selection bug (H8/H23/H26 not implicated): `.cache/b/d` (whole dependencies image: fixed.dmg, image.json, mount) was GONE on the coordinator; `.cache/b` mtime 09:47; `repo doctor` said "dependency image not attached or never created"; `node_modules` (05:42 link to `.cache/b/d/m/j/98bc.../node_modules`) and the repo product link dangled. No `d` mount, no `hdiutil` image path for it.
- Who deleted it: NOT proven. No retirement record, quarantine (`d.retired`) or journal survived. Strongest lead: an uncommitted, in-progress edit (09:38-09:45, not a wave lane) of the whole-partition retirement code (`💾️storage/🧹️whole`, `🍎️image/🧩️shard`, `🧹️cleanup/🎛️command/🐚️shell`: "live shard must be retired before its partition") ran against the real checkout at about 09:47. Workers still have `d` (m14 only has `d-stranded-*` quarantine copies from the stranded-root path, which only renames).

## Restore
`repo dependencies ensure` (MCP, 1248 s): new `d` image born, generations adopted from the fleet donor for all 6 scopes (`.`, ♻️mit-bestand, ✏️s, 🌎️hub, 🎓️teaching, 🏢️semio-tech), root link `.../98bcd02f92e33a230c9a` (same generation id as before), python env rebuilt (uv downloaded wheels, no fleet seed). `node_modules/typescript/lib/tsc.js` present, `repo typecheck` passes.

## Guard (landed, non-graph)
`requireNativeImageUnlinkedV1(directory)` in `💾️storage/🌐️platform/📋️contract/🟦️.ts`: for `<checkout>/.cache/b/<p>` refuses retirement while the checkout's root `node_modules` symlink resolves into the image and the target exists. Called in both retirement entry points next to the publication-settled check: `🍎️image/🟦️.ts` `retireNativeImageStorageV1` and `🧱️fixed/🟦️.ts` `retireNativeFixedPlatformV1`. Test in `🌐️platform/📋️contract/🧪️tests/🟦️.ts` (1 pass: absent link admitted, live link refused, other partition admitted, dangling link admitted). Typecheck 4 files 0 errors.

## Unverified / open
- The deleter is unknown; if it was a raw `rm -rf` (not the retire entry points) the guard does not cover it. Ask the owner of the 09:38-09:45 storage edits; their tests must use a tmp checkout.
- Missing tool self-healing ("typed install instead of failing") NOT implemented: tsc-missing message stays; would be a separate change in typecheck.
- Workers not checked for the same loss beyond m14 listing (they keep `d`).
- Product link `🧰️framework/🛍️products/🦑️repo/node_modules` is valid again (w/125).
