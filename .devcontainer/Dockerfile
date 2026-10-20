FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG BUN_VERSION=1.4.2
ARG NODE_VERSION=24.15.0

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    lldb \
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
    libjpeg-turbo-progs \
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
    locales \
    netbase \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN sed -i 's/^# *\(en_US.UTF-8 UTF-8\)/\1/' /etc/locale.gen \
    && locale-gen en_US.UTF-8 \
    && update-locale LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 \
    && locale -a | grep -qi '^en_US\.utf-\?8$'

ENV LANG=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8

RUN set -eu; \
    runtime_arch="$(dpkg --print-architecture)"; \
    case "$runtime_arch" in \
      amd64) bun_archive="x64-baseline"; bun_sha="c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f"; node_archive="x64"; node_sha="472655581fb851559730c48763e0c9d3bc25975c59d518003fc0849d3e4ba0f6";; \
      arm64) bun_archive="aarch64"; bun_sha="54328bbc2d9c8e0c9f892c544d66c57a83b84139e34909e5ee81758f1ac8fda7"; node_archive="arm64"; node_sha="f3d5a797b5d210ce8e2cb265544c8e482eaedcb8aa409a8b46da7e8595d0dda0";; \
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

USER vscode

ENV BUN_INSTALL=/home/vscode/.bun
ENV PATH=/home/vscode/.local/bin:/home/vscode/.bun/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV SEMIO_WORKSPACE_ROOT=/workspaces/semio

RUN mkdir -p /home/vscode/.local/bin \
    /home/vscode/.claude \
    /home/vscode/.codex \
    /home/vscode/.config/gh \
    /home/vscode/.config/openai

WORKDIR /workspaces/semio
