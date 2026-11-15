FROM oven/bun:1.3.14
RUN apt-get update && apt-get install -y --no-install-recommends git python3 ca-certificates procps && rm -rf /var/lib/apt/lists/*
RUN mkdir /opt/nm && cd /opt/nm && bun add ajv@8.20.0 && ln -s /opt/nm/node_modules /node_modules
