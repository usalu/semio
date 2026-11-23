#!/bin/sh
D="$(cd "$(dirname "$0")" && pwd)"
for p in semio-ui-render semio-ui-render-shader-contract semio-ui-backend-webgpu semio-ui-backend-metal semio-ui-draw-contract semio-ui-wgpu-draw-execution semio-ui-wgpu-pipelines semio-ui-wgpu-light semio-ui-wgpu-paint semio-ui-wgpu-mounted-layout semio-ui-wgpu-paint-laws; do
  sh "$D/cargo-ui.sh" "lane-test-$p" test -p "$p"
done
echo finished > "$D/🗑️generated/lane-done.txt"
