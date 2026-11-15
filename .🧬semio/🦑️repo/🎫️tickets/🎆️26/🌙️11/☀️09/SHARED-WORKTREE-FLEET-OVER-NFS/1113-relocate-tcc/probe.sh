#!/bin/bash
# Usage: probe.sh <variant: direct|launcher|shell|agent|cleanup> <random> ; macOS only; creates only ~/Documents/semio-tcc-probe-<random> and one LaunchAgent per variant.
set -u
VARIANT=$1; RANDOM_ID=$2
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
HERE=$(cd "$(dirname "$0")" && pwd)
DIR=$HOME/Documents/semio-tcc-probe-$RANDOM_ID
OUT=/private/tmp/semio-tcc-probe-$RANDOM_ID.txt
LABEL=tech.semio.tcc-probe-$RANDOM_ID-$VARIANT
PLIST=$HOME/Library/LaunchAgents/$LABEL.plist
UIDN=$(id -u)
WORK=/Users/Shared/semio
if [ "$VARIANT" = cleanup ]; then
  for v in direct launcher shell agent protected; do
    l=tech.semio.tcc-probe-$RANDOM_ID-$v
    launchctl bootout gui/$UIDN/$l 2>/dev/null
    rm -f "$HOME/Library/LaunchAgents/$l.plist"
  done
  rm -rf "$DIR" "$OUT" "$HOME/.local/share/semio-tcc-probe-$RANDOM_ID"
  exit 0
fi
mkdir -p "$DIR"; : > "$DIR/inside.txt"
case $VARIANT in
  direct) ARGS="<string>$BUN</string><string>$HERE/probe.js</string><string>$DIR</string><string>$OUT</string><string>$VARIANT</string>" ;;
  launcher) ARGS="<string>/usr/local/bin/repo</string><string>$HERE/probe.js</string><string>$DIR</string><string>$OUT</string><string>$VARIANT</string>" ;;
  shell) ARGS="<string>/bin/sh</string><string>-c</string><string>$BUN $HERE/probe.js $DIR $OUT $VARIANT; exit \$?</string>" ;;
  protected) ARGS="<string>/usr/local/bin/repo</string><string>$HERE/probe.js</string><string>$DIR</string><string>$OUT</string><string>$VARIANT</string>"; WORK=$HOME/Documents/semio ;;
  agent) AGENT=$HOME/.local/share/semio-tcc-probe-$RANDOM_ID/semio-fleet-agent; mkdir -p "$(dirname "$AGENT")"; cp /bin/sh "$AGENT"
    ARGS="<string>$AGENT</string><string>-c</string><string>$BUN $HERE/probe.js $DIR $OUT $VARIANT; exit \$?</string>" ;;
esac
cat > "$PLIST" <<PL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict><key>Label</key><string>$LABEL</string>
<key>ProgramArguments</key><array>$ARGS</array>
<key>EnvironmentVariables</key><dict><key>SEMIO_WORKSPACE_ROOT</key><string>$WORK</string></dict>
<key>WorkingDirectory</key><string>$WORK</string>
<key>RunAtLoad</key><true/><key>StandardOutPath</key><string>/private/tmp/$LABEL.log</string><key>StandardErrorPath</key><string>/private/tmp/$LABEL.log</string></dict></plist>
PL
launchctl bootstrap gui/$UIDN "$PLIST"
