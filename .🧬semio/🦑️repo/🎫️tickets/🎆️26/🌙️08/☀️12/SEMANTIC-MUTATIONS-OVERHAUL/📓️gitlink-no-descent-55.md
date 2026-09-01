# Gitlink No-Descent IO Contract

## Proposed Test Footprint

This ticket-only packet adds a schema-validated neutral vector and one controller at [gitlink-no-descent-55](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-no-descent-55). It will extract the real normalization `sourceAdmissionWalk` and `collectTaxonomySourceAdmission` into closed mock-I/O harnesses. No real workspace scan, nested repository access, or production source change is included.

The contract is deliberately conservative: complete Git index `160000` entries become private collector-derived fences before scope filtering. A walker can observe the exact fence root once but cannot enumerate it or probe a child. A root strictly under a fence—whether an explicit ticket, ignored output, or requested scope—must reject before filesystem probing. Matching is slash-segment exact and preserves raw NFD spelling; a prefix sibling remains ordinary.

The fixture covers exact and ancestor fences, a child root, prefix sibling, NFD spelling, a conflicted Gitlink fence, ordinary directory behavior, virtual `CoMpOsE`, and the three collector roots under a fence. The future source shape under review is a sixth private `readonly string[]` fence argument on `sourceAdmissionWalk`; the collector derives it solely from its complete index rows. This is not a public authority input.

## Required Source Boundary, If Released

Only normalization source-admission IO may change after review: derive fences from all `160000` entries (including nonzero/conflicted stages) before scope filtering; validate scope, explicit ticket, and ignored output roots against them; and prevent descent in the walker. The source-admission schema, S/D, taxonomy, and other policy families remain outside scope.

## Actual Current-Source RED

The completed [controller](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-no-descent-55/📜️script.ts) uses Ajv 2020 to validate the neutral [schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-no-descent-55/🧫️fixtures/🔣️schema.json) and [vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-no-descent-55/🧫️fixtures/🔣️vectors.json). It extracts the actual `sourceAdmissionWalk`, `collectTaxonomySourceAdmission`, `sourceAdmissionPrepareOptions`, and `sourceAdmissionCheckCancellation` through TypeScript AST declarations, then invokes them with a closed call-counting filesystem. The collector's actual Git-row dependency receives only synthetic full-index `160000` rows—including stages 1/2 for the conflict case—so no real repository or nested Git worktree is read.

The retained [RED receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-no-descent-55/🧫️runs/red-102de4ed-0b9b-4107-b45e-42b2966c7e00/result.json) ran through scoped Bun/Nx and exits nonzero after persisting all observations. It has 53 assertions across 13 cases at normalization SHA-256 `34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f`, with 22 desired-contract failures:

- exact, ancestor, NFD, and conflicted Gitlink roots are enumerated twice, readdir-ed, and child-lstat-ed;
- a child root under a fence does not reject before lstat;
- explicit ticket and ignored-output roots under a fence both invoke the actual walker and probe descendants;
- a scoped child root silently completes rather than rejects;
- taxonomy and cancellation paths under a fence are lstat-ed before an index fence can be consulted.

The ordinary prefix sibling and ordinary directory remain walkable; virtual `CoMpOsE` is rejected before any mock filesystem call. The receipt also proves the controller never enables a content-read seam. This is a genuine current-source RED, not a defect-expecting success mode, and there is no production edit or runtime claim.
