#!/bin/zsh
# 🐳️ H10: runs the built `semio/os-hub:<tag>` image in its production posture on a named volume seeded with a trusted
# catalog (and the verification memory a host hub of the same engine wrote), then measures /healthz, /readyz (booting
# answers + progress until 200), a credential sign-in through the proxy headers, and a clean `docker stop`.
# usage: docker-run-drill.sh <tag> <host port> <catalog data root to seed from>
TAG=$1; PORT=$2; SRC=$3
IMAGE="semio/os-hub:$TAG"; VOL="semio-h10-data-$TAG"; NAME="semio-h10-$TAG"
now_ms() { python3 -c 'import time;print(int(time.time()*1000))'; }
echo "=== start $(date +%T) image=$IMAGE volume=$VOL"
docker rm -f "$NAME" >/dev/null 2>&1
docker volume rm "$VOL" >/dev/null 2>&1
docker volume create "$VOL" >/dev/null
docker run --rm --user root --entrypoint sh -v "$VOL:/srv/semio-hub/data" -v "$SRC/trusted-catalog:/seed/trusted-catalog:ro" "$IMAGE" -c \
  'cp -R /seed/trusted-catalog /srv/semio-hub/data/ && chown -R semio:semio /srv/semio-hub/data && chmod 700 /srv/semio-hub/data /srv/semio-hub/data/trusted-catalog && ls /srv/semio-hub/data/trusted-catalog' || exit 1
printf '%s' "h10-docker-pass-1" | docker run --rm -i -v "$VOL:/srv/semio-hub/data" "$IMAGE" credential set --email ada@example.com --display-name Ada || exit 1
started=$(now_ms)
docker run -d --name "$NAME" -p "127.0.0.1:$PORT:8787" -v "$VOL:/srv/semio-hub/data" \
  -e OS_HUB_ADMIN_SUBJECTS=credential.password.v1:ada@example.com -e OS_HUB_ALLOWED_ORIGINS=https://s.example.com \
  -e OS_HUB_TRUSTED_FORWARDING=proxy --stop-timeout 30 "$IMAGE" >/dev/null || exit 1
proxy=(-H "X-Forwarded-Proto: https" -H "X-Forwarded-Host: hub.example.com")
first=""
while [ $(( $(now_ms) - started )) -lt 900000 ]; do
  body=$(curl -s --max-time 4 "${proxy[@]}" -w '\n%{http_code}' "http://127.0.0.1:$PORT/readyz")
  code=${body##*$'\n'}; json=${body%$'\n'*}
  if [ -n "$code" ] && [ "$code" != 000 ] && [ -z "$first" ]; then first=$(( $(now_ms) - started )); echo "first /readyz answer after $first ms: http=$code $(printf '%s' "$json" | head -c 400)"; fi
  if [ "$code" = 200 ]; then echo "ready after $(( $(now_ms) - started )) ms"; break; fi
  sleep 2
done
echo "healthz: $(curl -s --max-time 4 "${proxy[@]}" -w ' http=%{http_code}' "http://127.0.0.1:$PORT/healthz")"
echo "healthz without proxy headers: $(curl -s --max-time 4 -o /dev/null -w 'http=%{http_code}' "http://127.0.0.1:$PORT/healthz")"
for attempt in 1 2 3; do
  t0=$(now_ms)
  code=$(curl -s --max-time 30 "${proxy[@]}" -H "Origin: https://s.example.com" -H "content-type: application/json" -o /dev/null -w '%{http_code}' \
    -d "{\"schema\":\"semio.hub.auth.credential-sign-in/v1\",\"email\":\"ada@example.com\",\"password\":\"h10-docker-pass-1\",\"deviceInstanceId\":\"h10docker$(openssl rand -hex 11)\",\"clientClass\":\"browser\"}" \
    "http://127.0.0.1:$PORT/auth/sessions")
  echo "sign-in $attempt: http=$code $(( $(now_ms) - t0 )) ms"
  sleep 2
done
docker stats --no-stream --format '{{.Name}} mem={{.MemUsage}} cpu={{.CPUPerc}}' "$NAME"
t0=$(now_ms); docker stop "$NAME" >/dev/null; echo "docker stop took $(( $(now_ms) - t0 )) ms, exit code $(docker inspect -f '{{.State.ExitCode}}' "$NAME")"
docker logs "$NAME" 2>&1 | /usr/bin/grep -E "server.shutdown|server.readiness|server.boot|ERROR|panick" | tail -6
docker rm "$NAME" >/dev/null
echo "=== done $(date +%T)"
