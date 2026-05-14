FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    wget \
    fontconfig \
    fonts-noto-color-emoji \
    fonts-noto-cjk \
    fonts-noto-mono \
    fonts-noto \
    ripgrep \
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
    openjdk-21-jre-headless \
    netbase \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /etc/apt/keyrings \
    && wget -O - https://debian.neo4j.com/neotechnology.gpg.key | gpg --dearmor -o /etc/apt/keyrings/neotechnology.gpg \
    && echo 'deb [signed-by=/etc/apt/keyrings/neotechnology.gpg] https://debian.neo4j.com stable 5' > /etc/apt/sources.list.d/neo4j.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends neo4j \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl -LsSf https://astral.sh/uv/install.sh | UV_INSTALL_DIR=/usr/local/bin sh

USER vscode

ENV BUN_INSTALL=/home/vscode/.bun
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH=/home/vscode/.bun/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV NX_WORKSPACE_DATA_DIRECTORY=/workspaces/semio/.nx/workspace-data-terminal
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
    /home/vscode/.windsurf-server \
    /home/vscode/.cache/ms-playwright

WORKDIR /workspaces/semio
