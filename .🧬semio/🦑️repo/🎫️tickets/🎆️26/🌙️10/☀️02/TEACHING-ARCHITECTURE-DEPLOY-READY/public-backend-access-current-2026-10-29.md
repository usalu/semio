# Public Backend Access Revalidation

Current course deployment config still declares semio.iek.uni-hannover.de, proctor internal8791 and Caddy external80/443; README operator path describes university Docker host, compose services, backup/update/rollback and certificate handling. The owner runbook uses an SSH user placeholder rather than an actual account/connection. Host DNS/address and historical firewall statements identify the destination, not administrative access or present service cause. No live DNS/firewall inference was made.

Current architecture-course workflow builds/tests/publishes the proctor image to GHCR using packages:write/GITHUB_TOKEN, and separately deploys frontend Pages. It contains no university-host SSH/remote compose deployment step. ARCHITECTURE_COURSE_RUNNER selects a capable build runner, not a proven authenticated university service session. The normal docker-stack-bundle owner can prepare an operator artifact; that does not grant remote execution.

Rechecked enabled tool metadata for SSH/Docker/remote-host/service access. No callable authenticated connector to this university host or Docker daemon is available. Sites deployment tools apply to their own hosted Sites service, not this configured university proctor. No plugin installation, account/secret/key inspection, login, network mutation, remote write or Source edit occurred.

Conclusion: public backend restoration is not executable from the currently evidenced authority. Missing concrete capability is an authorized host connection/account or deployment runner bound to this university machine, with service-management permission and current compose/data/certificate location. If public ingress is blocked, actual network-operator access/routing permission is additionally needed; timeout alone cannot establish that condition. Do not infer absent credentials globally—the audit establishes no usable configured connection in inspected repo/tool surfaces. Latest public timeout and local runtime success remain separate evidence. No secret values were read or reported.

## Bounded Availability Refresh 2026-10-29T20:50:11.705Z

public frontend: HTTP 200 in716ms. public proctor: TimeoutError in10001ms. No authentication or mutation. Local maintenance and public readiness remain separate scopes.
