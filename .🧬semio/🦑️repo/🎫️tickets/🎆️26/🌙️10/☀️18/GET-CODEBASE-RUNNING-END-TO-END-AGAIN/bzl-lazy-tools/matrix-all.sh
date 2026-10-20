#!/bin/bash
# runs the representative matrix serially
T="$(cd "$(dirname "$0")" && pwd)"; M="$T/matrix.sh"
"$M" fg-generate '@semio-tech/framework-graph:generate'
"$M" fg-test '@semio-tech/framework-graph:test'
"$M" layout-src '@semio-tech/layout-layout-rs:test-media-adoption-source'
"$M" writer-test '@semio-tech/writer-writer-rs:test'
"$M" writer-build '@semio-tech/writer-writer-rs:build'
"$M" ui-styling '@semio-tech/ui-styling:generate'
"$M" hub-test 'os-hub-ts:test'
"$M" hub-typecheck 'os-hub-ts:typecheck'
"$M" arch-test '@teaching/architecture-course:test'
"$M" arch-typecheck '@teaching/architecture-course:typecheck'
"$M" arch-build '@teaching/architecture-course:build'
"$M" proctor-test '@teaching/proctor:test'
"$M" mb-bericht '@semio-tech/mit-bestand-bericht:akteursnetz-validate'
"$M" mb-demo-build '@semio-tech/mit-bestand-demonstrator:build'
"$M" mb-demo-activate '@semio-tech/mit-bestand-demonstrator:activate-prototype'
"$M" play-activate '@semio-tech/semio-tech-play:activate-prototype'
