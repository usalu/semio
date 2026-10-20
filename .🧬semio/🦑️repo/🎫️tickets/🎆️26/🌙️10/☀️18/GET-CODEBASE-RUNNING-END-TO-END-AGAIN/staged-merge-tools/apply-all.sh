#!/bin/sh
exec bun "$(dirname "$0")/plan.ts" apply "$@"
