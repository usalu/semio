#!/bin/sh
D=$(dirname "$0")
sh "$D/test.sh" v1 -p semio-framework-os-store-one-item-batch --test mutation_apply --no-fail-fast --message-format=short
sh "$D/test.sh" v2 -p semio-framework-plugin-guest-contract -p semio-framework-plugin-host-effects -p semio-framework-os-config-merge-policy -p semio-framework-os-config
