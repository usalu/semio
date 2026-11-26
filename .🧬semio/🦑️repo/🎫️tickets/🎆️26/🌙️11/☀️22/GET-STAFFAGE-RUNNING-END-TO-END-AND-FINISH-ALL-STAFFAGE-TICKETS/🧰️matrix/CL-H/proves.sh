#!/bin/zsh
# 🩺️ `prove.sh` for the five assets of lane CL-H one after the other inside one grant, each with the heading of its closing section in `requests.md`.
h="${0:A:h}"
zsh "$h/prove.sh" person-queuing '## closing — person-queuing'
for a in person-on-crutches tour-guide parent-with-stroller parent-carrying-toddler; do zsh "$h/prove.sh" "$a" '## closing — nine assets'; done
