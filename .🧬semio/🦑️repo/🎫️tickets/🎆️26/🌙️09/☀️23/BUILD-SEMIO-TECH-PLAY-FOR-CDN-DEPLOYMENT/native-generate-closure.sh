#!/usr/bin/env bash
# Native `generate` of the generated-source producers in the fem/demonstrator/energy closure (the graph is down). Logs: 🗑️generated/components/gen-*.log
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"; G="$T/🗑️generated/components"; mkdir -p "$G"
export SEMIO_BUILD_MODE=release SEMIO_WORKSPACE_ROOT=$R
gen(){ n=$1; c=$2; shift 2; ( cd "$R/$c" && repo capacity run --memory=1500 -- repo "$T/native-launch.ts" "$c" "$@" ) > "$G/gen-$n.log" 2>&1; echo "$n rc=$?"; }
M="🧰️framework/🔨️modules"
gen graph "$M/🕸️graph/📦️packages/🦀️rust" repo ./📜️script.ts generate
gen ui-contract "$M/🖱️ui/🧬️contract/📦️packages/🦀️rust" repo ./📜️script.ts generate
gen ui-styling "$M/🖱️ui/🎨️styling/📦️packages/🦀️rust" repo ./📜️script.ts generate
gen ui "$M/🖱️ui/📦️packages/🦀️rust" repo ./📜️script.ts generate
gen derive "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust" repo ./📜️script.ts generate
gen framework-root "🧰️framework" repo ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🦀️rust --manifest 📦️packages/🦀️rust/Cargo.toml -- repo ./📜️script.ts generate
