# 📓️ Final verification

Every number here was produced by running the command named beside it, at the end of the ticket.

## The platform builds and typechecks

| Surface | Command | Result |
| --- | --- | --- |
| TS core library + host | `tsc -p …/🟦️typescript/tsconfig.json` | clean |
| TS command router | `tsc` on `📜️script.ts` | clean |
| Rust protocol + runner | `cargo build --offline` | clean |
| Go host | `GOWORK=off go build .` | clean |
| .NET host | `dotnet build` | 0 errors, 0 warnings |
| Python host | `ast.parse` | parses |

## The adversarial harness

`🧪️verify/📜️script.ts` — **84/84 checks pass.** Every injected fault is caught by its intended gate:
wildcard subsets, duplicate owners, runtime-only / manifest-only / test-only mutations, outcome and
variant drift, a missing runtime inventory, a Semio-derived oracle, collapsed engine independence, a
no-oracle decision reaching for a mutation, a production-reachable oracle, a networked probe, an
unseeded sampling probe, a tampered fixture digest, a missing fixture file, a wildcard fixture target,
an unlicensed fixture, an uncapped tolerance override, a non-digest blob name, a live peer's run, a
reclaimable dead one, failure evidence, pinned evidence, an unreferenced blob, a symlinked cache root,
wrong geometry, wrong component count, a missing probe report, a failed probe, an unmeasured assertion,
a vacuous release gate, and a subject that replayed a vector.

## Discovery, the thing that was broken

| Measurement | Baseline | Now |
| --- | --- | --- |
| Owner contributions discovered | **0** | 169 |
| Malformed contributions | (invisible) | 0 |
| Test cases discovered | 4 — *all scratch copies inside one ticket folder* | **168** |
| Registered oracles visible | 0 | 138 |
| Mutation catalogs visible | 0 | 147 |

## Coverage, honestly

`test matrix` — `fixtureClassCoverage` 100% (3/3), `fixtureProvenanceCoverage` 100% (26/26),
`subsetOwnershipCoverage` 100%, `externalOracleCoverage` 100%, `dependencyIsolationCoverage` 100%.
`runtimeMutationCoverage` **0/1** — the cc6 manifest has no runtime inventory because its production
bridge cannot build, and that now reads as *uncovered* rather than as a vacuous 100%.

`test matrix --enforce` **blocks**, naming `runtimeMutationCoverage` and `productionBridgeCoverage` as
having empty denominators. That is the correct answer, and before this ticket's fix it was a green pass.

The report answers its ten questions by name. *"Which tests still use a Semio-derived oracle?"* lists
**57 oracle ids** — 41% of the registry.

## Oracle purity

`oracleImportsInProduction`: **36 → 1.**

Thirty-five were scope defects, now fixed: the scan walked the repository's own meta directory (a
scratch file in somebody's ticket read as production), and it did not know that `🔬️probes` and
`🏭️generator` are test-owned by what they are, so the platform's own measurement tools were reported as
production dependencies on the libraries they exist to invoke.

The one that remains is real and is left red:
`♻️mit-bestand/recherche/_neo4j/review/…/audit_sources.py` imports `pypdf`, a registered oracle. That
area is declared `clean` in the taxonomy, so it is in scope, and whether to retire the import or record
it as shrink-only debt belongs to its owner.

Separately, `brepjs` IS production-reachable — `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️brep-implementation.ts`
ships it as the CAD plugin's exact-BRep implementation — and is recorded as `productionDebt` with a path,
an owner and a retirement plan. The self-test asserted every oracle-linked package must classify as
`test-oracle` with no exemption for recorded debt, which meant honestly recording that reachability
failed the very gate demanding it be recorded. Recorded debt is now exempt and must carry all three
fields; unrecorded reachability still fails hard.

## Dependency ratchet

`dependency --scan`: **131 declared external dependencies, 84 production-reachable, 0 new production.**
Exit 0. It compares the committed baseline against a live scan of the tree — before this ticket it
compared the baseline against *itself*, so it could never see a new production dependency in any
language.

## Platform self-test: 56 → 67 passing

The 8 remaining failures, each attributed:

| Failure | Cause | Whose |
| --- | --- | --- |
| `every exempt area is excluded…` | the taxonomy's `areas` map now has **zero** `exempt` entries | peer taxonomy wave |
| `…never returns a compose path` | asserts `compose/` **exists**; a peer wave deleted it | peer (COMPOSE-TO-PUZZLE5D) |
| `no tracked fixture… is ever a clean candidate` | same — asserts `compose/` exists | peer |
| `every committed case satisfies the frozen contract` | 168 cases are now visible where 4 were; their real contract breaches surface for the first time | genuine backlog |
| `the migration backlog is a shrink-only ratchet` | the unmanaged-test count rose as peers added ticket scratch during this session | genuine, transient |
| `oracle purity` ×2 | the single `♻️mit-bestand` import above | genuine, that area's owner |
| `recorded production debt` | same import | genuine |

None is a regression from Protocol v2. Three are peer waves that deleted or emptied what the tests
assert; the rest are findings that only became visible because the registry stopped loading empty.
