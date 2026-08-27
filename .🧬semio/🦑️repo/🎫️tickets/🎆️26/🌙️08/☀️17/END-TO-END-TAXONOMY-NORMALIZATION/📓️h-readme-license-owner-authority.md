# Nested README and LICENSE Owner Authority

## Result

The current Git-admitted non-Compose cohort remains exactly 40 regular leaves: 32 `README.md` and eight `LICENSE.md`. All 40 are mode `0644`; every preimage SHA-256 and byte size is frozen in the language-neutral golden.

The owner split is exact:

| Disposition | Count | Authority |
| --- | ---: | --- |
| Preserve fixed basename | 4 | One publishable React-package `README.md` plus three publishable Bun-package `LICENSE.md` leaves |
| Exact owner-documentation projection | 28 | Configurable owner docs |
| Exact third-party-attribution projection | 4 | CC BY-ND attribution owned by an asset subtree, not a package publisher |
| Exact configurable owner-license projection | 1 | Go-module license with no demonstrated local exact-basename publisher/scanner contract |
| Exact ticket evidence/scratch projection | 3 | Two historical ticket docs and one CommonMark scratch input |

No repository-wide README/LICENSE blanket is proposed. Each configurable source has one exact destination. README semantics move to `📃️readme/📝️.md`, using the repo CLI's existing `EmojiFileDocs = 📃️` and `README.md -> docs/readme` precedent. License semantics move to `⚖️license/📝️.md`, using its existing `EmojiFileLicense = ⚖️` precedent. The physical leaf is the taxonomy's Markdown leaf `📝️.md` in both cases.

All 36 projected destinations are absent, byte/NFC/case-fold/VS16-fold/same-kind collision-free, mutually unique, and below the 240-byte path budget. The longest destination is 144 UTF-8 bytes. The four fixed paths are preserved in place.

## Fixed publication contracts

`bun pm pack --dry-run --ignore-scripts` independently includes these exact leaves:

1. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/README.md`
2. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/LICENSE.md`
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/LICENSE.md`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/LICENSE.md`

Each is adjacent to a non-private package manifest. The private `@semio-tech/repo-lib` manifest is explicit counter-evidence: its README is configurable and receives an exact owner projection, even though a local dry-run pack can enumerate it.

The VS Code package's `.vscodeignore` independently excludes `README.md` and re-includes `LICENSE.md`. Its manifest now separates the valid unscoped VSCE/package identity `repo-vscode` from the existing Nx project identity `@semio-tech/repo-vscode` through `nx.name`. The Nx build emits `out/extension.js`; `vsce ls --no-dependencies` and `vsce package --no-dependencies` both succeed with exactly `LICENSE.md`, `out/extension.js`, `package.json`, and `🖼️icon/🔣️.svg` as extension content. This independently confirms the package-root license selection without broadening the fixed LICENSE contract.

## Reference and generator owners

- Repo CLI Go dev-doc discovery/configuration owns the hardcoded technology/bundle `README.md` locations and must consume the projected exact owner paths.
- The CommonMark ticket scratch reader owns its `read_to_string("README.md")` input and must change with its projected path.
- Markdown relative-reference rewriting owns links whose base directory changes by one semantic segment.
- `assets-build` owns `🧰️framework/🔨️modules/🖼️assets/README.md`; its registered output must change to `🧰️framework/🔨️modules/🖼️assets/📃️readme/📝️.md` before regeneration.
- Asset distribution owners retain the four attribution texts after their exact semantic relocation.

The golden records the exact owner evidence, disposition, destination or fixed contract, reference and generator owners, preimage, path budget, and collision result for every source.

## Artifacts

- Golden: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️readme-license-owner-authority/🔣️.json`
- Portable test: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-license-owner-authority.test.ts`
- Golden SHA-256: `051394741822e92d51f3bda15ce64d84c236582c6927335c9c5e0ac3c18a1da4`
- NFC, UTF-8-byte-sorted, NUL-delimited source ledger SHA-256: `b4e4d352b041496f6e252ef3fbf4a8b5fb2009152a558e8b5ad99fe8999b896c`

## Verification

```text
git ls-files --cached --others --exclude-standard -- \
  .devcontainer .🧬semio ✏️s 🧰️framework
```

Filtering only exact basename `README.md` or `LICENSE.md`: 40 paths.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-license-owner-authority.test.ts'
```

Final combined rerun after the complete VSCE build/package repair: the authority file passed four tests with zero failures and 615 expectations; together with the companion publisher test the command passed ten tests and 652 expectations in 14.40 seconds. The authority checks the admitted Git ledger, literal-path `fast-glob` parity, every preimage, all collision folds, path budgets, package manifests, three Bun publisher dry runs, `.vscodeignore` semantics through the independent `ignore` package, repo CLI emoji/reference precedents, the CommonMark Rust reader, and the registered `assets-build` generator output.

The root coordinator independently reran the same exact command after the authority packet was handed off: four passed, zero failed, 615 expectations, 7.29 seconds reported by Bun (7.18 seconds process wall time).

## Remaining production work

1. Add only the exact four fixed package contracts and the exact enumerated owner projection contracts; do not broaden root README/LICENSE scope.
2. Update the repo CLI Go dev-doc owner, CommonMark scratch reader, Markdown references, and `assets-build` output before moving any source.
3. Apply all 36 moves through the signed transaction plan with the frozen preimages and rerun the publisher/generator/reference gates.
4. No VS Code publisher repair remains: manifest identity, build output, package listing, and VSIX creation are complete.

No taxonomy, discovery, normalization, shared test, production document, physical path, or Git state was changed. Actual `compose/**`, `temp/compose/**`, and `temp-compose/**` were never enumerated, traversed, or read; every census and filesystem operation used the four explicit non-Compose roots or the 40 exact golden paths.
