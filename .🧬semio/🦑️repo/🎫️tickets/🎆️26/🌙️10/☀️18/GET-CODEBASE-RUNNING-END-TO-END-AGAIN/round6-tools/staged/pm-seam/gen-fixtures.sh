#!/bin/sh
# usage: sh gen-fixtures.sh   regenerates real lock files with the real tools, offline, from tiny local projects (members and path dependencies only), into /private/tmp/pm-seam/gen/out; mkfixtures.py folds them in
# one small process at a time, nice 10; every step is skipped when its tool is missing
set -u
OUT=/private/tmp/pm-seam/gen/out
W=/private/tmp/pm-seam/gen/work
rm -rf "$W" "$OUT"; mkdir -p "$W" "$OUT"
has() { command -v "$1" >/dev/null 2>&1; }
if has bun; then
  mkdir -p "$W/bun/packages/member" && cd "$W/bun" || exit 1
  echo '{"name":"root","version":"1.0.0","workspaces":["packages/*"],"dependencies":{"member":"workspace:*"}}' > package.json
  echo '{"name":"member","version":"0.0.0"}' > packages/member/package.json
  nice -n 10 bun install --lockfile-only >/dev/null 2>&1 && cp bun.lock "$OUT/bun.lock"
  bun --version > "$OUT/bun.version"
fi
if has npm; then
  mkdir -p "$W/npm/packages/member" && cd "$W/npm" || exit 1
  echo '{"name":"root","version":"1.0.0","private":true,"workspaces":["packages/*"],"dependencies":{"member":"*"}}' > package.json
  echo '{"name":"member","version":"0.0.0"}' > packages/member/package.json
  nice -n 10 npm install --package-lock-only --offline --ignore-scripts --no-audit --no-fund >/dev/null 2>&1 && cp package-lock.json "$OUT/package-lock.json"
  npm --version > "$OUT/npm.version"
fi
if has cargo; then
  mkdir -p "$W/cargo/member/src" "$W/cargo/root/src" && cd "$W/cargo/root" || exit 1
  printf '[package]\nname = "root"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nmember = { path = "../member" }\n' > Cargo.toml
  printf '[package]\nname = "member"\nversion = "0.0.0"\nedition = "2021"\n' > ../member/Cargo.toml
  echo 'fn main(){}' > src/main.rs; echo '' > ../member/src/lib.rs
  nice -n 10 cargo generate-lockfile --offline >/dev/null 2>&1 && cp Cargo.lock "$OUT/Cargo.lock"
  cargo --version > "$OUT/cargo.version"
fi
if has uv; then
  mkdir -p "$W/uv/member" && cd "$W/uv" || exit 1
  printf '[project]\nname = "root"\nversion = "0.1.0"\nrequires-python = ">=3.9"\ndependencies = ["member"]\n[tool.uv.sources]\nmember = { path = "member" }\n[tool.uv]\npackage = false\n' > pyproject.toml
  printf '[project]\nname = "member"\nversion = "0.0.0"\nrequires-python = ">=3.9"\n[build-system]\nrequires = ["setuptools"]\nbuild-backend = "setuptools.build_meta"\n' > member/pyproject.toml
  nice -n 10 uv lock --offline >/dev/null 2>&1 && cp uv.lock "$OUT/uv.lock"
  uv --version > "$OUT/uv.version"
fi
if has go; then
  mkdir -p "$W/go" && cd "$W/go" || exit 1
  printf 'module example.com/m\n\ngo 1.21\n\nrequire github.com/bmatcuk/doublestar/v4 v4.6.1\n' > go.mod
  printf 'package main\n\nimport _ "github.com/bmatcuk/doublestar/v4"\n\nfunc main() {}\n' > main.go
  GOFLAGS=-mod=mod GOPROXY=off nice -n 10 go mod tidy >/dev/null 2>&1 && cp go.sum "$OUT/go.sum"
  go version > "$OUT/go.version"
fi
if has dotnet; then
  mkdir -p "$W/dotnet/Lib" "$W/dotnet/App" && cd "$W/dotnet/App" || exit 1
  printf '<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net10.0</TargetFramework><RestorePackagesWithLockFile>true</RestorePackagesWithLockFile></PropertyGroup><ItemGroup><ProjectReference Include="../Lib/Lib.csproj" /></ItemGroup></Project>' > App.csproj
  printf '<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net10.0</TargetFramework><RestorePackagesWithLockFile>true</RestorePackagesWithLockFile></PropertyGroup></Project>' > ../Lib/Lib.csproj
  DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 nice -n 10 dotnet restore --use-lock-file --source /nonexistent >/dev/null 2>&1; cp packages.lock.json "$OUT/packages.lock.json" 2>/dev/null
  dotnet --version > "$OUT/dotnet.version"
fi
ls -la "$OUT"
