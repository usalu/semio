# 🗒️ Deploy notes for the agents "domain" and "edge"

Written by the deploy-readiness agent, 2026-10-02 00:10. The proctor Rust files are yours; I do not touch them again.

## Already in the working tree (written before the verbs were assigned to "domain")

All green at 00:05: `RUSTC_WRAPPER="" cargo test -p teaching-proctor` → 48 unit, 14 conformance, 6 end-to-end.

| Where | What |
|---|---|
| `🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs` | `proctor ready` (GET `/instance` with `X-Forwarded-Proto: https` on `ProctorConfig::probe_address`, exit 0 only on `200`; the image `HEALTHCHECK`), `proctor backup <file\|->` (`VACUUM INTO` through a read-only handle, safe while serving; `-` streams to stdout through a spool file in `PROCTOR_DATA`), `proctor restore <file\|->` (spools into `PROCTOR_DATA`, checks the format row and `PRAGMA integrity_check`, refuses while another process holds the database, replaces the file and drops stale `-wal`/`-shm`) |
| `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs` | `Database::snapshot`, `Database::inspect`, `Database::adopt`, private `vacant` (exclusive-locking-mode probe) |
| `🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🦀️.rs` | `ProctorConfig::data_directory`, `ProctorConfig::probe_address`, helpers `setting` / `bind` / `port` (used by `from_environment`) |
| tests | storage unit (snapshot, adopt, refusals), cli unit (backup → restore round trip, `ready` against a fake listener), e2e `the_operator_probes_backs_up_while_serving_and_restores_elsewhere` (runs the real binary) |

Rename (`ready` → `health`) or reshape (`backup <dir>`) as the coordinator's naming requires. I read the final syntax from the
proctor README before I finish and adapt the Dockerfile, compose and the runbook.

## What the deployment needs from the verbs

The new image is distroless (`gcr.io/distroless/cc-debian12:nonroot`): no shell, no `cp`, no `curl`, read-only root filesystem,
uid 65532, one writable path (`/srv/quiz/data`, the volume).

1. **Health**: an exec-form verb without arguments that needs only the image's baked environment
   (`HEALTHCHECK CMD ["/usr/local/bin/proctor", "<verb>"]`), exit 0 only when `GET /instance` answers `200` as the proxy asks.
2. **Backup**: callable while serving through `docker compose exec -T proctor proctor backup …`. The operator needs the file on
   the host: either stdout (`backup -`, what exists) or a file inside `/srv/quiz/data` that `docker compose cp` can fetch. Never
   overwrite an existing backup silently.
3. **Restore**: a verb that runs as the image user in a one-off container (`docker compose run --rm -T proctor restore -`),
   because `docker cp` into the volume creates root-owned files the proctor (uid 65532) cannot open. It must refuse while a
   proctor serves the directory.

## For "edge"

- Tell me the final body limit (bytes) and every new `PROCTOR_*` variable with its production default: the Caddyfile
  `request_body { max_size … }`, the compose `environment:` block and the image check assert them.
- The image check sends `X-Forwarded-Proto: https` and `X-Forwarded-Host` on a direct loopback port. If rate limits key on
  `X-Forwarded-For`, the check's ~20 requests in a few seconds from one address must stay under the production defaults, or
  the check needs to know the limit to stay below it.
