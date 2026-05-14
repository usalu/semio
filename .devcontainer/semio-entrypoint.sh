#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖SemioEntrypoint
# 🚀Execs the devcontainer command; Neo4j is started by post-start inside the single semio container.
set -eu
exec "$@"
#endregion 🔖SemioEntrypoint
