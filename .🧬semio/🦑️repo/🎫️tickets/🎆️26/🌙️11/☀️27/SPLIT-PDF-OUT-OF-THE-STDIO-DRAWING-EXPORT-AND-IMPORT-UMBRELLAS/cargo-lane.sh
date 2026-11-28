#!/usr/bin/env bash
# 🚀️ repo cargo with fleet offload and without the stale remote-cache token of ~/.zprofile: cargo-lane.sh <jobs> <cargo args...>
jobs="$1"; shift
exec env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute SEMIO_LANGUAGE=en repo cargo --configuration=prototype --parallelism="$jobs" --store=shared -- "$@"
