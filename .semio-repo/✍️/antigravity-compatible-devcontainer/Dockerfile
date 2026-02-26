FROM debian:bookworm

# Only the bare minimum system utilities that any remote environment expects
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    procps \
    git \
    wget \
    iproute2 \
    libatomic1 \
    netbase \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
