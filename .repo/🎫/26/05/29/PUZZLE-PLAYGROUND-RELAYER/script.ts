#!/usr/bin/env bun
/** 🧩 Puzzle playground relayer ticket router. */
const cmd = process.argv[2];
const tasks: Record<string, string> = {
  "rename-scopes": "./rename-scopes.ts",
  "clean-react-imports": "./clean-react-imports.ts",
  "complete-relayer": "./complete-relayer.ts",
  "extract-play-hosts": "./extract-play-hosts.ts",
  "fix-2d-merge": "./fix-2d-merge.ts",
  "fix-3d-merge": "./fix-3d-merge.ts",
  "fix-host-imports": "./fix-host-imports.ts",
  "merge-host-back": "./merge-host-back.ts",
  "move-hosts-to-renderer": "./move-hosts-to-renderer.ts",
  "split-play-host": "./split-play-host.ts",
};

if (!cmd || !tasks[cmd]) {
  console.error(`usage: bun ./script.ts <${Object.keys(tasks).join("|")}>`);
  process.exit(1);
}

await import(tasks[cmd]!);
