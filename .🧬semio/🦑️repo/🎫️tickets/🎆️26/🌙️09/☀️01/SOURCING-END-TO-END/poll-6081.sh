#!/bin/sh
# Polls the sourcing serve every 30 s for up to 60 min; exits 0 once it answers 200.
for i in $(seq 1 120); do
  [ "$(curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:6081/)" = "200" ] && { echo UP; exit 0; }
  sleep 30
done
echo TIMEOUT; exit 1
