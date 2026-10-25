#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="sh $C/verify-d-tools/lane.sh"
echo "--- chain2 start $(date +%H:%M)" >> "$C/🗑️generated/verify-d/verdicts.tsv"
$L build-owners.2 build-owners
$L tool-spawns.2 tool-spawns
$L tool-declarations.2 tool-declarations
$L package-purity.2 package-purity
$L composed-child-refs.2 composed-child-refs
$L semantic-vocabulary.2 semantic-vocabulary
$L production-placeholders.2 production-placeholders
$L docstrings-at-emoji docstrings at-emoji
$L docstrings-emoji-first docstrings emoji-first
$L docstrings-emoji-unique docstrings emoji-unique
$L agnostic agnostic
$L mutation-caps mutation-caps
$L debug-tags debug-tags
$L interface-owners interface-owners
$L abstraction-ownership abstraction-ownership
$L taxonomy-implementation-enforce taxonomy implementation enforce
$L taxonomy-enforce taxonomy enforce
$L dependencies dependencies
$L dependencies-locks dependencies locks
echo CHAIN2-DONE >> "$C/🗑️generated/verify-d/verdicts.tsv"
