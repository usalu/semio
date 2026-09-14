# Report D — language-agnostic markdown test, oracle registry, launch entries

Agent D of the RESTORE-REVEAL-JS-PRESENTATION-PRODUCT-FRAMEWORK ticket. Scope: the language-neutral
test case for the owned slide markdown compiler, the presentation product's oracle contribution, the
oracle devDependencies of the React host package, and the `launch.json` / `🧩️launch.seed.jsonc`
registration of the new test targets.

## Files created

| Path | What it is |
| --- | --- |
| `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📝️markdown-html-compilation/🥒️.feature` | The normative, language-neutral contract: 7 scenarios, 30 markdown vectors in data tables |
| `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📝️markdown-html-compilation/🟦️.ts` | TypeScript adapter — `subject` = owned `compileOwnedMarkdownToHtml`, `oracle` = the remark stack |
| `🧰️framework/🛍️products/🎤️presentation/🧪️tests/📝️markdown-html-compilation/🧫️fixtures/*.md` | 30 immutable case-local markdown fixtures, one per vector |
| `🧰️framework/🛍️products/🎤️presentation/🔮️oracle/🔣️.json` | Oracle contribution manifest, `schemaVersion` 2, discovered by the harness at the taxonomy-declared contribution location |
| `🧰️framework/🛍️products/🎤️presentation/🔣️oracle.json` | Product-root reference summary, mirroring print's `schemaVersion` 1 file |

## Files changed

| Path | Change |
| --- | --- |
| `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | devDependencies `unified@11.0.5`, `remark-parse@11.0.0`, `remark-gfm@4.0.1`, `remark-rehype@11.1.2`, `rehype-stringify@10.0.1` (exact-pinned, as print pins its oracles) and `jsdom@^24.1.3` (the vitest config declares the jsdom environment and did not declare the package) |
| `.vscode/launch.json` | 7 new configurations |
| `.vscode/🧩️launch.seed.jsonc` | the same 7 configurations, byte-identical |

## The case

Owner is the product root `🧰️framework/🛍️products/🎤️presentation` — the compiler's behaviour is a
product-level contract, not a React package's. Discovered Nx project name:
`test-framework-products-presentation-e109a8-markdown-html-compilation`.

Tags: `@capability-presentation-markdown-html`, `@oracle-remark`, `@comparison-ordered-json-v1`.
Every scenario is `@mode-differential` and carries exactly one `@level-…`:

| Scenario | Level | Vectors |
| --- | --- | --- |
| `prose` | fundamental | paragraph, soft break, `h1`–`h6`, inline marks in a heading |
| `inline` | fundamental | `*`/`_` emphasis and strong, inline code, double-backtick code, `<`/`&` escaping, backslash escapes, hard break |
| `lists` | quick | `-`/`+`/`*` bullets, `1.`/`1)` ordered, `start` attribute, nesting, inline marks in items |
| `links` | quick | inline link, link title, inline marks in link text, `mailto:`, autolink |
| `code` | quick | fenced with language, fenced without, escaping inside a fence, `~~~` fence |
| `tables` | long | GFM table with per-column alignment, plain GFM table |
| `slide` | long | a whole slide body (heading + prose + list + fence) |

### Oracle

`unified` 11.0.5 composing `remark-parse` 11.0.0 + `remark-gfm` 4.0.1 + `remark-rehype` 11.1.2 +
`rehype-stringify` 10.0.1, all already present in the repository's `node_modules` and reached by the
TypeScript host from the repository root resolver. Registered as one entry `remark`, kind
`third-party-library`, ecosystem `javascript`, primary package `unified` with the other four as
`packages` (linked packages, each with its own version, license and role), `testOnly: true`,
`license: MIT`, `productionReachable: false`, `networkDuringExecution: false`,
`hostPath: 🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react`.

### Normalisation — a deliberate deviation from the brief

The brief asked for "collapse whitespace between tags, trim". That normalisation was implemented
first and then **rejected**, because it is lossy in exactly the place a markdown compiler can break:
`>\s+<` turns `<em>slide</em> <strong>title</strong>` into `<em>slide</em><strong>title</strong>`,
so a compiler that dropped the space between two inline marks would have been normalised into
agreement with the oracle. The first `prose` run reproduced precisely that — the recorded projection
read `<h2>A <em>slide</em><strong>title</strong></h2>`.

The shipped normalisation is therefore non-lossy: line endings to `\n`, then `trim()`. This is
sufficient because **all 30 vectors compile byte-identically** under both producers
(`🔧️markdown-oracle-exact.ts`: `exact=30 trim-only=0 diff=0`), so no structural allowance is needed
at all; the two rules that remain absorb only a CRLF checkout and a trailing newline. The feature
file states the refusal and its reason in prose, so the next reader does not "helpfully" restore it.

### Excluded constructs

The vectors are the subset the owned compiler implements. These constructs were measured and DIVERGE
from remark, and are deliberately absent from the feature (`🔧️markdown-oracle-excluded.ts`):

| Construct | Owned compiler | remark |
| --- | --- | --- |
| block quote | `<p>&#x3E; quoted line</p>` | `<blockquote><p>…</p></blockquote>` |
| thematic break | `<p>---</p>` | `<hr>` |
| image | `<p>!<a href="…">alt</a></p>` | `<p><img src="…" alt="alt"></p>` |
| setext heading | paragraph with the `=====` line | `<h1>` |
| strikethrough | `<p>~~gone~~</p>` | `<p><del>gone</del></p>` |
| task list | `<li>[ ] todo</li>` | `<li class="task-list-item"><input type="checkbox" disabled> todo</li>` |
| reference link | literal `[docs][d]` and a definition paragraph | resolved `<a href="…">docs</a>` |
| indented code block | paragraph with the leading spaces | `<pre><code>` |
| loose list | tight `<li>alpha</li>` | `<li><p>alpha</p></li>` |

Raw HTML blocks agree (both drop them). Fixing any of the divergences above by writing the owned
behaviour into the feature would freeze it as the contract, so the case states what the compiler
claims and the rest stays out until the compiler grows it.

## How the case was run

The harness runs product cases through the repository-root verb, which forwards to the test module's
own script (`bun nx run` was avoided throughout — the Nx project graph is broken by an unrelated
duplicate project). The `--case` selector matches the case DIRECTORY name including its emoji, so
`--project` is the reliable selector:

```bash
bun ./📜️script.ts test parity <fundamental|quick|long|exhaustive> \
  --project test-framework-products-presentation-e109a8-markdown-html-compilation
```

### Result — all green

```
--- parity fundamental --- [test] level=fundamental cases=1 executed=4  passed=4  failed=0 errored=0 parity=2/2
--- parity quick       --- [test] level=quick       cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
--- parity long        --- [test] level=long        cases=1 executed=14 passed=14 failed=0 errored=0 parity=7/7
--- parity exhaustive  --- [test] level=exhaustive  cases=1 executed=14 passed=14 failed=0 errored=0 parity=7/7
```

`oracle exhaustive` and `subject exhaustive` each execute 7 and pass 7 on their own.

### Negative control

Because a passing differential test proves nothing until it can fail, `paragraph.md` was temporarily
given a block quote — a measured divergence — and `parity fundamental` exited 1. The fixture was
restored byte-for-byte and the ladder re-run green. The comparison is real.

### Contract and dependency phases

`bun ./📜️script.ts test contract` and `… test dependency` are repository-wide, not case-scoped, and
both already fail on a large pre-existing backlog (1212 high-priority contract breaches, 159
dependency breaches, none of them in this ticket's area). The relevant fact is that **this case adds
none**: `grep -c "🛍️products/🎤️presentation"` over the contract breach set returns `0`, and
`dependency` classifies all five remark-stack packages as `test-oracle`, none production-reachable:

```
[dependency] test-oracle js:rehype-stringify@10.0.1 (remark)
[dependency] test-oracle js:remark-gfm@4.0.1 (remark)
[dependency] test-oracle js:remark-parse@11.0.0 (remark)
[dependency] test-oracle js:remark-rehype@11.1.2 (remark)
[dependency] test-oracle js:unified@11.0.5 (remark)
```

## Launch registration

Seven configurations inserted into BOTH `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`,
immediately after the `🧪️test🖨️print📊️viz-kernel🌌️exhaustive` entry, in the existing shape
(`type: node-terminal`, `request: launch`, `cwd: ${workspaceFolder}`, no `presentation` block):

| Name | Command |
| --- | --- |
| `🧪️test🎤️presentation⚡️quick` | `bun nx run @semio-tech/presentation:test-quick` |
| `🧪️test🎤️presentation🌕️long` | `bun nx run @semio-tech/presentation:test-long` |
| `🧪️test🎤️presentation🌌️exhaustive` | `bun nx run @semio-tech/presentation:test-exhaustive` |
| `🧪️test🎤️presentation⚛️react⚡️quick` | `bun nx run @semio-tech/presentation-react:test-quick` |
| `🧪️test🎤️presentation⚛️react🌕️long` | `bun nx run @semio-tech/presentation-react:test-long` |
| `🧪️test🎤️presentation⚛️react🌌️exhaustive` | `bun nx run @semio-tech/presentation-react:test-exhaustive` |
| `🧪️test📽️projektetage` | `bun nx run @semio-tech/mit-bestand-praesentation-projektetage:test` |

**No `🔰️fundamental` entry.** Neither `📋️project.json` defines `test-fundamental`; both define
`test`, `test-quick`, `test-long`, `test-exhaustive`. Print defines BOTH `test` and
`test-fundamental` and registers only the latter, so registering `🔰️fundamental` against `:test`
would create a name that does not match its target and would collide the day Agent A adds a real
`test-fundamental`. Open item for the product owner: either add `test-fundamental` to both
`📋️project.json` files and the two matching launch entries, or accept the three-rung ladder.

### Validation

`🔧️launch-validate.ts` strips comments and trailing commas and parses both files:

```
.vscode/launch.json:          OK, 899 configurations, duplicates=[]
.vscode/🧩️launch.seed.jsonc: OK, 833 configurations, duplicates=[]
```

The seed's 79 nameless entries are its pre-existing `@generated:…` string directives, untouched. A
`grep`-based diff confirms the print entries and the seven new entries are byte-identical between the
two files.

## Ticket scratch files

Kept (inputs, re-runnable from the repository root):

- `🔧️markdown-fixtures.ts` — writes the 30 fixtures
- `🔧️markdown-oracle-compare.ts` — owned vs. remark over the candidate vector set
- `🔧️markdown-oracle-exact.ts` — byte-exact agreement over the committed fixtures
- `🔧️markdown-oracle-excluded.ts` — the measured divergences of the unclaimed constructs
- `🔧️launch-insert.py` — idempotent launch insertion into both files
- `🔧️launch-validate.ts` — JSONC parse + duplicate-name check

Deleted: this agent's `🗑️generated/{contract,parity,ladder,dependency}.txt` and `probe.mjs`. The rest
of `🗑️generated/` belongs to other agents of this ticket and was left alone.
