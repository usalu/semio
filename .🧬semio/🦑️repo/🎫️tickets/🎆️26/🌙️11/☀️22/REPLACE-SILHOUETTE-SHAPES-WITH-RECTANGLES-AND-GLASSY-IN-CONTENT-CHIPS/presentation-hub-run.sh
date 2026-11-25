#!/bin/zsh
# usage: presentation-hub-run.sh <still|browser>; runs one still of the presentation focus frame under the manim lock, output in 🗑️generated
G="$(cd "$(dirname "$0")" && pwd)"
R=/Users/ueli/Documents/semio
mkdir -p "$G/🗑️generated"
export PYTHONPATH="$R"
if [ "$1" = browser ]; then
  exec lockf -k /private/tmp/claude-501/semio-manim.lock "$G/presentation-hub-py.sh" "$G/presentation-hub-browser-still.py" "$G/🗑️generated/hub-browser" > "$G/🗑️generated/browser-still.txt" 2>&1
fi
exec lockf -k /private/tmp/claude-501/semio-manim.lock zsh /private/tmp/claude-501/still.sh "$R" "${G#$R/}/presentation-hub-still.py" FocusFrame "$G/🗑️generated/hub-python" > "$G/🗑️generated/python-still.txt" 2>&1
