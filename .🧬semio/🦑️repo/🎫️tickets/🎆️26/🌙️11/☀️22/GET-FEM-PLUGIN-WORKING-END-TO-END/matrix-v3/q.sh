#!/usr/bin/env bash
# Evaluates a JS body file against the page-driver on port 6290. Usage: q.sh file.js
curl -s -m 170 -X POST --data-binary "@$1" http://127.0.0.1:6290/
