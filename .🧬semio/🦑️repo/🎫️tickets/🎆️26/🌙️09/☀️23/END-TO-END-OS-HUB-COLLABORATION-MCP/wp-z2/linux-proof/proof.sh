#!/usr/bin/env bash
# Z2 Linux proof (runs inside a plain distro container, repo copy at /src, home at /root):
# the repo's own native bootstrap provisions the toolchain, then bun install, nx graph, cargo check, and the
# linker comparison on the os-hub binary. Every phase prints `[z2-phase] <name> <start|end> <epoch> status=<n>`.
set -u
cd /src
BOOT="/src/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh"
CRATE="${Z2_CRATE:-semio-hub}"
BIN="${Z2_BIN:-os-hub}"
PHASES="${Z2_PHASES:-toolchain install graph check build link}"
phase() {
  local name="$1" start status
  shift
  start="$(date +%s)"
  echo "[z2-phase] $name start $start"
  "$@"
  status=$?
  echo "[z2-phase] $name end $(date +%s) status=$status elapsed=$(($(date +%s) - start))s"
  return 0
}
toolchain() { SKIP_NEO4J_DESKTOP=1 SKIP_REPO_BOOTSTRAP=1 bash "$BOOT" setup; }
native() {
  SKIP_NEO4J_DESKTOP=1 bash "$BOOT" setup >/root/z2-native.log 2>&1
  local status=$?
  grep -E '^\[compose\]|NX   |Successfully ran|Failed tasks|^- |error(:|\[)|Cannot find' /root/z2-native.log | grep -v '^- Downloading' | head -150
  echo "[z2-native] tail"
  tail -40 /root/z2-native.log
  return $status
}
tools_env() {
  export PATH="/root/.bun/bin:/root/.cargo/bin:/root/.local/bin:$PATH" DOTNET_ROOT=/root/.dotnet
  export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 NX_DAEMON=false CARGO_TERM_COLOR=never
}
versions() {
  for tool in "bun --version" "rustc -vV" "cargo -V" "go version" "dotnet --version" "uv --version" "cmake --version" "ninja --version" "cc --version" "git --version"; do
    echo "[z2-version] $tool :: $($tool 2>&1 | head -2 | tr '\n' ' ')"
  done
}
install() { bun install --frozen-lockfile; }
clone() {
  export DEBIAN_FRONTEND=noninteractive
  apt-get update -qq >/dev/null && apt-get install -y -qq --no-install-recommends git ca-certificates >/dev/null && gitinit
}
gitinit() {
  [ -d /src/.git ] && return 0
  git init -q /src && git -C /src add -A && git -C /src -c user.name=z2 -c user.email=z2@example.invalid commit -qm "z2 clean checkout" && git -C /src rev-parse --short HEAD
}
prepare() { bun nx run workspace:prepare --output-style=stream; }
setup() { bun nx run workspace:setup --output-style=stream; }
graph() {
  bun nx graph --file=/tmp/z2-nx-graph.json &&
    bun -e 'const g=JSON.parse(await Bun.file("/tmp/z2-nx-graph.json").text()).graph; console.log("[z2-graph] projects", Object.keys(g.nodes).length, "dependencies", Object.values(g.dependencies).flat().length)'
}
lock() {
  cp /src/Cargo.lock /tmp/Cargo.lock.before && cargo metadata --format-version 1 >/dev/null && diff /tmp/Cargo.lock.before /src/Cargo.lock | grep -E '^[<>] name = ' | sed 's/^/[z2-lock] /'
  return 0
}
check() { cargo check -p "$CRATE"; }
mcp() { cd "/src/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && bun ./📜️script.ts build 2>&1 | grep -vE '^\s*(Compiling|Checking|Downloaded)|^warning|^\s*\||-->|^\s*=|help:|^[0-9]+ [-+]|^\s*$' | tail -60; local status=${PIPESTATUS[0]}; cd /src; return "$status"; }
stracesetup() {
  apt-get install -y -qq --no-install-recommends strace >/dev/null </dev/null
  strace -f -qq -e trace=kill,tgkill,tkill,execve -e signal=none -o /root/z2-strace.log bun nx run workspace:setup --output-style=stream >/root/z2-setup.log 2>&1
  local status=$?
  grep -E 'SIGINT|SIGTERM|SIGKILL' /root/z2-strace.log | grep -v '= -1 ESRCH' | head -40
  for pid in $(grep -E 'SIGINT' /root/z2-strace.log | awk '{print $1}' | sort -u | head -5); do echo "[z2-strace] sender $pid :: $(grep -E "^$pid " /root/z2-strace.log | head -2 | tr '\n' ' ')"; done
  return "$status"
}
setupall() { bun nx run workspace:setup --output-style=stream >/root/z2-setup.log 2>&1; local status=$?; grep -E "Failed tasks|^- |Successfully ran|✖" /root/z2-setup.log | tail -20; return "$status"; }
build() { cargo build -p "$CRATE" --bin "$BIN"; }
link() {
  command -v mold >/dev/null 2>&1 || apt-get install -y -qq --no-install-recommends mold >/dev/null 2>&1 </dev/null
  cargo rustc -p "$CRATE" --bin "$BIN" -- --print link-args -C save-temps --cfg "z2_linkargs_$(date +%s)" >/tmp/z2-linkargs.txt 2>&1
  local command linker run started seconds status exe
  command="$(grep -E '"-fuse-ld=lld"' /tmp/z2-linkargs.txt | tail -1 | perl -CS -pe 's/\\u\{([0-9a-fA-F]+)\}/chr(hex($1))/ge')"
  if [ -z "$command" ]; then
    echo "[z2-link] no link command captured"
    tail -20 /tmp/z2-linkargs.txt
    return 1
  fi
  echo "[z2-link] command bytes=${#command} mold=$(mold --version | head -1) ld=$(ld.bfd --version | head -1)"
  for run in 1 2 3; do
    for linker in lld mold bfd; do
      started="$(date +%s%N)"
      (cd /src && eval "${command//\"-fuse-ld=lld\"/\"-fuse-ld=$linker\"}") >/tmp/z2-link-$linker.log 2>&1
      status=$?
      seconds="$(awk -v ns="$(($(date +%s%N) - started))" 'BEGIN { printf "%.2f", ns / 1e9 }')"
      echo "[z2-link] $linker run=$run status=$status seconds=$seconds"
      [ "$status" -eq 0 ] || head -c 600 "/tmp/z2-link-$linker.log"
    done
  done
  exe="$(printf '%s' "$command" | grep -o '"-o" "[^"]*"' | sed 's/^"-o" "//; s/"$//')"
  echo "[z2-link] output $exe $(du -h "$exe" | cut -f1) :: $(readelf -p .comment "$exe" 2>/dev/null | grep -Eo 'LLD [0-9.]+|mold [0-9.]+|GNU ld[^]]*' | sort -u | tr '\n' ' ')"
}
echo "[z2-proof] $(date -u +%FT%TZ) $(uname -srm) $(. /etc/os-release && echo "$PRETTY_NAME") cpus=$(nproc)"
for name in $PHASES; do
  case "$name" in
  toolchain) phase toolchain toolchain; tools_env; versions ;;
  sysdeps) phase sysdeps bash -c 'apt-get update -qq >/dev/null </dev/null && apt-get install -y -qq --no-install-recommends build-essential ca-certificates curl git unzip xz-utils libicu-dev procps mold binutils >/dev/null </dev/null'; tools_env ;;
  native) phase native native; tools_env; versions ;;
  *) tools_env; phase "$name" "$name" ;;
  esac
done
echo "[z2-proof] done $(date -u +%FT%TZ)"
