#!/bin/bash
# GitKraken launcher script for VS Code integration
# This script handles WSL compatibility and prevents debugger hanging

WORKSPACE="${1:-/workspaces/semio}"

# Check if GitKraken is already running (exclude defunct processes)
RUNNING_PROCESSES=$(pgrep -f "gitkraken" | grep -v "defunct" | wc -l)
if [ "$RUNNING_PROCESSES" -gt 0 ]; then
    echo "GitKraken is already running ($RUNNING_PROCESSES processes)."
    exit 0
fi

# Set up virtual display if needed
if [ -z "$DISPLAY" ]; then
    export DISPLAY=:99
fi
if ! pgrep -f "Xvfb.*:99" >/dev/null 2>&1; then
    Xvfb :99 -screen 0 1920x1080x24 -ac +extension GLX +render -noreset >/dev/null 2>&1 &
    sleep 2
fi

echo "Starting GitKraken..."
gitkraken --no-sandbox --no-debug --disable-gpu --disable-dev-shm-usage --path "$WORKSPACE" &

echo "GitKraken launched successfully!"
