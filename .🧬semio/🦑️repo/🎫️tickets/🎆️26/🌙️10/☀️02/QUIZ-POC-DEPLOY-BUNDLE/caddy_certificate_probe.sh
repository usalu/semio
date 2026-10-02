#!/usr/bin/env bash
# Probes how the pinned Caddy image treats `tls { load /certificates }`: which certificate it serves and whether it
# still orders one, with an empty directory, with a matching PEM bundle, with an unreadable one, for `localhost` and for
# the public proctor host. Usage: caddy_certificate_probe.sh <Caddyfile> <scratch directory without emoji>
set -u
export MSYS_NO_PATHCONV=1
IMAGE="caddy:2.11.4@sha256:0c994536bddb66445885237f1a5dcc1916bccea922661c76b4e9fc24061f9b52"
CADDYFILE="$1"
SCRATCH="$2"
PUBLIC="semio.iek.uni-hannover.de"
NAME="caddy-certificate-probe"
PORT=18449
HERE="$(cd "$(dirname "$0")" && pwd -W)"
mkdir -p "$SCRATCH"
SCRATCH="$(cd "$SCRATCH" && pwd -W)"
cp "$CADDYFILE" "$SCRATCH/Caddyfile"

mint() {
  docker run --rm --entrypoint /bin/sh "$IMAGE" -c "printf '{\n\tskip_install_trust\n}\n$1 {\n\ttls internal\n\trespond ok\n}\n' > /tmp/Caddyfile && caddy start --config /tmp/Caddyfile --adapter caddyfile > /dev/null 2>&1; for i in \$(seq 1 100); do [ -s /data/caddy/certificates/local/$1/$1.crt ] && [ -s /data/caddy/certificates/local/$1/$1.key ] && break; sleep 0.2; done; cat /data/caddy/certificates/local/$1/$1.crt /data/caddy/certificates/local/$1/$1.key"
}

fingerprint() {
  bun "$HERE/certificate_fingerprint.ts" file "$1"
}

served() {
  bun "$HERE/certificate_fingerprint.ts" served "$PORT" "$1"
}

run() {
  docker rm --force "$NAME" > /dev/null 2>&1
  if [ "${3:-open}" = isolated ]; then NETWORK="--network $NAME-isolated"; else NETWORK="--publish 127.0.0.1:$PORT:443"; fi
  docker run --detach --name "$NAME" --env "PROCTOR_HOST=$1" --read-only --cap-drop ALL --cap-add NET_BIND_SERVICE $NETWORK --security-opt no-new-privileges --volume "$NAME-data:/data" --volume "$NAME-config:/config" --volume "$2:/certificates:ro" --mount "type=bind,source=$SCRATCH/Caddyfile,target=/etc/caddy/Caddyfile,readonly" "$IMAGE" > /dev/null
  sleep 6
  echo "state: $(docker inspect --format '{{.State.Status}} exit={{.State.ExitCode}}' "$NAME")"
}

logs() {
  docker logs "$NAME" 2>&1 | grep -E -i "$1" | cut -c1-420
}

docker volume rm --force "$NAME-data" "$NAME-config" "$NAME-certificates" > /dev/null 2>&1
docker volume create "$NAME-certificates" > /dev/null
docker network rm "$NAME-isolated" > /dev/null 2>&1
docker network create --internal "$NAME-isolated" > /dev/null

echo "== 1. localhost, empty certificates directory"
run localhost "$NAME-certificates"
served localhost
logs "skipping automatic|obtain|certificate obtained|error"

echo "== 2. localhost, a matching bundle (root:root 0600)"
mint localhost > "$SCRATCH/localhost.pem"
echo "supplied: $(fingerprint "$SCRATCH/localhost.pem")"
docker run --rm --interactive --volume "$NAME-certificates:/certificates" --entrypoint /bin/sh "$IMAGE" -c "cat > /certificates/own.pem && chmod 600 /certificates/own.pem && ls -ln /certificates" < "$SCRATCH/localhost.pem"
docker restart "$NAME" > /dev/null
sleep 6
echo "after restart: $(served localhost)"
logs "skipping automatic|obtain|error"

echo "== 3. localhost, the bundle owned by 1000:1000 with 0600 (not readable by a root without capabilities)"
docker run --rm --volume "$NAME-certificates:/certificates" --entrypoint /bin/sh "$IMAGE" -c "chown 1000:1000 /certificates/own.pem && ls -ln /certificates"
docker restart "$NAME" > /dev/null
sleep 6
echo "state: $(docker inspect --format '{{.State.Status}} exit={{.State.ExitCode}}' "$NAME")"
docker logs --tail 3 "$NAME" 2>&1 | cut -c1-420
docker run --rm --volume "$NAME-certificates:/certificates" --entrypoint /bin/sh "$IMAGE" -c "chmod 644 /certificates/own.pem && ls -ln /certificates"
docker restart "$NAME" > /dev/null
sleep 6
echo "0644 foreign owner: state $(docker inspect --format '{{.State.Status}}' "$NAME"), $(served localhost)"

echo "== 4. $PUBLIC on a network without egress, no bundle for it (the localhost bundle does not match)"
docker volume rm --force "$NAME-data" > /dev/null 2>&1
run "$PUBLIC" "$NAME-certificates" isolated
logs "skipping automatic|obtaining certificate|challenge|acme" | head -6

echo "== 5. $PUBLIC, a matching bundle: first without egress (the log), then published (the served certificate)"
mint "$PUBLIC" > "$SCRATCH/public.pem"
echo "supplied: $(fingerprint "$SCRATCH/public.pem")"
docker run --rm --interactive --volume "$NAME-certificates:/certificates" --entrypoint /bin/sh "$IMAGE" -c "rm /certificates/own.pem && cat > /certificates/semio.pem && chmod 600 /certificates/semio.pem && ls -ln /certificates" < "$SCRATCH/public.pem"
docker rm --force "$NAME" > /dev/null 2>&1
docker volume rm --force "$NAME-data" "$NAME-config" > /dev/null 2>&1
run "$PUBLIC" "$NAME-certificates" isolated
echo "log lines about ACME (none expected):"
logs "obtaining certificate|challenge|acme|letsencrypt"
echo "log lines about skipping:"
logs "skipping automatic"
if logs "obtaining certificate|acme" | grep -q .; then echo "ACME attempted: not publishing"; else
  run "$PUBLIC" "$NAME-certificates"
  echo "served: $(served "$PUBLIC")"
  echo "log lines about ACME (none expected):"
  logs "obtaining certificate|challenge|acme|letsencrypt"
fi

docker rm --force "$NAME" > /dev/null 2>&1
docker volume rm --force "$NAME-data" "$NAME-config" "$NAME-certificates" > /dev/null 2>&1
docker network rm "$NAME-isolated" > /dev/null 2>&1
echo "== done"
