# Backend Remote Access Refresh — 2026-10-29

Read-only refresh while retaining local server41024. The parent independently reports public frontend HTTPS200 and public backend HTTPS timeout10s at13:09UTC; this report did not repeat that network request or infer its cause. No remote authentication or mutation occurred.

## Current Available Access

Canonical deploy JSON still declares semio.iek.uni-hannover.de, image ghcr.io/usalu/architecture-course-proctor and internal port8791. It has no remote account or authenticated deployment transport. README331 uses literal <user> for SCP; README612 uses literal <user> for SSH backup. No concrete authorized university login is established.

Fresh repo MCP tools_list succeeded: Docker and gh are registered but missing; no SSH/SCP/Ansible tool registration is present. Repo remote_execution_status and remote_cache_status both report configured:false, absent server/token; these are build facilities and do not declare deployment access in any event. Only presence was inspected: ~/.ssh exists, but config and known_hosts do not; SSH_AUTH_SOCK, GITHUB_TOKEN, GH_TOKEN and SEMIO_TOOL_SSH are absent. No private keys, credential contents, config secrets or private environment values were read or printed. Presence/absence is not authentication proof.

The GitHub workflow still requires ARCHITECTURE_COURSE_RUNNER and uses its Actions-issued GITHUB_TOKEN to publish an already checked image. It does not provision or update the university host. The live GitHub runner variable and registry account/package state cannot be authenticated through the currently missing tool/token. Registry publication alone does not start that host.

## Supported Next Action

An existing authorized host operator should enter ~/architecture-course and inspect docker compose ps plus docker compose logs --since 1h proctor caddy (README545/679). If services are healthy, check proctor:8791 inside the stack and Caddy public ingress; if unhealthy, use those actual logs/image selection as the causal repair basis. TCP timeout alone cannot select firewall, DNS, listener or certificate repair.

After the current complete release/browser/image validation, the normal course deploy-check, docker-image-build/check, docker-stack-check and docker-stack-bundle targets produce a reviewable deployment candidate. The supported registry-free handoff copies the verified bundle to ~/architecture-course, loads proctor-image.tar and runs docker compose up --detach --wait. An existing installation must first take the documented consistent backup and retain its .env, certificates and learner volumes; select the exact produced PROCTOR_TAG. Do not run the fresh-bundle producer directly into an existing configured live stack directory.

At present there is no declared authenticated repo-supported route here to execute even the first remote host inspection. The concrete external prerequisite is an authorized account/transport supplied through a supported repository tool or the existing host operator. No guessed login, raw SSH, ad-hoc installer, deployment, message to another person, credential request or permission prompt was used. The local runtime is available independently; service41024 remains retained.

Source references: course/🚀️deploy/🔣️.json; course/README.md331,342,545,612,679; .github/workflows/architecture-course.yml52,186–188. Earlier deployment-readiness-2026-10-28.md remains historical; this refresh establishes current access presence only, not a current complete production deployment acceptance.
