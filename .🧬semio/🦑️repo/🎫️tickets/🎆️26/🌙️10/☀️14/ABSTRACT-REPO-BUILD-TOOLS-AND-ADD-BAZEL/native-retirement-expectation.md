# Expected Image Retirement Admission

Source is held; no ticket image retirement has been executed. Low's final runtime and ticket review is clean. All High runtime and products/native custody has settled. Root retains the final cleanup release and SDK/ALL qualification lanes.

## Final Contract

The existing native storage schema now distinguishes `ImageRetirementRequest` from general `ImageMaintenanceRequest`. Retirement accepts an optional closed, all-fields-required `expectedImage` containing lowercase64hex raw receipt SHA-256 and safe finite backing dev/ino/size/allocated bytes. The parser copies each field before any await, freezes the nested image and outer request, and validates the constructed snapshot again so changing accessors cannot publish an invalid admitted image. General maintenance requests are unchanged.

The dispatcher and directly callable backends capture the retirement request before queued acquisition. Under the SAME canonical whole-image EX, the common validator checks the stable nofollow raw receipt SHA and a retained nofollow backing FD against bigint dev/ino/size/blocks*512 and stable named/descriptor stamps. Original checks surround the work; close always runs and preserves arbitrary primary failure. Mac and the shared Linux/Windows implementation check the expectation after EX acquisition, after awaited mount proof immediately before detach, and after detach before final removal. Existing receipt semantics, finite envelope, zero-allocation journal, actual mount proof, normal non-forced detach and post-detach proof remain required. No EX release/reacquire is used to establish historical identity. Existing callers without expected values retain their current self-consistency contract and make no new historical claim.

## TDD and Runtime Evidence

The first two runs were fixture failures: `retirement-expected-identity-red.log` had an invalid SQLite oracle and the genuine60 expired; `retirement-expected-identity-red-fixed-fixture.log` fixed that oracle but still failed to settle within the original60. Neither is semantic feature proof.

After adding an explicit actual Original continuation after blocker release, `retirement-expected-identity-queue-debug.log` captured the semantic RED: terminal1, 1pass/1fail/26assertions/666ms. A real canonical SQLite image EX blocker was held; retirement was submitted; backing and receipt were replaced and the caller's expected object mutated to the replacement before EX was released. The unguarded backend resolved true and removed the replacement where the law expected refusal. The detached fixture is ordinary owned filesystem data, not a physical-volume proof.

The guard produced `retirement-expected-identity-green.log`: terminal0, 2pass/31assertions/1.046s. Final affected contract suite `retirement-expected-contract-final.log`, session67233: **6pass/0fail/224assertions/1.473s**. It includes independent Ajv and SQLite schema vectors; detached request mutation/accessor negatives; actual queued replacement refusal; separate dev/ino/size/allocated mismatches preserving bytes; and the unchanged exact detached positive.

The existing products-funded physical lifecycle law was extended rather than replaced. `retirement-expected-physical.log`, session22092: **1pass/0fail/27assertions/7.62s**. The actual isolated Mac fixed image had backing and allocated bytes66,060,288, filesystem66,019,328 and mounted device16777310. A live reader prevented retirement; an actual Node content/filesystem oracle agreed; existing independent wrong capacity cases refused; the new wrong receipt SHA refused while preserving the mounted content; and exact expected identity allowed normal detach, mount/backing absence proof and owned removal. This proves the Mac isolated lifecycle, not Windows/Linux physical execution or full Nx pool provisioning. The ticket initial-authority image was untouched.

## Prepared Final Ticket Mode

`cleanup-preflight/📜️script.ts --final-retire` is now prepared with one closed literal historical request for `🗑️generated/initial-authority/.cache/b/l`. It requires the canonical budget directory/control/resource, exact existing receipt/backing, nofollow ancestor identities and finite genuine Original60; then passes the required expected image to the existing dispatcher with `wait:false`. Busy custody refuses. Successful managed retirement must prove directory/mount/backing absence. The script performs no raw rm/unlink/detach and cannot select global caches or peer stores. No broad generated cleanup is included.

`cleanup-model/📜️script.ts` completed terminal0 under genuine60 with nine literal proof vectors and four invalid input mutations checked against Ajv and SQLite, plus actual TypeScript AST assertions for one exact dispatcher call, no raw deletion and busy/post-absence checks. Retained `cleanup-retirement-model.log` and JSON explicitly state `retirementExecuted:false`. This is input/model proof, not a claim of ticket image EX eligibility or actual cleanup.

All authored reports/configuration/input scripts remain preserved. Actual ticket image retirement and final generated removal are still held until Root's explicit release.

## Current Source Hashes

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json`: `6dc0ddcf0e31d77f7225620c18e92d951bb17037225a3a4d1f343661eb24b158`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts`: `1093475635b685b2d9ddc5f4f04760146903d32994b31754d1b5d3962bf9d331`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts`: `febee7be7c5af3b2d78ea3f6fb4e001d3f2bc5c366495fb5e19e2f19fed5bf1e`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`: `93e8c873a23b4134eaf4a420eea27a231109fe78c56d2119fbde4b4f1a788139`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts`: `c0d1abb2f8369daeb0a4f4b314cfb1c680e799fd3aca616b4832fc00cdea79f3`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🐧️image/🟦️.ts`: `2e41f95868b26b0c339c7191966444589d4c6000242feb6155517b5b3e691dce`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️image/🟦️.ts`: `7f204ec604edc50ffb7975b6a6c1e8e139fe4525e4841c6fdcc6f1429804d383`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts`: `0a99303a78a7a9a47e4453b94473c641ccccc3289e920f565cac7b63271c708c`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🔣️.json`: `cbc2a3790c42fb33f3d2441de0b64518aebba8ca5b6de1294f7811676d169e7d`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts`: `849b82785d5720bfcafb2d16a5bfabc084e57b8968b65b94a05bba8fdf635078`

## Later Permanent-Route CWD Audit

See native-retirement-registration.md: the existing process dispatcher, target and bilingual launches include the new laws; graph import/aggregate includes the physical augmentation. A test-only locator was corrected from process.cwd() to canonical findWorkspaceRoot(import.meta.dir), because permanent process Bun children use framework cwd. Earlier runtime results above predate that test-only fix; actual permanent-route rerun is held for Root release. The Source hashes above record the gate-era bytes, not a claim of post-fix runtime acceptance. Production Source remains held.
