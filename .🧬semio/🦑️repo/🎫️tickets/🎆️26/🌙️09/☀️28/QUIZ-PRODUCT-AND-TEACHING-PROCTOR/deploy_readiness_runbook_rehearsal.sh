#!/bin/sh
# 📖️ Deploy-readiness rehearsal of the runbook's host commands, typed exactly as the README gives them, in a fresh
# directory against the local Docker engine. Three things differ from a real host and are set where an operator sets
# overrides — in `.env`: the proctor host is `localhost` (Caddy's internal certificate authority instead of a public
# certificate), the ports are 18080/18443, and COMPOSE_PROJECT_NAME keeps the rehearsal apart from a real stack. The
# image is the locally built one, so `docker pull` / `docker compose pull` (the registry) are the only lines skipped.
#   sh deploy_readiness_runbook_rehearsal.sh
set -eu
say() { printf '\n[DEBUG] ===== %s =====\n' "$*"; }
home="$(mktemp -d)"
cd "$home"

say "bootstrap in $home"
mkdir -p architecture-quiz && cd architecture-quiz
container=$(docker create ghcr.io/usalu/architecture-quiz-proctor:latest) && docker cp "$container:/srv/quiz/deploy/." . && docker rm --volumes "$container"
ls -la
printf 'PROCTOR_HOST=localhost\nQUIZ_HTTP_PORT=18080\nQUIZ_HTTPS_PORT=18443\nCOMPOSE_PROJECT_NAME=architecture-quiz-rehearsal\n' > .env
docker compose up --detach --wait

say "verify"
docker compose ps
curl --fail --silent --insecure https://localhost:18443/instance | cut -c1-80
curl --silent --output /dev/null --write-out '%{http_code} %{redirect_url}\n' http://localhost:18080/instance
curl --silent --insecure --include --request OPTIONS --header 'Origin: https://quizzes.architektur-und-technologie.de' --header 'Access-Control-Request-Method: POST' https://localhost:18443/queries | tr -d '\r' | grep -i -E '^HTTP|access-control-allow-origin'

say "backup while serving"
docker compose exec -T proctor proctor backup - > "proctor-$(date +%Y-%m-%dT%H%M%S).sqlite"
ls -la proctor-*.sqlite
head -c 15 proctor-*.sqlite; echo

say "the daily cron line, run once"
mkdir -p backups && docker compose exec -T proctor proctor backup - > "backups/proctor-$(date +%F).sqlite" && find backups -name 'proctor-*.sqlite' -mtime +30 -delete
ls -la backups

say "update"
docker compose exec -T proctor proctor backup - > "proctor-before-update-$(date +%Y-%m-%dT%H%M%S).sqlite"
docker compose images proctor
container=$(docker create ghcr.io/usalu/architecture-quiz-proctor:latest) && docker cp "$container:/srv/quiz/deploy/." . && docker rm --volumes "$container"
docker compose up --detach --wait

say "rollback to an immutable tag"
revision=$(docker image inspect --format '{{index .Config.Labels "org.opencontainers.image.revision"}}' ghcr.io/usalu/architecture-quiz-proctor:latest | cut -c1-12)
docker tag ghcr.io/usalu/architecture-quiz-proctor:latest "ghcr.io/usalu/architecture-quiz-proctor:sha-$revision"
PROCTOR_TAG="sha-$revision" docker compose up --detach --wait
docker compose images proctor

say "restore"
backup=$(ls proctor-2*.sqlite | head -1)
docker compose stop proctor
docker compose run --rm -T --no-deps proctor restore - < "$backup"
docker compose up --detach --wait
curl --fail --silent --insecure https://localhost:18443/instance | cut -c1-80

say "rebuild the read models"
docker compose stop proctor
docker compose run --rm --no-deps proctor rebuild
docker compose up --detach --wait

say "logs"
docker compose logs --since 1h proctor caddy | tail -6 | cut -c1-200
docker compose logs caddy | grep -i -E 'obtain|acme|challenge|error' | tail -3 | cut -c1-200 || true

say "down keeps the volumes"
docker compose down
docker volume ls --format '{{.Name}}' | grep architecture-quiz-rehearsal

say "clean up the rehearsal (the one command a host never runs)"
docker compose down --volumes
docker image rm "ghcr.io/usalu/architecture-quiz-proctor:sha-$revision" > /dev/null
cd / && rm -rf "$home"
echo "[DEBUG] rehearsal complete"
