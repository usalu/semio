#!/bin/sh
# usage: story-static-build.sh ; static Storybook (ui+styling) into 🗑️generated/static, prototype mode like the dev recipe
R=/Users/ueli/Documents/semio
T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS"
cd "$R" && STORYBOOK_SCOPE=ui,styling STORYBOOK_DISABLE_TELEMETRY=1 SEMIO_BUILD_MODE=prototype SEMIO_VITE_CACHE_DIRECTORY="$R/.cache/b/p/m/finishing-sb-vite" exec "$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)" node_modules/storybook/dist/bin/dispatcher.js build -c .storybook -o "$T/🗑️generated/static" --disable-telemetry > "$T/🗑️generated/storybook-static-build.txt" 2>&1
