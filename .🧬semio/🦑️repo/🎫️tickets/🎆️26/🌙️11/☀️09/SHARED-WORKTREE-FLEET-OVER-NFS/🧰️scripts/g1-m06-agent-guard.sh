#!/bin/bash
# 🛟 Restarts the replica LaunchAgent of this account while launchd leaves it down (`pended nondemand spawn = inefficient` after the daemon ended with 0 to start new code on a host under memory pressure), for as long as the native build retry loop runs; every restart is recorded.
# Usage: nohup bash g1-m06-agent-guard.sh >/dev/null 2>&1 &
# @see g1-m06-native-retry.sh
set -u
label=gui/$(id -u)/tech.semio.fleet.replica
record=$HOME/Documents/semio/.cache/f/agent-guard.log
while pgrep -f g1-m06-native-retry.sh >/dev/null; do
  if launchctl print "$label" 2>/dev/null | grep -q 'state = not running'; then
    sleep 20
    if launchctl print "$label" 2>/dev/null | grep -q 'state = not running'; then
      launchctl kickstart "$label"
      printf '%s kickstart %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$?" >> "$record"
    fi
  fi
  sleep 30
done
printf '%s ended with the native retry loop\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$record"
