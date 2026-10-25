#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="sh $C/verify-d-tools/lane.sh"
echo "--- chain4 start $(date +%H:%M)" >> "$C/🗑️generated/verify-d/verdicts.tsv"
$L pb-artifact-builder policy-breach artifact-builder
$L pb-artifact-decomposer policy-breach artifact-decomposer
$L pb-schema-representation policy-breach schema-representation
$L pb-io-serializer-matrix policy-breach io-serializer-matrix
$L pb-io-terminality policy-breach io-terminality
$L pb-codec-fidelity policy-breach codec-fidelity
$L pb-standards-coverage policy-breach standards-coverage
$L pb-artifact-analyzer policy-breach artifact-analyzer
$L pb-artifact-composer policy-breach artifact-composer
$L pb-artifact-builder-migrated policy-breach artifact-builder-migrated
$L pb-plugin-dependency-parity policy-breach plugin-dependency-parity
$L pb-contribution-target policy-breach contribution-target
$L artifact-field-parity artifact-field-parity
echo CHAIN4-DONE >> "$C/🗑️generated/verify-d/verdicts.tsv"
