#!/usr/bin/env bash
set -euo pipefail

# Keep this kit directory directly inside the repository root.
morph_kit_dir="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
morph_repo_root="$(CDPATH= cd -- "$morph_kit_dir/.." && pwd -P)"
morph_prompt_path="$morph_kit_dir/prompt.txt"
morph_agents_path="$morph_kit_dir/agents.json"

for morph_input_path in "$morph_prompt_path" "$morph_agents_path" "$morph_kit_dir/acceptance.md" "$morph_kit_dir/fleet.md"; do
  if [[ ! -f "$morph_input_path" || ! -r "$morph_input_path" || ! -s "$morph_input_path" ]]; then
    printf 'Missing, unreadable, or empty launch file: %s\n' "$morph_input_path" >&2
    exit 2
  fi
done

cd -- "$morph_repo_root"
morph_claude_bin="$(type -P claude || true)"
if [[ -z "$morph_claude_bin" ]]; then
  printf '%s\n' 'Claude Code was not found on PATH. Use the terminal where your existing Claude installation is available.' >&2
  exit 127
fi

export CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20
export CLAUDE_CODE_EFFORT_LEVEL=medium

exec "$morph_claude_bin" -p \
  --model claude-opus-5-5 \
  --effort medium \
  --agents "$morph_agents_path" \
  --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' \
  < "$morph_prompt_path"
