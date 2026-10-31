#!/bin/bash
D="$(dirname "$0")"
"$D/chk.sh" $1 "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.toml" wasm --lib -p semio-s-artifact-playbook-playbook-inferences -p semio-s-artifact-playbook-playbook-examples -p semio-s-artifact-playbook-playbook-schema
"$D/chk.sh" $(($1+1)) "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml" wasm --lib -p semio-s-artifact-demonstrator-playground-editor
