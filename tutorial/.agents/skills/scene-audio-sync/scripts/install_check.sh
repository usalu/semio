#!/usr/bin/env bash
# Verify skill dependencies for scene-audio-sync.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ok=1

need() {
  if command -v "$1" >/dev/null 2>&1; then
    echo "OK  $1: $(command -v "$1")"
  else
    echo "MISS $1 — install ffmpeg (brew install ffmpeg / apt install ffmpeg)"
    ok=0
  fi
}

need ffmpeg
need ffprobe

if python3 -c "import edge_tts" 2>/dev/null; then
  echo "OK  edge_tts"
else
  echo "MISS edge_tts — run: python3 -m pip install 'edge-tts>=6.1.0'"
  ok=0
fi

if [[ "$ok" -eq 1 ]]; then
  echo "All required deps present."
  exit 0
fi
echo "Dependency check failed."
exit 1
