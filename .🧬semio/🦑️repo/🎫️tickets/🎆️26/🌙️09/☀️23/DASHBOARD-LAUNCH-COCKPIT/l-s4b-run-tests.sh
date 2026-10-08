#!/bin/zsh
# Runs table A rows 28-54 library tests one at a time: l-s4b-run-tests.sh <label> [route...]
set -u
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
label="$1"; shift
out="$T/🗑️generated/launch-s4b/$label"
mkdir -p "$out"
typeset -A folder
folder=(
  taxonomy-cli-cancellation "🛑️taxonomy-cli-cancellation"
  transaction-recovery-authority "🛟️transaction-recovery-authority"
  typescript-path-collection "🛤️typescript-path-collection"
  preflight-reference-basis "🛫️preflight-reference-basis"
  readme-current-source-activation "🟢️readme-current-source-activation"
  gherkin-description-inline-code "🥒️gherkin-description-inline-code"
  rust-finite-target-consumption "🥤️rust-finite-target-consumption"
  repo-source-ownership "🦑️repo-source-ownership"
  os-dev-composition-ownership "🧑‍💻os-dev-composition-ownership"
  framework-root-source-topology "🧰️framework-root-source-topology"
  cargo-transaction-command-source "🧱️cargo-transaction-command-source"
  framework-source-topology "🧱️framework-source-topology"
  manifestless-source-closure "🧱️manifestless-source-closure"
  root-artifact-dependency-source "🧱️root-artifact-dependency-source"
  root-artifact-schema-law-source "🧱️root-artifact-schema-law-source"
  root-clean-scaffold-source "🧱️root-clean-scaffold-source"
  root-inference-law-source "🧱️root-inference-law-source"
  root-schema-field-source "🧱️root-schema-field-source"
  root-surface-abstraction-law-source "🧱️root-surface-abstraction-law-source"
  root-taxonomy-workflow-source "🧱️root-taxonomy-workflow-source"
  wasm-package-wrappers "🧱️wasm-package-wrappers"
  workspace-publication-source "🧱️workspace-publication-source"
  rust-physical-reference-context "🧲️rust-physical-reference-context"
  registry-catalog-gitlink-boundary "🧾️registry-catalog-gitlink-boundary"
  windows-command-paths "🪟️windows-command-paths"
  artifact-empty-facet-authoring "🪶️artifact-empty-facet-authoring"
  artifact-empty-facet-authority "🫙️artifact-empty-facet-authority"
)
for route in "$@"; do
  source="$PWD/$L/🧪️tests/${folder[$route]}/🟦️.ts"
  artifacts="$PWD/$T/🗑️generated/launch-s4b/artifacts/$route"
  mkdir -p "$artifacts"
  start=$(date +%s)
  SEMIO_TEST_ARTIFACT_DIR="$artifacts" bun test "$source" > "$out/$route.txt" 2>&1
  code=$?
  end=$(date +%s)
  summary=$(/usr/bin/grep -E "^ *[0-9]+ (pass|fail|skip)|^Ran " "$out/$route.txt" | tr '\n' ' ')
  echo "$route exit=$code $((end-start))s :: $summary"
done
