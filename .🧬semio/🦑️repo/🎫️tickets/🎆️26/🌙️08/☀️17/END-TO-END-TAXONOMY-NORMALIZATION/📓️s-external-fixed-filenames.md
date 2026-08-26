# External Fixed Filenames

## Outcome

The three externally owned fixed filenames are canonical and every live exact non-Compose code, test, and documentation reference was updated:

| Authority | Source | Canonical destination | Final SHA-256 |
|---|---|---|---|
| GitHub Pages | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🌐️CNAME` | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/CNAME` | `71cb3dd49adba785c06944735fa2da8b639aca8e08b41440d2aa3c1785bc1adc` |
| Caddy | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🌐️Caddyfile` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/Caddyfile` | `0d7ff115c524a84b11b2a39a7a3b98a975270dfe45840de894ba47efb5be9476` |
| Docker | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🐳️Dockerfile` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/Dockerfile` | `dcffa4e0da323f0c56d29b6ba43298c5a1021550cbfcebba756dec21fde56139` |

The CNAME and Caddyfile bytes were preserved exactly. The Dockerfile hash changed only because its internal header URI and description now name `Dockerfile` instead of the obsolete emoji-prefixed basename.

The shared Vite static-deploy writer now emits `dist/CNAME`; both its public types and manifest mirror document the same filename. The Go file-kind test now exercises canonical `Dockerfile`. The completed demonstrator plan now names only `CNAME`.

## TDD and permanent authority

The portable language-neutral ledger is `🧪️external-fixed-filenames/🔣️.json`; its Bun test derives the repository root from `import.meta.url`. It validates strict taxonomy v7, exact fixed-contract IDs and authorities, destination hashes, absence of all three source names, exact `fast-glob` discovery, and `picomatch` parity with the repository matcher.

The failing-first run produced one pass and two failures: no canonical destinations existed and the first live emoji-prefixed reference remained. After the atomic path/reference update:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️external-fixed-filenames.test.ts'
3 pass, 0 fail, 30 expect() calls, 304 ms
```

Owner-package gates:

```text
NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun ./node_modules/nx/bin/nx.js run @semio-tech/repo-client:test --skip-nx-cache -- -run=^TestDeriveFileKind$
Successfully ran @semio-tech/repo-client:test; Go test ok (cached)

NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun ./node_modules/nx/bin/nx.js run @semio-tech/ui-styling:test --skip-nx-cache
Successfully ran @semio-tech/ui-styling:test; 30 pass, 0 fail, 129 expect() calls
```

The direct uncached Go filter also passed:

```text
bun ./📜️script.ts test -run '^TestDeriveFileKind$'
ok github.com/usalu/semio/repo/client 0.586s
```

An earlier invocation using `-- -run` did not filter and exposed unrelated pre-existing full-suite failures; the correct routed argument form above is the acceptance command.

`git diff --check` over the bounded paths returned no output. An exact excluded-prefix-safe scan returned no live `🌐️CNAME`, `🌐️Caddyfile`, or `🐳️Dockerfile` reference outside retained evidence. No Git command changed state and neither excluded Compose prefix was accessed.

## Residuals

- Caddy is not installed in this environment, so `caddy validate` could not run.
- Docker CLI is installed, but `docker build --check -f Dockerfile .` could not connect to `/Users/ueli/.docker/run/docker.sock`; no image/build state was created.
- Historical ticket artifacts and the obsolete `.cursor/plans/emoji_unlock_and_repo_11ab8751.plan.md` retain their original strings as historical evidence. They are not live consumers.

## Independent Root Verification

The root lane reran the portable authority test after the packet completed: `3 pass`, `0 fail`, `30 expect()` calls. It also reran the exact Go owner selector through `⌨️cli/📦️packages/🟦️typescript/📜️script.ts`; the package returned `ok` from its cache.

An initial direct command incorrectly targeted a nonexistent `⌨️cli/📜️script.ts` and failed at module routing before tests. The corrected package-owned script path above is the recorded owner evidence; the routing failure is not a test result.
