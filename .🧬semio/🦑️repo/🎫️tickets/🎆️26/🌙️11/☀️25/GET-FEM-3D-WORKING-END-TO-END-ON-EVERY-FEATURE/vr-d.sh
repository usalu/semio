#!/usr/bin/env bash
# 🕹️ Posts a JS body (stdin) to the page driver on port $1 (default 6196).
curl -s --max-time 600 -X POST --data-binary @- "http://127.0.0.1:${1:-6196}/"
