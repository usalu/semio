---
description: Inject the Antigravity Devcontainer Pattern
---

This workflow empowers the AI agent to adapt an existing repository to support the Antigravity IDE connection setup, resolving Error 127, connection timeouts, and silent Node.js crashes.

**Context:** The agent must safely patch the 4 core components (`Dockerfile`, `docker-compose.yml`, `devcontainer.json`, and `server-install.sh`) without destroying the project's existing dependencies or architecture.

### Step 1: Download `server-install.sh`
Retrieve the stable server installation script from the primary template repository.
```bash
// turbo
curl -sL "https://raw.githubusercontent.com/hucaico/demo-devcontainer/main/server-install.sh" -o server-install.sh
chmod +x server-install.sh
```

### Step 2: Patch `Dockerfile`
Ensure the Dockerfile installs the core system dependencies required by the Antigravity server: `iproute2`, `libatomic1`, `netbase`, and `wget`.
- **Constraint:** Use code editing tools to cautiously append these to the *existing* package manager installation block (e.g., `apt-get install`, `apk add`). 
- **Constraint:** Do NOT overwrite the user's base image or existing project dependencies.

### Step 3: Patch `docker-compose.yml`
Modify the `docker-compose.yml` to support the devcontainer lifecycle and script injection.
- **Constraint:** Ensure the volume mount for the install script exists under the primary app service:
  `- ./server-install.sh:/usr/local/bin/server-install.sh`
- **Constraint:** Ensure the `command:` is set to `sleep infinity` (or an equivalent keep-alive command) if the base image defaults to a short-lived process like `/bin/bash` or `sh`. Do NOT override the command if the container already runs a permanent blocking process (like a web server).

### Step 4: Patch `devcontainer.json`
Update `.devcontainer/devcontainer.json` to properly trigger the background installation hook and point the remote IDE to the cached server location.
- **Constraint:** Add or update the `"postCreateCommand"` to execute the installation script: `"chmod +x /usr/local/bin/server-install.sh && /usr/local/bin/server-install.sh"`
- **Constraint:** Add the setting `"vscode": { "remoteEnv": { "VSCODE_AGENT_FOLDER": "/workspace/.antigravity-server-cache" } }` inside the `"customizations"` object so the IDE knows where to find the persistent cached server.
- **Constraint:** Ensure the obsolete `"overrideCommand": false` directive is NOT present, as it will block the IDE connection lifecycle indefinitely.
