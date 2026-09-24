"""Z2 one-off codemod: native Unix bootstrap provisioning (toolchain region, no global git config, frozen install)."""
import sys
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh"
s=open(B,encoding='utf-8').read()
def rep(a,b,n=1):
    global s
    c=s.count(a)
    if c!=n: sys.exit(f"count {c}!={n}: {a[:80]!r}")
    s=s.replace(a,b)
rep("""# Zero-touch macOS/Linux bootstrap: Neo4j Desktop, uv, Neo4j env vars for MCP, bun install, workspace:setup.
# 🧭️Invoke: `bash ./⌨️script.sh setup` (full bootstrap) or `bash ./⌨️script.sh start` (IDE session / Neo4j-only path).
# Native setup is rooted here and does not depend on devcontainer scripts.""",
"""# Zero-touch macOS/Linux bootstrap: the native C/C++ toolchain (apt-get, dnf, pacman, zypper or Homebrew), Bun, rustup
# with the `rust-toolchain.toml` pin, Go, .NET, uv, optional Neo4j Desktop, then `bun install` and `workspace:setup`.
# 🧭️Invoke from a fresh clone: `bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup` (full
# bootstrap) or `… start` (IDE session / Neo4j-only path); `bun ./📜️script.ts setup native` routes here once Bun exists.
# Every step probes before it installs and never writes user-global tool configuration.
# Native setup is rooted here and does not depend on devcontainer scripts.""")
rep("""  printf '%s\\n' "usage: bash ./⌨️script.sh <setup|start>" >&2""",
"""  printf '%s\\n' "usage: bash 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh <setup|start>" >&2""")
rep("""SKIP_REPO_BOOTSTRAP="${SKIP_REPO_BOOTSTRAP:-0}"
""","""SKIP_REPO_BOOTSTRAP="${SKIP_REPO_BOOTSTRAP:-0}"
DOTNET_CHANNEL="${DOTNET_CHANNEL:-8.0}"
LOCAL_BIN="${HOME}/.local/bin"
""")
a=s.index("java_major_version() {"); b=s.index("ensure_native_neo4j_tools() {")
s=s[:a]+s[b:]
a=s.index("#region 🔖️EnsureCpp"); b=s.index("#endregion 🔖️EnsureCpp")+len("#endregion 🔖️EnsureCpp\n")
s=s[:a]+open(sys.argv[1],encoding='utf-8').read()+s[b:]
a=s.index("#region 🔖️GitSafe"); b=s.index("#endregion 🔖️GitSafe")+len("#endregion 🔖️GitSafe\n\n")
s=s[:a]+s[b:]
rep("""  mkdir -p "$NX_WORKSPACE_DATA_DIRECTORY"
  configure_git_safe_directories
  ensure_bun
  log "bun install…"
  bun install
  if command -v go >/dev/null 2>&1; then
    log "Building repo client binary…"
    bun nx run @semio-tech/repo-client:build || log "repo client build failed."
  fi
  log "workspace:setup…"
  bun nx run workspace:setup""",
"""  mkdir -p "$NX_WORKSPACE_DATA_DIRECTORY"
  log "bun install…"
  bun install --frozen-lockfile
  log "Building repo client binary…"
  bun nx run @semio-tech/repo-client:build || log "repo client build failed; workspace:setup reports the Go toolchain state."
  log "workspace:setup…"
  bun nx run workspace:setup""")
rep("""install_neo4j_desktop
ensure_uv
ensure_cpp_toolchain
ensure_native_neo4j
repo_bootstrap""",
"""ensure_native_toolchain
ensure_bun
ensure_rust_toolchain || log "Rust toolchain provisioning failed; install rustup from https://rustup.rs and rerun setup."
ensure_go || log "Go provisioning failed; install Go 1.21+ from https://go.dev/dl and rerun setup."
ensure_dotnet || log ".NET SDK ${DOTNET_CHANNEL} provisioning failed; install it from https://dot.net and rerun setup."
ensure_uv || log "uv provisioning failed; install it from https://docs.astral.sh/uv and rerun setup."
install_neo4j_desktop
ensure_native_neo4j
repo_bootstrap""")
open(B,'w',encoding='utf-8').write(s)
print("ok")
