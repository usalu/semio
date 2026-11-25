#!/usr/bin/env bash
# 🔁️ Rebuilds the five Standards components one at a time, then sweeps all 15 Play routes.
T="$(cd "$(dirname "$0")" && pwd)"
REBUILD_ATTEMPTS=5 REBUILD_PAUSE_SECONDS=120 "$T/rebuild-and-probe.sh" sweep12 "norm-eurocode-structural norm-eurocode norm-eurocode-materials norm norm-energy" din4108 din16798 iso16757 vdi3805 din18599 en1990 en1991 en1997 en1998 en1992 en1993 en1994 en1995 en1996 en1999
