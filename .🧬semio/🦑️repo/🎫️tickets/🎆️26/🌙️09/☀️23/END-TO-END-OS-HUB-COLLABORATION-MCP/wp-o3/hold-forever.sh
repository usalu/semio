#!/bin/zsh
export OS_HUB_CREDENTIAL_SIGN_IN=1 OS_HUB_ADMIN_TOKEN=e2e-admin
exec bun "$@"
