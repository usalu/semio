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

# Check if we're in WSL environment
if grep -q "Microsoft\|WSL" /proc/version 2>/dev/null; then
    echo "Starting GitKraken with WSL-compatible flags..."
    # Start GitKraken with no-sandbox and no-debug flags for WSL compatibility
    gitkraken --no-sandbox --no-debug --path "$WORKSPACE" &
else
    echo "Starting GitKraken..."
    gitkraken --no-debug --path "$WORKSPACE" &
fi

echo "GitKraken launched successfully!"
