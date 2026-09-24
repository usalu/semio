#!/usr/bin/env bash
# Z2: native arm64 PowerShell 7 (release tarball) runs the lint oracle over /in.
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null && apt-get install -y -qq --no-install-recommends curl ca-certificates libicu74 tar >/dev/null
tag="$(curl -fsSI https://github.com/PowerShell/PowerShell/releases/latest | sed -n 's#^location: .*/tag/\(v[0-9.]*\).*#\1#Ip' | tr -d '\r')"
curl -fsSL -o /tmp/pwsh.tgz "https://github.com/PowerShell/PowerShell/releases/download/$tag/powershell-${tag#v}-linux-arm64.tar.gz"
mkdir -p /opt/pwsh && tar -xzf /tmp/pwsh.tgz -C /opt/pwsh && chmod +x /opt/pwsh/pwsh
echo "[z2-ps] PowerShell $tag"
/opt/pwsh/pwsh -NoProfile -File /work/lint.ps1
