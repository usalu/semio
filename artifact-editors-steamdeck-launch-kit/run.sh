#!/usr/bin/env bash
set -euo pipefail

# Keep this kit directory directly inside the repository root.
artifact_kit_dir="$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
artifact_repo_root="$(CDPATH= cd -- "$artifact_kit_dir/.." && pwd -P)"
artifact_prompt_path="$artifact_kit_dir/prompt.txt"
artifact_agents_path="$artifact_kit_dir/agents.json"

for artifact_input_path in "$artifact_prompt_path" "$artifact_agents_path"; do
  if [[ ! -f "$artifact_input_path" || ! -r "$artifact_input_path" || ! -s "$artifact_input_path" ]]; then
    printf 'Missing, unreadable, or empty launch file: %s\n' "$artifact_input_path" >&2
    exit 2
  fi
done

cd -- "$artifact_repo_root"
artifact_claude_bin="$(type -P claude || true)"
if [[ -z "$artifact_claude_bin" ]]; then
  printf '%s\n' 'Claude Code was not found on PATH. Use the terminal where your existing Claude installation is available.' >&2
  exit 127
fi

export CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20
export CLAUDE_CODE_EFFORT_LEVEL=medium

exec "$artifact_claude_bin" -p \
  --model claude-opus-5-5 \
  --effort medium \
  --agents "$artifact_agents_path" \
  --allowedTools 'Agent,Read,Glob,Grep,Write,Edit,Bash,WebSearch,WebFetch' \
  < "$artifact_prompt_path"
