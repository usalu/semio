#!/bin/bash
# Ticket input script: keeps the course dev site (port 6061) up, restarting it when a peer's live edit stales the graph.
cd /Users/ueli/Documents/semio || exit 1
for attempt in 1 2 3 4 5 6; do
  echo "[dev-site] attempt $attempt $(date +%H:%M:%S)"
  nice -n 10 bun repo --tool=bazel run @teaching/architecture-course:dev
  sleep 20
done
