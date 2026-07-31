#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖️ComposeEntrypoint
# 🚀️Execs the devcontainer command; Neo4j is started by post-start inside the single compose container.
set -eu
exec "$@"
#endregion 🔖️ComposeEntrypoint
