#!/bin/bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖PostStart
set -e
WORKSPACE="${containerWorkspaceFolder:-/workspaces/semio}"
SSH_SIGNING_KEY="${HOME}/.ssh/id_ed25519_signing"
SSH_SIGNING_PUBLIC_KEY="${SSH_SIGNING_KEY}.pub"
SSH_AGENT_SOCKET="${HOME}/.ssh/semio-ssh-agent.sock"
SSH_AGENT_ENV="${HOME}/.ssh/semio-ssh-agent.env"
#region 🔖EmojiFonts
configure_emoji_fonts() {
  sudo mkdir -p /etc/fonts
  sudo tee /etc/fonts/local.conf >/dev/null <<'FONTCONFIG'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <!-- Add emoji font families to generic font families -->
  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>Noto Sans</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>serif</family>
    <prefer>
      <family>Noto Serif</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Noto Sans Mono</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  
  <!-- Ensure emoji font is found for emoji characters -->
  <match target="pattern">
    <test name="lang">
      <string>en</string>
    </test>
    <test name="family">
      <string>emoji</string>
    </test>
    <edit name="family" mode="prepend">
      <string>Noto Color Emoji</string>
    </edit>
  </match>
  
  <!-- Force emoji rendering for color emoji -->
  <match target="pattern">
    <test name="family">
      <string>Noto Color Emoji</string>
    </test>
    <edit name="fontformat" mode="assign">
      <string>TrueType</string>
    </edit>
    <edit name="scalable" mode="assign">
      <bool>true</bool>
    </edit>
  </match>
</fontconfig>
FONTCONFIG
  sudo fc-cache -f
  echo "✅ Emoji font fallback configured."
}
#endregion 🔖EmojiFonts
#region 🔖Startup
#region 🔖StashCleanup
# Drop all spurious git stash entries. Stashing is forbidden in concurrent editing workflows.
cd "$WORKSPACE"
stash_count=$(git stash list 2>/dev/null | wc -l)
if [ "$stash_count" -gt 0 ]; then
  git stash clear
  echo "✅ Cleared $stash_count spurious git stash entries."
fi
#endregion 🔖StashCleanup
#endregion 🔖Startup
#region 🔖Ownership
sudo chown -R vscode:vscode /home/vscode/.cache 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.claude 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codex 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.codeium 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.gitkraken 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.local/share/GitKrakenCLI 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.local/share/gk 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.config/F3D 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.cursor-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.antigravity-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.vscode-server 2>/dev/null || true
sudo chown -R vscode:vscode /home/vscode/.windsurf-server 2>/dev/null || true
echo "✅ Fixed ownership for persisted volume mounts."
#endregion 🔖Ownership
#region 🔖EmojiFonts
configure_emoji_fonts
#endregion 🔖EmojiFonts
#region 🔖Neo4jEnv
configure_neo4j_compose_env() {
  local profile_script="/etc/profile.d/99-semio-neo4j-mcp.sh"
  sudo tee "$profile_script" >/dev/null <<'NEO4JPROFILE'
export NEO4J_URI=bolt://localhost:7687
export NEO4J_USERNAME=neo4j
export NEO4J_PASSWORD=password
export NEO4J_TELEMETRY=false
NEO4JPROFILE
  sudo chmod 0644 "$profile_script" || true
  local marker="#region 🔌Neo4jMcp"
  local bashrc="${HOME}/.bashrc"
  if [ -f "$bashrc" ] && ! grep -Fq "$marker" "$bashrc" 2>/dev/null; then
    cat >>"$bashrc" <<'BASHRC'

#region 🔌Neo4jMcp
if [ -f /etc/profile.d/99-semio-neo4j-mcp.sh ]; then
  # shellcheck source=/dev/null
  . /etc/profile.d/99-semio-neo4j-mcp.sh
fi
#endregion 🔌Neo4jMcp
BASHRC
  fi
  if [ -f /etc/profile.d/99-semio-neo4j-mcp.sh ]; then
    # shellcheck source=/dev/null
    . /etc/profile.d/99-semio-neo4j-mcp.sh
  fi
  echo "✅ Neo4j MCP env (bolt://localhost:7687 in the semio devcontainer) installed for login shells and this session."
}

configure_neo4j_compose_env
#endregion 🔖Neo4jEnv
#region 🗄️Neo4jService
configure_neo4j_server() {
  if ! command -v neo4j >/dev/null 2>&1; then
    echo "⚠️ Neo4j is not installed in this devcontainer image."
    return 1
  fi

  local conf="/etc/neo4j/neo4j.conf"
  sudo mkdir -p /var/lib/neo4j/data /var/log/neo4j /var/run/neo4j
  sudo chown -R neo4j:neo4j /var/lib/neo4j /var/log/neo4j /var/run/neo4j

  set_neo4j_conf_value() {
    local key="$1"
    local value="$2"
    if sudo grep -Eq "^[#[:space:]]*${key}=" "$conf"; then
      sudo sed -i -E "s|^[#[:space:]]*${key}=.*|${key}=${value}|" "$conf"
    else
      printf '%s=%s\n' "$key" "$value" | sudo tee -a "$conf" >/dev/null
    fi
  }

  set_neo4j_conf_value "server.default_listen_address" "0.0.0.0"
  set_neo4j_conf_value "server.bolt.listen_address" ":7687"
  set_neo4j_conf_value "server.http.listen_address" ":7474"
  set_neo4j_conf_value "dbms.usage_report.enabled" "false"
  set_neo4j_conf_value "server.directories.import" "/workspaces/semio"
  set_neo4j_conf_value "dbms.security.procedures.allowlist" "apoc.*"
  set_neo4j_conf_value "dbms.security.procedures.unrestricted" "apoc.*"

  sudo tee /etc/neo4j/apoc.conf >/dev/null <<'APOCCONF'
apoc.export.file.enabled=true
apoc.import.file.enabled=true
apoc.import.file.use_neo4j_config=false
APOCCONF

  if [ ! -f /var/lib/neo4j/data/dbms/auth.ini ] && [ ! -f /var/lib/neo4j/data/dbms/auth ]; then
    sudo neo4j-admin dbms set-initial-password "${NEO4J_PASSWORD:-password}" >/dev/null
  fi
}

start_neo4j_server() {
  if command -v nc >/dev/null 2>&1 && nc -z localhost 7687 2>/dev/null; then
    echo "✅ Neo4j is already running at bolt://localhost:7687."
    return 0
  fi

  sudo neo4j start >/tmp/semio-neo4j-start.log 2>&1 || {
    echo "⚠️ Neo4j failed to start. Last startup log lines:"
    tail -40 /tmp/semio-neo4j-start.log || true
    return 1
  }
}

wait_for_neo4j_bolt() {
  for _ in $(seq 1 60); do
    if command -v nc >/dev/null 2>&1 && nc -z localhost 7687 2>/dev/null; then
      return 0
    fi
    if timeout 1 bash -c "echo >/dev/tcp/localhost/7687" 2>/dev/null; then
      return 0
    fi
    sleep 2
  done
  return 1
}

if configure_neo4j_server && start_neo4j_server && wait_for_neo4j_bolt; then
  echo "✅ Neo4j is running inside the semio devcontainer at bolt://localhost:7687."
else
  echo "⚠️ Neo4j was not reachable at bolt://localhost:7687 during post-start."
fi
#endregion 🗄️Neo4jService
#region 🧾Neo4jCypherPersistence
ensure_neo4j_schema_files() {
  local technologies=("semio" "elements" "coda" "reuse")
  for technology in "${technologies[@]}"; do
    local schema_dir="$WORKSPACE/$technology/schema/cypher"
    local schema_file="$schema_dir/schema.cypher"
    mkdir -p "$schema_dir"
    if [ ! -f "$schema_file" ]; then
      cat >"$schema_file" <<EOF
// SPDX-License-Identifier: AGPL-3.0-only
// Neo4j Cypher persistence for $technology.
// Keep this file replayable with cypher-shell or APOC.
EOF
    fi
  done
}

import_neo4j_schema_files_if_empty() {
  if ! command -v cypher-shell >/dev/null 2>&1; then
    return 0
  fi
  local node_count
  node_count="$(cypher-shell -a bolt://localhost:7687 -u "${NEO4J_USERNAME:-neo4j}" -p "${NEO4J_PASSWORD:-password}" --format plain 'MATCH (n) RETURN count(n) AS count;' 2>/dev/null | tail -n 1 | tr -d '[:space:]')" || return 0
  if [ "$node_count" != "0" ]; then
    echo "✅ Neo4j contains data; leaving schema/cypher imports untouched."
    return 0
  fi
  local apoc_procedure_count
  apoc_procedure_count="$(cypher-shell -a bolt://localhost:7687 -u "${NEO4J_USERNAME:-neo4j}" -p "${NEO4J_PASSWORD:-password}" --format plain "SHOW PROCEDURES YIELD name WHERE name IN ['apoc.cypher.runFile', 'apoc.export.cypher.query'] RETURN count(name) AS count;" 2>/dev/null | tail -n 1 | tr -d '[:space:]')" || return 0
  if [ "$apoc_procedure_count" != "2" ]; then
    echo "⚠️ Neo4j APOC Cypher persistence skipped because required APOC procedures are unavailable."
    return 0
  fi
  local imported=0
  for technology in semio elements coda reuse; do
    local schema_file="$WORKSPACE/$technology/schema/cypher/schema.cypher"
    if grep -Ev '^[[:space:]]*(//|:|$)' "$schema_file" >/dev/null 2>&1; then
      cypher-shell -a bolt://localhost:7687 -u "${NEO4J_USERNAME:-neo4j}" -p "${NEO4J_PASSWORD:-password}" "CALL apoc.cypher.runFile('file://$schema_file') YIELD row RETURN count(row) AS rows;" >/dev/null
      imported=$((imported + 1))
    fi
  done
  echo "✅ Neo4j APOC Cypher persistence ready ($imported schema files imported into empty DB)."
}

ensure_neo4j_schema_files
import_neo4j_schema_files_if_empty || echo "⚠️ Neo4j APOC Cypher import skipped."
#endregion 🧾Neo4jCypherPersistence
#region 🔖ClaudeAuth
CLAUDE_HOME="/home/vscode"
CLAUDE_DIR="${CLAUDE_HOME}/.claude"
CLAUDE_JSON="${CLAUDE_DIR}/.claude.json"
CLAUDE_JSON_BACKUP="${CLAUDE_DIR}/.claude.json.backup"
CLAUDE_JSON_LINK="${CLAUDE_HOME}/.claude.json"
CLAUDE_JSON_BACKUP_LINK="${CLAUDE_HOME}/.claude.json.backup"
mkdir -p "$CLAUDE_DIR"
if [ -f "$CLAUDE_JSON_LINK" ] && [ ! -L "$CLAUDE_JSON_LINK" ]; then
  if [ ! -f "$CLAUDE_JSON" ]; then
    mv "$CLAUDE_JSON_LINK" "$CLAUDE_JSON"
  else
    rm "$CLAUDE_JSON_LINK"
  fi
fi
if [ -f "$CLAUDE_JSON_BACKUP_LINK" ] && [ ! -L "$CLAUDE_JSON_BACKUP_LINK" ]; then
  if [ ! -f "$CLAUDE_JSON_BACKUP" ]; then
    mv "$CLAUDE_JSON_BACKUP_LINK" "$CLAUDE_JSON_BACKUP"
  else
    rm "$CLAUDE_JSON_BACKUP_LINK"
  fi
fi
if [ -f "$CLAUDE_JSON" ] && [ ! -e "$CLAUDE_JSON_LINK" ]; then
  ln -s "$CLAUDE_JSON" "$CLAUDE_JSON_LINK"
fi
if [ -f "$CLAUDE_JSON_BACKUP" ] && [ ! -e "$CLAUDE_JSON_BACKUP_LINK" ]; then
  ln -s "$CLAUDE_JSON_BACKUP" "$CLAUDE_JSON_BACKUP_LINK"
fi
echo "✅ Normalized Claude Code auth storage."
#endregion 🔖ClaudeAuth
#region 🔖GitOwnership
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    sudo chown -R vscode:vscode "$WORKSPACE/$path" 2>/dev/null || true
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
echo "✅ Fixed ownership for workspace + submodules."
#endregion 🔖GitOwnership
#region 🔖GitSafe
git config --global --add safe.directory "$WORKSPACE"
if [ -f "$WORKSPACE/.gitmodules" ]; then
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    git config --global --add safe.directory "$WORKSPACE/$path"
  done < <(git config -f "$WORKSPACE/.gitmodules" --get-regexp '^submodule\..*\.path$' | awk '{print $2}')
fi
echo "✅ Marked workspace + submodules as safe.directory for git."
#endregion 🔖GitSafe
#region 🔐GitSshSigning
ensure_shell_loads_ssh_agent() {
  local bashrc="${HOME}/.bashrc"
  local marker="#region 🔐SemioSshAgent"
  if [ -f "$bashrc" ] && grep -Fq "$marker" "$bashrc"; then
    return 0
  fi
  cat >>"$bashrc" <<'SHELLRC'

#region 🔐SemioSshAgent
if [ -f "$HOME/.ssh/semio-ssh-agent.env" ]; then
  . "$HOME/.ssh/semio-ssh-agent.env" >/dev/null 2>&1 || true
fi
#endregion 🔐SemioSshAgent
SHELLRC
}

start_ssh_signing_agent() {
  mkdir -p "$HOME/.ssh"
  chmod 700 "$HOME/.ssh"
  rm -f "$SSH_AGENT_SOCKET"
  eval "$(ssh-agent -a "$SSH_AGENT_SOCKET" -s)" >/dev/null
  {
    echo "export SSH_AUTH_SOCK=$SSH_AGENT_SOCKET"
    echo "export SSH_AGENT_PID=$SSH_AGENT_PID"
  } >"$SSH_AGENT_ENV"
  chmod 600 "$SSH_AGENT_ENV"
}

configure_git_ssh_signing() {
  if [ ! -f "$SSH_SIGNING_PUBLIC_KEY" ]; then
    echo "⚠️  SSH signing public key not found, skipping git SSH signing setup."
    return 0
  fi
  if [ -f "$SSH_SIGNING_KEY" ]; then
    chmod 600 "$SSH_SIGNING_KEY"
  fi
  chmod 644 "$SSH_SIGNING_PUBLIC_KEY"

  git config --global gpg.format ssh
  git config --global gpg.ssh.program ssh-keygen
  git config --global user.signingkey "$SSH_SIGNING_PUBLIC_KEY"
  git config --global commit.gpgsign true
  git config --global tag.gpgsign true

  if [ ! -S "$SSH_AGENT_SOCKET" ] || ! SSH_AUTH_SOCK="$SSH_AGENT_SOCKET" ssh-add -l >/dev/null 2>&1; then
    start_ssh_signing_agent
  fi
  ensure_shell_loads_ssh_agent
  echo "✅ Configured SSH commit signing agent."
  if ! SSH_AUTH_SOCK="$SSH_AGENT_SOCKET" ssh-add -l 2>/dev/null | grep -Fq "$(ssh-keygen -lf "$SSH_SIGNING_PUBLIC_KEY" | awk '{print $2}')"; then
    echo "⚠️  Unlock signing once per container session with: ssh-add $SSH_SIGNING_KEY"
  fi
}

configure_git_ssh_signing
#endregion 🔐GitSshSigning
#region 🔖PythonVenv
if [ -f "$WORKSPACE/.venv/bin/activate" ]; then
  source "$WORKSPACE/.venv/bin/activate"
  echo "✅ Activated Python virtual environment."
else
  echo "ℹ️ Python virtual environment is not present for this container OS yet."
fi
#endregion 🔖PythonVenv
echo "✅ Environment ready."
#endregion 🔖PostStart
