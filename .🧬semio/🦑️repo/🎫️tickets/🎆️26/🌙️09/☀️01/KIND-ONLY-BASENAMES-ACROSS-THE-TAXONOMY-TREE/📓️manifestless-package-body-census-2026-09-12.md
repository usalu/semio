# Manifestless Package Body Census

A coordinator no-follow filesystem probe inspected four active scopes (framework, s, hub and Mit-Bestand), 158,742 entries in 4.968 seconds. It found 11 language package roots without the selected common native manifest. This is a bounded probe: caches, outputs, dependency/vendor trees, temp intake and ticket state were skipped as listed in the retained JSON; it does not establish a full-tree exception policy. A missing package.json is not automatically an error for a native JavaScript/tool scope.

The actual discovery gate has a concrete blind spot: `scanPackagesDir` in library discovery calls `collectPackageRoles` if the ecosystem has no manifest filename, or if the selected manifest exists. A non-target package root with a missing selected manifest falls through without body inspection. Thus the earlier 207 package-body findings were not a complete inventory of substantive bodies under language folders. The policy executor has the paired manifest-present/absent regression and correction assigned. It must inspect source independently of metadata existence without inventing a package.json requirement for every language root or changing catalog read-only behavior.

Recursive source classification of these 11 roots found the following 11 non-script/non-configuration leaves. Nine are implementation, one unresolved, and one a declaration. Four implementation cases were already queued (Norm vocabulary and three Flow leaves); five implementations and one unresolved source are additional closure work. Classifier evidence is not an independent syntax oracle and does not establish that the unresolved 8-line import rewrite source is invalid syntax.

| Source | Lines | Shared Classifier Role | Evidence |
| --- | --- | --- | --- |
| `✏️s/🔌️plugins/📕️norm/🔮️oracles/📦️packages/🐍️python/🗣️vocabulary.py` | 875 | implementation | Python function or class declaration |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` | 287 | implementation | Python function or class declaration |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/📦️packages/🟦️typescript/🟦️.ts` | 915 | implementation | runtime type or namespace declaration |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` | 1254 | implementation | domain type declaration is owned inside the package |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📦️distribution/🟦️.ts` | 67 | implementation | domain type declaration is owned inside the package |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📦️distribution/⚡️vite/🟦️.ts` | 18 | implementation | domain type declaration is owned inside the package |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🕸️imports/🟦️.ts` | 8 | unresolved | unbalanced lexical structure |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📝️flow-browser.d.ts` | 206 | implementation | runtime type or namespace declaration |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js` | 156 | implementation | runtime type or namespace declaration |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js` | 706 | implementation | runtime type or namespace declaration |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🟨️.js` | 2 | declaration | import, export, or type wiring only |

Disposable exact observations: `🗑️generated/coordinator/manifestless-package-roots.json` and `manifestless-package-body-decisions.json`. The latter is recursive; the initial direct-child-only probe missed three OS plugin support sources and is superseded. Ordinary source bodies are delegated in `📓️manifestless-source-closure-packet-2026-09-12.md`; named Norm/Flow owners remain in their existing plugin/metadata packets.

The coordinator also inspected the required Next health route: the shared classifier calls it thin-delegation (GET only constructs an imported NextResponse with constant status). It is distinct from the omitted substantive server library. That server library is 915 lines of database, authentication, event, queue and source-parsing behavior with no package.json. Current coordinator tsconfig maps `@/lib` to absent `../lib/index.ts`, while its route imports request exactly this server library API. A Bun bundle that externalizes aliases cannot verify that runtime edge. The source closure lane must repair the actual referent and validate it with the native resolver.
