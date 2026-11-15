#!/bin/bash
# Restores the dotnet host program ($DOTNET_ROOT/dotnet) that a test of the native bootstrap overwrote with a fake script (package N3, 2026-11-14 12:14).
# Downloads two things: the pinned dotnet-install.sh (sha256-checked below) and the 10.0.8 runtime (about 30 MB) into a temporary directory; only the `dotnet` file is copied back.
set -eu
target="${DOTNET_ROOT:-$HOME/.dotnet}"
runtime="${1:-10.0.8}"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
curl -fsSL -o "$scratch/dotnet-install.sh" "https://raw.githubusercontent.com/dotnet/install-scripts/5147e32300a8e908f5d737c8cff63a76b4b63531/src/dotnet-install.sh"
echo "082f7685e156738a1b2e2ed8381a621870d4ce8e8c59278034556f05c186eb2e  $scratch/dotnet-install.sh" | shasum -a 256 -c -
bash "$scratch/dotnet-install.sh" --runtime dotnet --version "$runtime" --install-dir "$scratch/root" --no-path
file "$scratch/root/dotnet" | grep -q "Mach-O\|ELF" || { echo "the installed host is not a native program" >&2; exit 1; }
cp -p "$target/dotnet" "$target/dotnet.fake-n3.bak"
cp -p "$scratch/root/dotnet" "$target/dotnet"
rm -f "$target/payload"
"$target/dotnet" --list-sdks
