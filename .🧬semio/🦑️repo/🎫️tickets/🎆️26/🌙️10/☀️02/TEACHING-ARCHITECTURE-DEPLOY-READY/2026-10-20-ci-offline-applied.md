# Native CI and Offline Bundle Implementation

The permanent native CI corpus/schema and existing site-build consumer were added first, followed by existing readiness expectations and compose/workflow assertions. Production workflow, staged offline plan and README changes followed. Native collection remains blocked by live shared-resource admission, so no observed assertion RED or GREEN is claimed. This supersedes the earlier self-imposed hold requiring observedRED before implementing these already tested-first fixes.

All three CI jobs retain the existing pinned actions, dispatch/defaultbranch/authentication order, sudo PATH/HOME/release/CI environment and publication boundaries. Each probes the absolute native executable, performs one existing owned recovery only when absent/unusable (birthrelease, skipcache, no explicitconfiguration), requires final help success, publishes its directory through GITHUB_PATH, then uses native repo for all13ordinary commands. The17-row corpus is a metadata/schema contract check, not17executed shell scenarios.

Offline staging now rejects an unexpected authored proctor policy and writes pull_policy:never for both proctor and Caddy, keeping the existing pinned Caddy localdigest alias. Onlinecompose and operator.env/certificates/volumes/tag handling remain. Bothimages are still included in the same saved archive. README explicitly documents bothservice no-pull behavior; 53 ordinary command examples across English/German now consistently use native repo. Historical actualverification claims were not expanded.

Independent afteredit review is2026-10-20-ci-offline-current-patch-audit.md. YAML/JSON parse and three changed TypeScript syntax parses are clean; this is not native test execution or CI runtime proof. Final currentSource architecture verification41305 remains alive, originallynamedred4.log; its filename is not an observedREDclaim. Finalfresh Source seal must include actual newhashes after finalreview/actualGREEN, not preparedpatch basehashes.

The compiler-runner documentation now places native frontend recovery/help/PATH before ordinary preflight and dependency preparation, matching the actual workflow; it no longer promises preflight precedes the cold frontend bootstrap.

## Current File Roster

- .github/workflows/architecture-course.yml: 11265B SHA256 48b33b5b58cd1b4390ac5dec840ba7a9961bc5091c192ad3a3d6a1a4117866c8
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🔣️.json: 8347B SHA256 0d5f8fff06529be16df577c9075dabb7751fa036d00781628b5d45e55fd6dbb8
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🛗️native/🔣️.json: 6046B SHA256 58ca9c85c8bcf261d3c7cb37098120860995a63eb5f497c604bc677efc2b82d5
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🛗️native/🧬️schema/🔣️.json: 7999B SHA256 558d5f99583cbbb9eacc24d1bbc66a11188800cf4ef83033b20f65435ad4aedb
- 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🏗️site-build/🟦️.ts: 5832B SHA256 34b9cba13d3c069fcdba0865e0b55992a62f8b583813f422c50c63eefb884de1
- 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🧩️compose-caddy-and-workflow/🟦️.ts: 9195B SHA256 b1927ca919ee7526b5aa6286a8339c2763fda83d1f15df3d2d94a385a38151ba
- 🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🎁️stack-bundle/🟦️.ts: 9718B SHA256 d4e3a6129fa137db968135560993c2ec2ac6fd35297c12c8a8c6941951f8d07e
- 🎓️teaching/🏛️architecture/📚️course/README.md: 82141B SHA256 19c46b4308354ce638f29b05b65cc66db23f890781b5869df69811b2224598a9
