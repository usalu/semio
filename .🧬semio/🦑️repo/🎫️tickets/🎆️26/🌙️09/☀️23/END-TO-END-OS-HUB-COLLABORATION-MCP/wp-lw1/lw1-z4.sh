#!/bin/zsh
# ⚖️ LW1 → Z4 laws (T4 z4-b123 + z4-lifecycle) on the LIVE tree (both sets landed): discovery validateTaxonomy +
# validateGeneratorContractsAgainstWorkspace + container laws (lifecycle, runtime-bootstrap, devcontainer-context) via Z4's
# view-checks with view = repo; law `fresh-clone generator outputs`. No Docker, no network.
R=/Users/ueli/Documents/semio
bun "$R/.tmp-ticket/wp-z4/z4-view-checks.ts" "$R" "$R/.🧬semio/🌐hub/s14-lw1-logs/z4-view-out"; echo "LW1-STEP z4-view-checks rc=$?"
cd "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" && bun test "./🧪️tests/🔬️workspace-contract/🟦️.ts" -t "fresh-clone generator outputs"; echo "LW1-STEP fresh-clone-law rc=$?"
