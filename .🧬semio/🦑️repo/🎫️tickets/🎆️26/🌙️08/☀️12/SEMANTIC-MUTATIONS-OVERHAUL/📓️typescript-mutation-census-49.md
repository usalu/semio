# TypeScript Mutation Census 49

## Scope

This is a TypeScript AST candidate census, not a semantic-completeness or consumer-graph claim. Its controller is `🧪️typescript-mutation-census-49/📜️script.ts`. It reuses the live taxonomy loader for repository roots and verifies the central opaque taxonomy exclusions are exactly `compose/` and `temp/compose/` before any real traversal.

Every candidate path is rejected before filesystem access if it enters any ASCII-case-folded `compose` segment, exact ticket internals, dependency/cache/build/generated roots, or a symlink. Unlike the earlier Rust controller, it does not blanket-reject dot-prefixed segments: the virtual probe admits a hidden authored path, while exact `.git`, `.nx`, `.🧬semio`, and `.cache` internals remain excluded.

The real scope is taxonomy roots plus the established repository owners `✏️s`, `🧰️framework`, `🌎️hub`, and `♻️mit-bestand`; no Plugin-only selection is used. The controller accepts only authored `.ts`, `.tsx`, `.mts`, and `.cts` files, records every accepted file's SHA-256 and parser-diagnostic count, and runs in a worker with a 115-second deadline and 30-second progress-stall cancellation guard.

## Virtual Validation

The adjacent draft-2020 `🛂schema.json` validates the neutral `🧫️fixtures/🔣️probes.json`. Before any real filesystem walk, the controller uses the installed TypeScript compiler's `createSourceFile` AST on POSIX, Windows-separator, nested, and case-folded compose paths; a virtual symlink escape; ticket, dependency, generated, parent-escape, and hidden-authored paths; and direct discriminated-union, generic-contract, and malformed-TypeScript cases. All virtual probes passed in the retained full source run.

## Executed Result

The retained full source scan `🧪️typescript-mutation-census-49/🧫️run-zvsJF1` finished in 49,079ms, below the 115,000ms deadline, without cancellation and with stable controller/discovery/probe/schema hashes. It scanned 8,388 TypeScript files, recorded no real parse diagnostics, and retained exact per-file hashes and candidates in `🔣️inventory.json`.

| AST candidate kind | Count |
| --- | ---: |
| Discriminated union/type alias | 331 |
| Mutation-named enum | 0 |
| Mutation-named class | 2 |
| Mutation-named interface | 168 |
| Mutation-named function | 76 |
| Object discriminator | 4,403 |
| Total | 4,980 |

The object-discriminator set is deliberately broad and heuristic; it includes many JSON-schema/value objects. Every such record is marked `ast-heuristic` and must not be treated as a real mutation without dispatch review.

## Final Scoped Nx Replay

The final scoped launcher command completed successfully on 2026-08-27:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️typescript-mutation-census-49/📜️script.ts'
```

Its retained result is [run-scxu7f summary](./🧪️typescript-mutation-census-49/🧫️run-scxu7f/🔣️summary.json) and [inventory](./🧪️typescript-mutation-census-49/🧫️run-scxu7f/🔣️inventory.json). It completed in 18,877ms, with exit code 0, no cancellation, no worker error, and stable first/final hashes for all controller inputs.

| Measure | Current result |
| --- | ---: |
| TypeScript sources | 8,388 |
| Candidates | 4,983 |
| Parse diagnostics | 0 |
| Concrete-looking | 4,877 |
| Generic-contract | 5 |
| Test-fixture | 101 |

The current controller SHA-256 is `97d8199c38d4233b4fa71bc27e6ed5d610c2a577cabdc050abcc0b5d09d94805`. The discovery authority, virtual probes, and probe schema respectively remained `807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423`, `a56b7ad8d17973ed4124b82d83358f4c973ddf80e0d2c2dbbee5b3c260f90ed8`, and `069cf8f23fecc5a7c9e33002f8ce4cfbe0cfffac984f98be4a0387da258f8496`.

## Candidate Triage

The following highest-volume non-generated owners are the largest *unexplained candidate groups*, not claimed mutations. All are dominated by heuristic object discriminators and need semantic dispatch review before any owner action:

| Owner | Candidate count | Why unresolved |
| --- | ---: | --- |
| `renderer/engine/.../typescript/targets/react` | 444 | React target test and render value discriminators (`kind=load`, `type=componentScene`), not proven operations. |
| `framework/ui/.../typescript/targets/react` | 381 | UI script object metadata (`kind=raw-dom-primitive` etc.), not a mutation owner proof. |
| `renderer/engine/elements/ShellHost` | 321 | UI state/event object literals plus two heuristic mutation-named functions. |
| `cad/.../editor/engine/artifact` | 305 | Engine event/value `kind` objects, including `selection.changed`; dispatch has not been checked. |
| `infinite/world/r3f` | 279 | Render topology `type` values such as `face` and `corner`. |
| `framework/actor/typescript` | 170 | Mailbox status objects (`coalesced`, `rejected`, `dropped`). |
| `framework/os` | 169 | Worker status/event objects; no semantic mutation relationship established. |
| `renderer/engine/elements/UiDocumentStore` | 143 | UI event/value discriminators, not a direct mutation contract. |
| `repo/library/typescript` | 112 | Repository policy/normalization metadata and one candidate union; requires separate contract review. |
| `demonstrator` | 110 | Vite/static configuration object discriminators. |

The live inventory preserves exact paths, lines, source hashes, candidate classifications, and confidence labels for subsequent targeted triage. This census is explicitly not whole-language completion proof: it does not resolve cross-file symbols, dynamic property keys, runtime-created objects, JavaScript sources, or excluded generated/dependency/ticket content, and it does not establish concrete mutation semantics, dispatch, reachability, codecs, or descriptor compliance.

## Scope Repair — Current Red Evidence

The preceding replay is historical only. Independent review found that its four-root walker omitted repository-root TypeScript, blanket-skipped authored dotted metadata, skipped generic authored `targets`/`build`/`cache` names without taxonomy authority, and did not final-verify the source roster.

The controller now starts at the repository root, uses the discovery module's `loadTaxonomy()` and `taxonomyRelativePathIsExcluded(path, taxonomy)` as the opaque-path authority, and retains only exact repository internals (`.git`, `.nx`, `.venv`, `node_modules`, and `.🧬semio/🦑️repo/🎫️tickets`). It does not adopt the current shared `policyRepositoryOwnedRoots` / `policyFindAllMutationsDirs` behavior: their current source still hardcodes four roots and rejects every dotted component, so they are not adequate full-repository TypeScript scope evidence. Root owns their shared repair.

The controller now has schema-validated virtual probes for repository-root TypeScript, hidden authored metadata outside ticket internals, authored target/build/cache/generated-named directories, lexical case-folded compose rejection, a post-discovery symlink, and source-byte drift. It uses no-follow checks during discovery, then repeats the complete source path walk and rereads every recorded source file and every controller/taxonomy/probe input before producing a result.

Retained red runs are intentional evidence, not successful census claims:

- `🧫️run-ZJ2ZQ6`: rejected the existing non-TypeScript root symlink `CLAUDE.md`; this drove the correction to skip inert symlink entries while still rejecting a source symlink at guarded read.
- `🧫️run-RRiVDC`: rejected attempted traversal into `.venv/.../sklearn/compose`; this drove the exact `.venv` dependency exclusion before access.
- `🧫️run-Jvtwbe` and `🧫️run-m5qRcj`: retained bounded deadline failures while the full final source verification was being added and measured.
- `🧫️run-hLEcJh`: worker exit 0 and all authority inputs stable, but the final roster rejected a real concurrent mutation of `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts` from `8f89b2019030fe041e5ac2f9c71505f9b03d6ea2e3ed2d5fe920a5aa30150e64` to `676c22e5d777532356873a3c02a8c672462f84d7d61d8d88364de2130fa6a025`.

The current source roster therefore remains observed-only and rejected for drift. No stable complete TypeScript census is claimed; a future bounded replay must produce a matching final roster before this packet can be considered scope evidence.

## Limits

- TypeScript AST parsing identifies syntax only; no type program/checker or cross-file symbol resolution is constructed.
- Dynamic keys, runtime-created objects, JavaScript files, and excluded generated/dependency/ticket content remain outside the candidate result.
- Candidate names and discriminators do not prove reachability, direct-owner suitability, codec behavior, descriptor compliance, or semantic mutation coverage.
- No production source, launch/generator, Cargo, or compose path was touched.
