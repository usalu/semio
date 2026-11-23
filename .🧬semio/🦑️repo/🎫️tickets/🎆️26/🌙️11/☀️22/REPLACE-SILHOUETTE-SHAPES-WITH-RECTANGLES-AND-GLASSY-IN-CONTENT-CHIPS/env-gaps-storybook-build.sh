#!/bin/sh
# usage: env-gaps-storybook-build.sh ; builds the ui+styling Storybook with the build script behind `build-storybook` (the graph route fails on a missing suite import); output to 🗑️generated
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
cd "$R" && export STORYBOOK_SCOPE=ui,styling STORYBOOK_PORT=6010 PLAYWRIGHT_BROWSERS_PATH="$R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"
exec timeout 900 repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts build storybook > "$T/🗑️generated/storybook-build2.txt" 2>&1
