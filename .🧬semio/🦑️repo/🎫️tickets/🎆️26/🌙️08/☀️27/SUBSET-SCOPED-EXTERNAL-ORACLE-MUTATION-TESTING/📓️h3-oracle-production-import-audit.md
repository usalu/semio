# 📓️ H3 — oracle imports in production: audit

`oracleImportsInProduction` reported **36** hits at the moment contribution discovery started working.
After two scope fixes in this ticket it reports **7**, and an independent audit classified all seven.

## What the two scope fixes were

1. **The repository's own meta directory is never production source.** The scan walked
   `.🧬semio/🦑️repo/🎫️tickets/**` and reported a scratch file inside somebody's ticket folder as a
   production import of a registered oracle. That is a finding about a scratch file, not about what
   ships.
2. **`🔬️probes` and `🏭️generator` are test-owned by what they are**, exactly like `🧪️oracle`. All three
   link a reference library *on purpose*. The scan only knew the third, so the test platform's own
   measurement tools were reported as a production dependency on the library they exist to invoke.
   Both names are now taxonomy vocabulary (`testProbeDirName`, `testGeneratorDirName`).

## The remaining 7

| Path | Package | Class | Why |
| --- | --- | --- | --- |
| `…/📄️pdf/🧪️tests/extract-text-pdf-1-4/🐍️component.py` | pypdf | B | oracle adapter in a test case |
| `…/🏗️ifc/🧪️tests/differential-ifc-4/🐍️component.py` | ifcopenshell | B | oracle adapter in a test case |
| `…/🏗️ifc/🧪️tests/differential-ifc-2x3/🐍️component.py` | ifcopenshell | B | oracle adapter in a test case |
| `…/🔣️json/🧪️tests/mutate-json-rfc8259-i-json/🐍️component.py` | simplejson | B | oracle adapter in a test case |
| `…/🏷️class-name-composition/🧪️tests/flatten-class-name-inputs/🟦️component.ts` | clsx | B | oracle adapter in a test case |
| `…/🏷️style-variants/🧪️tests/compile-style-variants/🟦️component.ts` | class-variance-authority | B | oracle adapter in a test case |
| `…/🎠️kernel/🧪️tests/satisfy-version-requirements/🟦️component.ts` | semver | B | oracle adapter in a test case |

**Class A (real production dependency): 0. Class C (false positive): 0.**

All seven are the SAME artifact with the same single cause: `discoverTestCases` currently returns 4
instead of ~169, because the taxonomy renders `🥒️.feature` while the case files on disk are still named
`component.feature`. `isTestOwned` derives its case-directory set from that discovery, so every real
test case is invisible to the scan and its oracle adapter looks like production source. Each of the
seven files says so in its own header — e.g. `🏷️class-name-composition`'s adapter carries the line
"The registered `clsx` reference implementation — linked only here, never by production."

**These seven resolve themselves the moment the case-file rename lands.** No code change is owed.

## The one real reachability, recorded rather than hidden

Separately from these seven, `brepjs` IS genuinely production-reachable:
`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️brep-implementation.ts` imports `brepjs` and
`brepjs-opencascade` as the CAD plugin's production exact-BRep implementation, behind the `OwnedBrep*`
interface the repository requires of an external library.

That is the plan's shared-kernel-ancestry risk, already realised, and it is recorded as shrink-only
`productionDebt` on the `brepjs-occt` registration with its retirement plan. Its consequence differs
per artifact: for `s.cad.cad` brepjs is **not** independent evidence (reference and subject would share
one OpenCASCADE kernel and agree on its defects), while for `s.stdio.step@ap214/✳️cc6` — whose subject
is the pure-Rust STEP codec with no geometry kernel — it remains genuinely independent.
`engineIndependenceBreaches` reports the first case as soon as a CAD subject declares its engine family.
