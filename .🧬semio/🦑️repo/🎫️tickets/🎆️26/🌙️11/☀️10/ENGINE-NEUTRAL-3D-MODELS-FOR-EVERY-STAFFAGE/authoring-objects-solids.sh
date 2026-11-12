BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1); cd "$(dirname "$0")"; $BUN authoring-objects-solidcheck.ts "$1"
