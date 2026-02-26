# Antigravity Devcontainer Template

This repository serves as a robust template for creating Antigravity-compatible Devcontainers, specifically designed to resolve common connectivity and installation issues found in recent versions (e.g., 1.18.x).

## The Problem

Users often encounter the following issues when trying to set up a Devcontainer with Antigravity:

1. **Configuration Errors**: `devcontainer.json` complaining about missing properties like `dockerComposeFile`.
2. **Stalled Containers**: The container starts but stalls during initialization due to incorrect volume mounting (mounting the parent directory instead of the project root), leading to massive indexing overhead.
3. **Server Installation Failures**: The IDE fails to connect because the remote server binary isn't installed automatically. This is often caused by:
* Missing dependencies (like `wget`) in the Docker image.
* Automatic installation scripts failing silently or attempting to download from incorrect URLs.



## The Solution

This template implements a multi-layered fix to ensure a smooth "Reopen in Container" experience:

### 1. Corrected Configuration

* **`devcontainer.json`**:
  * Explicitly links to the `docker-compose.yml` file.
  * Uses the `"postCreateCommand"` hook to safely execute the custom server installation script in the background *after* the container is created, without disrupting the IDE's connection lifecycle.
* **`docker-compose.yml`**:
  * **Volume Mount**: Corrects the mount to `.:/workspace` (current directory) to prevent indexing unrelated files.
  * **Keep-Alive**: Uses `command: sleep infinity` to prevent the default Debian image from immediately exiting.



### 2. Dependency Management

* **`Dockerfile`**: Includes `wget`, `iproute2`, `netbase`, and other essential utilities to ensure the server installation script can download the necessary binaries and establish networking tunnels.

### 3. Manual Server Installation

* **`server-install.sh`**: A robust script that:
* Detects if the Antigravity server is missing.
* Downloads the specific, compatible version of the server (e.g., 1.18.3) from a verified URL.
* Installs it to the correct location (`/root/.antigravity-server`).
* Ensures correct permissions.



---

## 🛠 Troubleshooting Error 127 & Startup Issues

If the container hangs at "Starting Remote Server" or throws **Error 127**, it typically means the Antigravity agent is missing a low-level system dependency required for its internal Node.js runtime or networking tunnel.

| Symptom | Cause | Solution |
| --- | --- | --- |
| **Error 127** during port forward | The forwarder cannot find the `ip` command. | Ensure `iproute2` is installed in the Dockerfile. |
| **Silent Crash** on terminal open | Node.js is missing atomic memory libraries. | Ensure `libatomic1` is installed (crucial for `slim` images). |
| **Connection Refused** | Missing network protocol definitions. | Ensure `netbase` is installed (provides `/etc/services`). |
| **Agent Won't Start** | A stale installation lock exists. | Run `rm -f /root/.antigravity-server/.installation_lock`. |

---

## How to Use

1. **Clone this repository** (or use it as a template).
2. **Open in Antigravity**.
3. **Reopen in Container**: The setup will automatically:
* Build the Docker image.
* Start the container.
* Run `server-install.sh` to setup the remote server.
* Connect the IDE successfully.
* **Pro-Tip**: You can verify the installation progress by clicking "show logs" in the Dev Container creation notification (bottom right of the IDE) to see the output of `server-install.sh` and the new download progress bar.


## 🤖 Applying to Existing Repositories

If you have an existing repository and want to inject this robust Antigravity Devcontainer setup without manually modifying your files, you can use the included AI Workflow skill:

1. In your target repository, download the skill file into the `.agent/workflows` directory:
   ```bash
   mkdir -p .agent/workflows
   curl -sL "https://raw.githubusercontent.com/hucaico/demo-devcontainer/main/.agent/workflows/inject-antigravity-devcontainer.md" -o .agent/workflows/inject-antigravity-devcontainer.md
   ```
2. Open the repository in your AI IDE and prompt the agent:
   > *"Please apply the `.agent/workflows/inject-antigravity-devcontainer.md` skill to this repository."*
   
The AI will automatically patch your `Dockerfile`, `docker-compose.yml`, and `devcontainer.json` to support Antigravity while preserving your existing project setup!


## Files of Interest

* `.devcontainer/devcontainer.json`: Main entry point configuration.
* `docker-compose.yml`: Orchestrates the container and mounts the install script.
* `Dockerfile`: Defines the OS and dependencies.
* `server-install.sh`: The magic script that fixes the server installation.

