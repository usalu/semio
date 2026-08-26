# H-FULL-COVERAGE — Taxonomy v7 Full Path Coverage Audit

## Snapshot and method

This is a read-only census of the current Git index plus current untracked paths. It excludes `compose` lexically in the Git pathspec before any filesystem inspection. It does not traverse `compose/`, follow symlinks, or inspect nested untracked fixture repositories. The latter enter the census as the single directory paths returned by the outer repository, matching the normalization mechanism's Git-index/active-ticket contract.

The stable pre-report snapshot is:

| Datum | Value |
| --- | ---: |
| HEAD | `a03e259755a2448dea999fc9e621139b5b480881` |
| taxonomy schema | v7 |
| taxonomy SHA-256 | `8895d61e73ad1912290d8221e0f4d0eef48f8fff866f15dda271b8b97accfe81` |
| taxonomy registries | 70 file kinds; 69 directory kinds; 17 fixed contracts; 4 configurable contracts |
| admitted-path SHA-256 | `0710bbd8bee51d622c40e9da7b7bf4557941f4f83eb2a8953bf6b468bc69b601` |
| existing admitted paths | 64,720 |
| files/symlinks | 64,705 |
| directories derived from admitted paths | 37,982 |
| current untracked outer-repository paths | 19 |

The requested report did not exist when that hash was taken and is intentionally not part of the snapshot. Every count below is tied to the two hashes above, so later concurrent schema/report edits cannot silently change its meaning.

Commands used:

```text
git ls-files --cached -z -- . :(exclude,top,literal)compose
git ls-files --others --exclude-standard -z -- . :(exclude,top,literal)compose
git rev-parse HEAD
shasum -a 256 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
```

The resolver simulation mirrors `resolveFileKind` and leading-emoji `matchDirectoryKind`: NFC basenames, longest extension chain, VS16-insensitive emoji comparison, exact slug regexes, and no invented fallback emoji. Directory parents are derived from admitted Git paths rather than a filesystem walk.

## A. Terminal extension and file-kind coverage

### Counts

| Resolution | Files | Share |
| --- | ---: | ---: |
| unique file-kind resolution | 54,550 | 84.30% |
| ambiguous longest chain | 9,895 | 15.29% |
| no registered chain | 260 | 0.40% |
| total | 64,705 | 100% |

Removing any basename already named by a fixed/configurable contract—an intentionally generous basename-only upper bound, not a scope match—leaves 9,705 ambiguous files and 252 files with no chain.

All ambiguity is concentrated in three chains:

| Chain | Candidate kinds | Total | historical top-level ticket | elsewhere |
| --- | --- | ---: | ---: | ---: |
| `.json` | `json`, `manifest-json`, `project-json`, `test-output-marker` | 4,795 | 4,150 | 645 |
| `.md` | `empty-marker`, `markdown` | 4,706 | 4,243 | 463 |
| `.rs` | `rust-binary`, `rust-package`, `rust-source`, `rust-test`, `tui-source`, `wgpu-source` | 204 | 182 | 22 |

The main exact precedents inside those ambiguous sets are:

- `🎫️ticket.json`: 3,348 historical ticket records.
- `📓️*.md`: 1,699 historical/active ticket reports.
- unprefixed Markdown: 2,668 total, including 547 outside top-level historical tickets.
- unprefixed JSON: 959 total, including 394 elsewhere.
- `📋️*.md`: 190 ticket handoff/registrar documents.
- unprefixed Rust: 190 total, including 16 elsewhere.
- `🧪*.md`: 189 ticket evidence documents.
- `🦠️*.json`: 167 production fixture mutation records.
- `🧪*.json`: 162 ticket evidence records across VS16 variants.
- `🎯️goal.json`: 58 goal records.
- `📜️artifact-definition.json`: 36 production schema records.
- `⬅️before.json`: 27 production comparison preimages.
- `➡️after.json`: 27 production comparison postimages.
- `🔒️*.json`: 4 policy/generated registries.
- Production Rust ambiguity includes `⏳️{imports,runtime}.rs`, `🎠️{runtime,activation}.rs`, `🏃️executor.rs`, and `🛡️hostile-batch-edge.rs`.

### No-chain suffix census

The actionable/mixed suffixes are:

| Suffix | Total | historical top-level ticket | elsewhere | Disposition |
| --- | ---: | ---: | ---: | --- |
| `.TAG` | 53 | 47 | 6 | All are `CACHEDIR.TAG`; the six “elsewhere” paths are nested ticket roots inside product package trees. Treat as misplaced build evidence, not a production kind. |
| `.shields` | 19 | 0 | 19 | Add a badge-definition asset kind using the observed `🛡️` prefix. |
| no dot | 17 | 7 | 10 | Exact-contract candidates; see section D. |
| `.dsl` | 6 | 1 | 5 | Add an observed `🗣️` textual DSL data/spec kind. |
| `.spk` | 6 | 1 | 5 | Add an observed `📦️` binary pack asset kind. |
| `.gh` | 3 | 0 | 3 | Add a Grasshopper asset kind using observed `🦗️`. |
| `.zz` | 3 | 0 | 3 | Add a deflate fixture asset kind using observed `🗜️`. |
| `.bcf` | 2 | 0 | 2 | Add a BCF coordination asset kind using observed `💬️`. |
| `.sum` | 2 | 0 | 2 | Both are `go.work.sum`; add an exact Go workspace checksum contract. |
| `.cff` | 1 | 0 | 1 | Add exact root `CITATION.cff`. |
| `.collection` | 1 | 0 | 1 | Add the observed collection document kind. |
| `.db` | 1 | 0 | 1 | `hub/.semio/.../directory.db`; generated local state should be removed from versioned scope, not normalized. |
| `.desktop` | 1 | 0 | 1 | Add exact Linux desktop-integration contract/kind. |
| `.dockerignore` | 1 | 0 | 1 | Exact tool filename contract. |
| `.example` | 1 | 0 | 1 | Exact `.env.example` contract. |
| `.gitattributes` | 1 | 0 | 1 | Exact repository-wide Git contract. |
| `.gitignore` | 1 | 0 | 1 | Exact repository-wide Git contract. |
| `.gitmodules` | 1 | 0 | 1 | Exact repository-root Git contract. |
| `.kiro` | 1 | 0 | 1 | Exact Kiro configuration contract. |
| `.nojekyll` | 1 | 0 | 1 | Exact static-site marker contract. |
| `.nxignore` | 1 | 0 | 1 | Exact Nx contract. |
| `.plist` | 1 | 0 | 1 | Add Apple property-list configuration kind. |
| `.prettierignore` | 1 | 0 | 1 | Exact Prettier contract. |
| `.rhl` | 1 | 0 | 1 | Domain data; owner must confirm semantic kind before registry addition. |
| `.space` | 1 | 0 | 1 | Add observed Semio space data kind only after its owner confirms authority. |
| `.vscodeignore` | 1 | 0 | 1 | Exact VS Code packaging contract. |

Ticket-only suffixes account for the remaining 152 no-chain instances and must not be globally blessed as production formats:

```text
.pid 36; .fragment 11; .orig 11; .tpl 10; .cpuprofile 6;
.register-toc 5; .sctoc 5; .head 4;
.after 2; .after-edit 2; .before 2; .done 2; .final 2;
.orig-backup 2; .ready 2; .recovered 2; .rmeta 2; .work 2;
.before-dedup 1; .before-edit 1; .before-repair 1; .before-stub 1;
.broken_backup 1; .gitkeep 1; .partial 1; .patched-root 1;
.pre-edit-backup 1; .pre-normalize-backup 1; .pre-overlay-backup 1;
.recreated-1308 1; .S 1; .snapshot 1; .template 1; .window-breaks 1.
```

These need an owner-scoped `ticket-evidence` file-kind mechanism or a declared historical-ticket evidence contract. Adding those extension chains globally would make backup/build residue valid in production areas.

### File-kind additions and resolver changes

Safe additions supported directly by observed emoji precedents are:

| Proposed ID | Emoji | Chain | Role |
| --- | --- | --- | --- |
| `shields-badge` | `🛡️` | `.shields` | asset/configuration |
| `dsl-data` | `🗣️` | `.dsl` | specification/asset |
| `semio-pack` | `📦️` | `.spk` | asset |
| `grasshopper` | `🦗️` | `.gh` | asset |
| `deflate-data` | `🗜️` | `.zz` | asset |
| `bcf-data` | `💬️` | `.bcf` | asset |

The ambiguous chains need context, not more global extension owners:

1. Add prefix-specific kinds/contracts for `🎫️ticket.json`, `🎯️goal.json`, `🦠️*.json`, `📜️artifact-definition.json`, `⬅️before.json`, `➡️after.json`, `🔒️*.json`, `📓️*.md`, `📋️*.md`, and ticket `🧪*.{md,json}`.
2. Make `markdown` the default `.md` kind and reserve `empty-marker` for exact/prefix `📌️empty.md`. Today any other Markdown prefix is ambiguous even though it cannot semantically be an empty marker.
3. Resolve `.rs` by package/target/parent/configuration context: configured `📦️.rs`/`🚀️.rs`, test parents, WGPU/TUI target parents, then default `rust-source`. Semantic stems such as `⏳️imports` must not require invented Rust-role emojis.
4. Do not default all JSON to one role. Tool exact names, ticket/goal records, fixture mutations, generated registries, and schema JSON require distinct prefix/path contracts.

## B. Existing emoji-leading directory registry coverage

### Counts

| Directory class | Count |
| --- | ---: |
| derived directories | 37,982 |
| emoji-leading | 32,056 |
| uniquely registered | 22,104 |
| ambiguous | 0 |
| unregistered | 9,952 |
| distinct unregistered basenames | 2,924 |
| unprefixed | 5,926 |

The leading unregistered exact basenames are:

| Name | Count | Proposed treatment |
| --- | ---: | --- |
| `⬅️before` | 1,559 | global comparison-preimage kind |
| `➡️after` | 1,558 | global comparison-postimage kind |
| `🎯️outcome` | 1,558 | global comparison-outcome kind |
| `🪟️main` | 178 | contextual window-item kind, excluding `windows` |
| `👁️view` | 147 | exact viewer-mode kind |
| `✏️edit` | 138 | exact editor-mode kind |
| `📄txt` | 89 | artifact-format identity |
| `🔣️json` | 85 | artifact-format identity |
| `🎬️demo` | 77 | example identity/manifest member |
| `🎬️demo-session` | 53 | example identity/manifest member |
| `📷️png` | 51 | artifact-format identity |
| `📄️artifact` | 49 | owner-local semantic member |
| `📄set-snapshot` | 48 | owner-local command/mutation member |
| `🔍️inspection` | 44 | owner-local semantic member |
| `🖊️dwg` | 37 | artifact-format identity |
| `📊️csv` | 33 | artifact-format identity |
| `🎒️zip` | 31 | artifact-format identity |
| `🧾outline` | 31 | owner-local semantic member |
| `📄️pdf` | 29 | artifact-format identity |
| `📝️md` | 29 | artifact-format identity |
| `🧊️obj` | 29 | artifact-format identity |
| `🟪️stl` | 29 | artifact-format identity |
| `🎨️svg` | 27 | artifact-format identity |
| `🛍️catalogue` | 23 | owner-local semantic member |
| `📦bounds` | 22 | owner-local semantic member |
| `📚️catalogue` | 19 | owner-local semantic member |
| `🧊️gltf` | 19 | artifact-format identity |
| `🧭topology` | 19 | owner-local semantic member |
| `📊️results` | 18 | owner-local output member |
| `📊️report` | 17 | owner-local output member |
| `🖊️dxf` | 17 | artifact-format identity |

Ticket hierarchy is also absent from the registry: observed `🎆️YY`, `🌙️MM`, and `☀️DD` directories need explicit ticket-year/month/day kinds; `🎫️tickets`, `🎯️goals`, `🧑️‍💻️devs`, and `💬️prompts` need structural registry entries. Day directories alone account for 261 unresolved instances.

### Registry design conclusion

Adding 2,924 global exact names is not a maintainable schema. The gaps divide cleanly:

- Add global structural kinds for ticket/goal/dev/prompt hierarchy and the high-frequency exact comparison/window/mode identities above.
- Add exact artifact-format identities for the stable known-format catalog (`txt`, `json`, `png`, `dwg`, `csv`, `zip`, `pdf`, `md`, `obj`, `stl`, `svg`, `gltf`, `dxf`, and the remaining registered stdio formats).
- Load owner-local exact semantic members from their existing `x-semio` manifests/generated registries for plugin, module, artifact, mutation, inference, command, example, asset, and output collections.
- Fail closed when an owner-local member is absent from its manifest. Do not add an “any emoji + any slug” global wildcard; it would erase the registry guarantee.

Current `parentKindIds` cannot by itself solve this: leading-emoji `matchDirectoryKind` does not filter matches by parent context. New broad entries must either have disjoint exact slug patterns or the resolver must apply parent constraints to emoji-leading names before such contextual kinds land.

## C. Unprefixed directory stems with an unambiguous existing emoji precedent

Method: strip only the first emoji grapheme from every existing emoji-leading directory, NFC/case-fold the remaining stem, and match unprefixed basenames. A candidate is admitted only when every observed precedent for that stem has the same VS16-folded emoji. This identifies 74 directory instances across 58 stems: 29 in top-level historical tickets and 45 elsewhere.

The complete stem census is:

| Canonical precedent | Instances | Historical ticket | Elsewhere |
| --- | ---: | ---: | ---: |
| `🧪️scratch` | 11 | 11 | 0 |
| `🧰️framework` | 3 | 1 | 2 |
| `🎨️styling` | 2 | 0 | 2 |
| `💻️os` | 2 | 0 | 2 |
| `🔺️diff` | 2 | 1 | 1 |
| `🦑️repo` | 2 | 1 | 1 |
| `#⃣hash`, `🌐️harness`, `🌐️html`, `🌦️epw`, `🌿️vcs`, `🎥️mp4`, `🎵️mp3`, `📄️pdf-render`, `📑️tsv`, `📸️snapshot`, `📼️avi`, `🔄️sync`, `🔊️wav`, `🫀️core` | 1 each | 1 each | 0 |
| `◻2d`, `♾️infinite`, `⚙️settings`, `⚪️compactness`, `✂️delete-primitive`, `🌎️hub`, `🌳️reparent-node`, `🎨create-material`, `🎫️handles`, `🏛️architect`, `📕️norm`, `🔀reorder-nodes`, `🔄️transform-node`, `🔖️v1`, `🔗️bind-node-mesh`, `🔗️bind-primitive-material`, `🔗️graphql`, `🔤️anta`, `🔤change-node-name`, `🔤️kelly-slab`, `🔤️share-tech-mono`, `🔺create-primitive`, `🕳️delete-texture`, `🕸️create-mesh`, `🖐️5d`, `🖱️ui`, `🖼️create-texture`, `🗂️catalog`, `🧊️3d`, `🧠️interpreter`, `🧩️puzzle`, `🪝️hooks`, `🪟️windows`, `🪧️logo`, `😀️noto-emoji`, `🚮delete-material`, `🛢️db` | 1 each | 0 | 1 each |
| `⬅️before` | 1 | 1 | 0 |

These are rename candidates supported by repository precedent, not automatic decisions. Stem identity alone can still be semantically misleading—for example `handles` may mean UI handles rather than ticket handles—so parent ownership and local manifest evidence remain required. The strongest direct precedents are `styling`, `framework`, `os`, `repo`, `hub`, `graphql`, `ui`, `windows`, `3d`, and the GLTF mutation stems that already coexist in both prefixed and unprefixed form.

## D. Exact no-extension and fixed-name candidates

### Current exact-contract scope misses

The engine currently matches only `repository-root`, package-root, or directory-kind scope. It does not consult `repoWideContractIds`, even though taxonomy v7 declares that list. Consequently the following observed exact names remain unprotected outside their narrow scope:

| Exact basename | Observed | Matched by any current scope | Unmatched |
| --- | ---: | ---: | ---: |
| `Cargo.toml` | 191 | 121 | 70 |
| `package.json` | 80 | 66 | 14 |
| `tsconfig.json` | 17 | 13 | 4 |
| `📋️project.json` | 180 | 3 | 177 |
| `pyproject.toml` | 2 | 1 | 1 |
| `go.mod` | 6 | 1 | 5 |
| `go.sum` | 1 | 0 | 1 |
| `AGENTS.md` | 50 | 1 | 49 |
| `README.md` | 33 | 1 | 32 |
| `LICENSE.md` | 9 | 1 | 8 |
| `📜️script.ts` | 183 | 1 | 182 |
| `nx.json` | 1 | 1 | 0 |
| `go.work` | 1 | 1 | 0 |

`repoWideContractIds` already names `root-script`, `root-project`, `root-nx`, `root-package`, `root-cargo`, and `root-go-work`; ignoring it is therefore a normalizer gap, not missing schema data. Honor that registry before adding duplicate contracts, and deduplicate the overlapping root/package contracts by contract identity and filename to avoid false ambiguity.

Add repository-wide or declared project-root scope for nested `AGENTS.md`, conventional `README.md`/`LICENSE.md`, `tsconfig.json`, Go module/workspace files, and Python project manifests. `📋️project.json` and `📜️script.ts` are especially important because AGENTS/Nx make those exact names authoritative throughout the workspace.

### No-extension candidates

All 17 no-dot files are exact-name cases; none should become a global extension kind:

```text
.devcontainer/Dockerfile
.🧬semio/🦑️repo/compose-micro-commit-bun
♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🌐️CNAME
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🌐️Caddyfile
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🐳️Dockerfile
🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-checkout
🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-commit
🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-merge
🧰️framework/🛍️products/🦑️repo/🪝️hooks/post-rewrite
🧰️framework/🛍️products/🦑️repo/🪝️hooks/prepare-commit-msg
```

The other seven are historical ticket evidence: four `*undefined` print outputs, a single-space filename, `🎯️pdf-deflate-blocks`, and `probe-atd-bin`. Preserve them as ticket evidence or explicitly clean/reclassify them; do not add semantic production contracts for those names.

Proposed exact contracts:

- `Dockerfile` with devcontainer/container-build scope.
- `Caddyfile` with coordinator/deployment scope.
- `CNAME` with static-site public-output scope.
- The five Git hook basenames with the existing `🪝️hooks` directory scope.
- `go.work.sum`, `CITATION.cff`, `.gitattributes`, `.gitignore`, `.gitmodules`, `.nxignore`, `.prettierignore`, `.vscodeignore`, `.dockerignore`, `.nojekyll`, and `.env.example` at their authoritative scopes.

## Risks and acceptance checks

1. Re-run the same census against an immutable taxonomy hash after every registry patch; concurrent edits changed v7 materially during this audit.
2. Require zero no-chain files outside declared ticket-evidence scope and zero ambiguous `.json`/`.md`/`.rs` after context/exact-contract resolution.
3. Require every emoji-leading directory to resolve through either a global structural kind or an exact owner-local manifest member; no wildcard fallback.
4. Require `repoWideContractIds` to be exercised by tests with nested `📜️script.ts`, `📋️project.json`, `package.json`, and `Cargo.toml` paths.
5. Add tests proving parent constraints apply to emoji-leading contextual kinds before introducing window/mode/example member patterns.
6. Re-run the unprefixed-precedent census and require every accepted rename to cite its precedent and parent owner; unresolved/semantically doubtful stems remain violations.
7. Keep `compose` lexical exclusion as the first path operation and continue treating symlink identity without target traversal.

## Conclusion

Taxonomy v7 now covers almost every observed terminal chain, but it does not yet produce a deterministic full-repository plan: 9,895 files remain role-ambiguous, 260 have no chain, 9,952 emoji-leading directories have no registered identity, and exact repo-wide contracts are declared but not honored. The bounded additions above address stable global structure and known formats; owner-local manifest overlays and exact-contract scope enforcement are required to close the rest without inventing semantic emoji or weakening fail-closed purity.
