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
#region 🔖Neo4jComposeEnv
configure_neo4j_compose_env() {
  local profile_script="/etc/profile.d/99-semio-neo4j-mcp.sh"
  sudo tee "$profile_script" >/dev/null <<'NEO4JPROFILE'
export NEO4J_URI=bolt://neo4j:7687
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
  echo "✅ Neo4j MCP env (bolt://neo4j:7687 compose service) installed for login shells and this session."
}

configure_neo4j_compose_env
#endregion 🔖Neo4jComposeEnv
#region 🔖Neo4jReady
wait_for_neo4j_bolt() {
  for _ in $(seq 1 60); do
    if command -v nc >/dev/null 2>&1 && nc -z neo4j 7687 2>/dev/null; then
      return 0
    fi
    if timeout 1 bash -c "echo >/dev/tcp/neo4j/7687" 2>/dev/null; then
      return 0
    fi
    sleep 2
  done
  return 1
}

if wait_for_neo4j_bolt; then
  echo "✅ Neo4j is reachable at bolt://neo4j:7687 from the devcontainer."
else
  echo "⚠️ Neo4j was not reachable at bolt://neo4j:7687 during post-start."
fi
#endregion 🔖Neo4jReady
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
if [ -d "$WORKSPACE/.venv" ]; then
  source "$WORKSPACE/.venv/bin/activate"
fi
echo "✅ Activated Python virtual environment."
#endregion 🔖PythonVenv
echo "✅ Environment ready."
#endregion 🔖PostStart
