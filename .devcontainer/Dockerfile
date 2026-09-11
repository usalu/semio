FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG BUN_VERSION=1.3.14
ARG NODE_VERSION=24.15.0

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    uuid-dev \
    curl \
    wget \
    fontconfig \
    fonts-noto-color-emoji \
    fonts-noto-cjk \
    fonts-noto-mono \
    fonts-noto \
    ripgrep \
    cloc \
    netcat-openbsd \
    iproute2 \
    net-tools \
    lsof \
    psmisc \
    procps \
    openssh-client \
    vim \
    dnsutils \
    iputils-ping \
    file \
    xz-utils \
    unzip \
    zip \
    jq \
    sqlite3 \
    libsqlite3-dev \
    graphviz \
    xvfb \
    libnss3 \
    libnspr4 \
    libatk1.0-0 \
    libatk-bridge2.0-0 \
    libcups2 \
    libdrm2 \
    libxkbcommon0 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxrandr2 \
    libgbm1 \
    libasound2t64 \
    libpango-1.0-0 \
    libcairo2 \
    libatspi2.0-0 \
    fonts-liberation \
    libappindicator3-1 \
    libu2f-udev \
    libvulkan1 \
    xdg-utils \
    ca-certificates \
    gnupg \
    lsb-release \
    libatomic1 \
    mold \
    openjdk-21-jre-headless \
    netbase \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN set -eu; \
    runtime_arch="$(dpkg --print-architecture)"; \
    case "$runtime_arch" in \
      amd64) bun_archive="x64-baseline"; bun_sha="a063908ae08b7852ca10939bbdc6ceed3ddabce8fb9402dce83d65d73b36e6c7"; node_archive="x64"; node_sha="472655581fb851559730c48763e0c9d3bc25975c59d518003fc0849d3e4ba0f6";; \
      arm64) bun_archive="aarch64"; bun_sha="a27ffb63a8310375836e0d6f668ae17fa8d8d18b88c37c821c65331973a19a3b"; node_archive="arm64"; node_sha="f3d5a797b5d210ce8e2cb265544c8e482eaedcb8aa409a8b46da7e8595d0dda0";; \
      *) echo "Unsupported runtime architecture: $runtime_arch" >&2; exit 1;; \
    esac; \
    mkdir -p /tmp/semio-runtime; \
    cd /tmp/semio-runtime; \
    curl --fail --location --retry 3 --max-time 300 "https://github.com/oven-sh/bun/releases/download/bun-v$BUN_VERSION/bun-linux-$bun_archive.zip" --output bun.zip; \
    printf '%s  bun.zip\n' "$bun_sha" | sha256sum -c -; \
    unzip -p bun.zip "bun-linux-$bun_archive/bun" > /usr/local/bin/bun; \
    chmod 755 /usr/local/bin/bun; \
    ln -s bun /usr/local/bin/bunx; \
    test "$(bun --version)" = "$BUN_VERSION"; \
    curl --fail --location --retry 3 --max-time 300 "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$node_archive.tar.xz" --output node.tar.xz; \
    printf '%s  node.tar.xz\n' "$node_sha" | sha256sum -c -; \
    tar -xJf node.tar.xz --strip-components=2 --no-same-owner -C /usr/local/bin "node-v$NODE_VERSION-linux-$node_archive/bin/node"; \
    test "$(node --version)" = "v$NODE_VERSION"; \
    cd /; \
    rm -rf /tmp/semio-runtime

RUN mkdir -p /etc/apt/keyrings /var/lib/neo4j/plugins \
    && wget -O - https://debian.neo4j.com/neotechnology.gpg.key | gpg --dearmor -o /etc/apt/keyrings/neotechnology.gpg \
    && echo 'deb [signed-by=/etc/apt/keyrings/neotechnology.gpg] https://debian.neo4j.com stable 5' > /etc/apt/sources.list.d/neo4j.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends neo4j \
    && wget -O /var/lib/neo4j/plugins/apoc-core-5.26.4-core.jar https://repo.maven.apache.org/maven2/org/neo4j/procedure/apoc-core/5.26.4/apoc-core-5.26.4-core.jar \
    && wget -O /var/lib/neo4j/plugins/apoc-5.26.4-extended.jar https://github.com/neo4j-contrib/neo4j-apoc-procedures/releases/download/5.26.4/apoc-5.26.4-extended.jar \
    && chown -R neo4j:neo4j /var/lib/neo4j/plugins \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl -LsSf https://astral.sh/uv/install.sh | UV_INSTALL_DIR=/usr/local/bin sh

USER vscode

ENV BUN_INSTALL=/home/vscode/.bun
ENV PATH=/home/vscode/.bun/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV NX_WORKSPACE_DATA_DIRECTORY=/workspaces/semio/.nx/workspace-data
ENV NEO4J_URI=bolt://localhost:7687
ENV NEO4J_USERNAME=neo4j
ENV NEO4J_PASSWORD=password
ENV NEO4J_TELEMETRY=false

RUN mkdir -p /home/vscode/.local/bin \
    /home/vscode/.gitkraken \
    /home/vscode/.local/share/GitKrakenCLI \
    /home/vscode/.local/share/gk \
    /home/vscode/.claude \
    /home/vscode/.codex \
    /home/vscode/.config/gh \
    /home/vscode/.config/cursor \
    /home/vscode/.config/antigravity \
    /home/vscode/.config/openai \
    /home/vscode/.codeium \
    /home/vscode/.cursor-server \
    /home/vscode/.antigravity-server \
    /home/vscode/.vscode-server \
    /home/vscode/.windsurf-server

WORKDIR /workspaces/semio
