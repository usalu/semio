FROM semio-linux-bun
ARG PWSH=7.7.0
RUN apt-get update && apt-get install -y --no-install-recommends curl libssl3t64 less && rm -rf /var/lib/apt/lists/* \
 && mkdir -p /opt/microsoft/powershell && curl -fsSL https://github.com/PowerShell/PowerShell/releases/download/v${PWSH}/powershell-${PWSH}-linux-arm64.tar.gz | tar -xz -C /opt/microsoft/powershell \
 && chmod +x /opt/microsoft/powershell/pwsh && ln -s /opt/microsoft/powershell/pwsh /usr/local/bin/pwsh
ENV DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1 POWERSHELL_TELEMETRY_OPTOUT=1
